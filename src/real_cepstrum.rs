//! Real cepstra, minimum-phase angles and frame-major spectrum conversion.
pub use crate::phase::NyquistPolicy;
use crate::{
    Error, approximation,
    fourier::{Complex, Direction, FftPlan},
    statistics::Scalar,
    vector,
};

/// One-sided cepstral lifter used to compute minimum-phase angles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifter {
    /// Double positive quefrencies, retain DC/Nyquist, clear negative ones.
    Causal,
    /// Retain upstream's doubled Nyquist and one uncleared negative sample.
    Legacy,
}

/// Forward/inverse FFT plans and one complex workspace, reused across frames.
#[derive(Debug)]
pub struct Plan<T: Scalar> {
    fft: FftPlan<T>,
    workspace: Vec<Complex<T>>,
}
impl<T: Scalar> Plan<T> {
    pub fn new(length: usize) -> Result<Self, Error> {
        let fft = FftPlan::new(length)?;
        let mut workspace = vector::buffer(length)?;
        workspace.resize(length, Complex::new(T::zero(), T::zero()));
        Ok(Self { fft, workspace })
    }
    pub fn len(&self) -> usize {
        self.fft.len()
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn bins(&self) -> usize {
        self.len() / 2 + 1
    }
    fn used_count(&self, count: usize, policy: NyquistPolicy) -> usize {
        count.min(if policy == NyquistPolicy::Legacy && self.len() > 1 {
            self.len() / 2
        } else {
            self.bins()
        })
    }
    fn mirror(&mut self, policy: NyquistPolicy) {
        let n = self.len();
        if policy == NyquistPolicy::Legacy && n > 1 {
            self.workspace[n / 2].re = self.workspace[n / 2 - 1].re;
        }
        for i in n / 2 + 1..n {
            self.workspace[i].re = self.workspace[n - i].re;
        }
    }
    fn prepare(&mut self, values: &[T], policy: NyquistPolicy) -> Result<(), Error> {
        let count = self.used_count(values.len(), policy);
        vector::validate(&values[..count])?;
        self.workspace.fill(Complex::new(T::zero(), T::zero()));
        for (value, &input) in self.workspace.iter_mut().zip(&values[..count]) {
            value.re = input;
        }
        self.mirror(policy);
        Ok(())
    }
    fn transform(&mut self, direction: Direction) -> Result<(), Error> {
        self.fft.process(&mut self.workspace, direction)?;
        if self
            .workspace
            .iter()
            .any(|value| !value.re.into().is_finite() || !value.im.into().is_finite())
        {
            return Err(Error("cepstral transform overflowed"));
        }
        Ok(())
    }
    fn output(&self, imaginary: bool) -> Result<Vec<T>, Error> {
        let mut output = vector::buffer(self.len())?;
        output.extend(self.workspace.iter().map(
            |value| {
                if imaginary { value.im } else { value.re }
            },
        ));
        Ok(output)
    }
    /// Invert a one-sided log spectrum. No logarithm is applied here.
    pub fn analyze(&mut self, logs: &[T], policy: NyquistPolicy) -> Result<Vec<T>, Error> {
        if logs.len() != self.bins() {
            return Err(Error("log spectrum length differs from the cepstral plan"));
        }
        self.prepare(logs, policy)?;
        self.transform(Direction::Inverse)?;
        self.output(false)
    }
    /// Mirror a cepstral prefix and transform it into a full-grid log spectrum.
    /// Empty coefficients mean zero; the source's mirrored suffix is ignored.
    pub fn reconstruct(
        &mut self,
        coefficients: &[T],
        policy: NyquistPolicy,
    ) -> Result<Vec<T>, Error> {
        if coefficients.len() > self.len() {
            return Err(Error("cepstral coefficient count exceeds the transform"));
        }
        self.prepare(coefficients, policy)?;
        self.transform(Direction::Forward)?;
        self.output(false)
    }
    /// Return full-grid minimum-phase angles, without an additional log or exp.
    pub fn minimum_phase_with(
        &mut self,
        logs: &[T],
        policy: NyquistPolicy,
        lifter: Lifter,
    ) -> Result<Vec<T>, Error> {
        if logs.len() != self.bins() {
            return Err(Error("log spectrum length differs from the cepstral plan"));
        }
        self.prepare(logs, policy)?;
        self.transform(Direction::Inverse)?;
        for value in &mut self.workspace {
            value.im = T::zero();
        }
        let n = self.len();
        let end = n / 2 + usize::from(lifter == Lifter::Legacy);
        let two = T::from_f64(2.0).unwrap();
        for value in self.workspace.iter_mut().take(end).skip(1) {
            value.re = value.re * two;
        }
        let start = n / 2 + if lifter == Lifter::Legacy { 2 } else { 1 };
        for value in self.workspace.iter_mut().skip(start) {
            value.re = T::zero();
        }
        self.transform(Direction::Forward)?;
        self.output(true)
    }
    /// Apply source level-two log(magnitude + 1e-15) and retain each prefix.
    pub fn spectrogram_to_cepstra(
        &mut self,
        magnitudes: &[T],
        count: usize,
        policy: NyquistPolicy,
    ) -> Result<Vec<T>, Error> {
        let bins = self.bins();
        if count > self.len() || !magnitudes.len().is_multiple_of(bins) {
            return Err(Error("spectrogram or cepstral output shape is invalid"));
        }
        let length = (magnitudes.len() / bins)
            .checked_mul(count)
            .ok_or(Error("cepstral matrix size overflow"))?;
        let mut output = vector::buffer(length)?;
        for frame in magnitudes.chunks_exact(bins) {
            let used = self.used_count(bins, policy);
            vector::validate(&frame[..used])?;
            self.workspace.fill(Complex::new(T::zero(), T::zero()));
            for (value, &magnitude) in self.workspace.iter_mut().zip(&frame[..used]) {
                value.re =
                    T::from_f32(approximation::logarithm((magnitude.into() + 1e-15) as f32)?)
                        .unwrap();
            }
            self.mirror(policy);
            self.transform(Direction::Inverse)?;
            output.extend(self.workspace[..count].iter().map(|value| value.re));
        }
        Ok(output)
    }
    /// Reconstruct frame-major one-sided magnitudes with source level-two exp.
    /// Explicit frame count permits zero coefficients and empty frame matrices.
    pub fn cepstra_to_spectrogram(
        &mut self,
        coefficients: &[T],
        frames: usize,
        count: usize,
        policy: NyquistPolicy,
    ) -> Result<Vec<T>, Error> {
        if count > self.len() || frames.checked_mul(count) != Some(coefficients.len()) {
            return Err(Error("cepstral input matrix shape is invalid"));
        }
        let length = frames
            .checked_mul(self.bins())
            .ok_or(Error("spectrum matrix size overflow"))?;
        let mut output = vector::buffer(length)?;
        for frame in 0..frames {
            self.prepare(&coefficients[frame * count..(frame + 1) * count], policy)?;
            self.transform(Direction::Forward)?;
            for value in &self.workspace[..self.bins()] {
                output.push(
                    T::from_f32(approximation::exponential(value.re.into() as f32)?).unwrap(),
                );
            }
        }
        vector::validate(&output)?;
        Ok(output)
    }
}

fn spectrum_length(bins: usize) -> Result<usize, Error> {
    bins.checked_sub(1)
        .filter(|&count| count > 0)
        .and_then(|count| count.checked_mul(2))
        .ok_or(Error("one-sided log spectrum requires at least two bins"))
}
/// Real cepstrum retaining supplied Nyquist data. Input is already logarithmic.
pub fn from_log_spectrum<T: Scalar>(logs: &[T]) -> Result<Vec<T>, Error> {
    Plan::new(spectrum_length(logs.len())?)?.analyze(logs, NyquistPolicy::Preserve)
}
pub fn to_log_spectrum<T: Scalar>(coefficients: &[T], length: usize) -> Result<Vec<T>, Error> {
    Plan::new(length)?.reconstruct(coefficients, NyquistPolicy::Preserve)
}
/// Minimum-phase angles using a causal lifter and retained Nyquist data.
pub fn minimum_phase<T: Scalar>(logs: &[T]) -> Result<Vec<T>, Error> {
    Plan::new(spectrum_length(logs.len())?)?.minimum_phase_with(
        logs,
        NyquistPolicy::Preserve,
        Lifter::Causal,
    )
}
