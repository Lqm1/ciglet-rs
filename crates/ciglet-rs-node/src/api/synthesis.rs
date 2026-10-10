use crate::synthesis::{self, Sinusoid};
use napi::Error as BindingError;
/// Complete native sinusoid configuration, including arbitrary field bits.
pub struct SinusoidF32 {
    value: Sinusoid<f32>,
}
impl SinusoidF32 {
    pub fn new(frequency_hz: f32, amplitude: f32, phase_radians: f32) -> Self {
        Self {
            value: Sinusoid {
                frequency_hz,
                amplitude,
                phase_radians,
            },
        }
    }
    pub fn from_values(values: &[f32]) -> Result<Self, BindingError> {
        let [frequency, amplitude, phase]: [f32; 3] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("sinusoid requires exactly three values"))?;
        Ok(Self::new(frequency, amplitude, phase))
    }
    pub fn frequency_hz(&self) -> f32 {
        self.value.frequency_hz
    }
    pub fn set_frequency_hz(&mut self, value: f32) {
        self.value.frequency_hz = value;
    }
    pub fn amplitude(&self) -> f32 {
        self.value.amplitude
    }
    pub fn set_amplitude(&mut self, value: f32) {
        self.value.amplitude = value;
    }
    pub fn phase_radians(&self) -> f32 {
        self.value.phase_radians
    }
    pub fn set_phase_radians(&mut self, value: f32) {
        self.value.phase_radians = value;
    }
    pub fn values(&self) -> Vec<f32> {
        vec![
            self.value.frequency_hz,
            self.value.amplitude,
            self.value.phase_radians,
        ]
    }
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        self.value = Self::from_values(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
}
pub fn sinusoid_f32(
    tone: &SinusoidF32,
    sample_rate: u32,
    length: usize,
) -> Result<Vec<f32>, BindingError> {
    synthesis::sinusoid(tone.value, sample_rate, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn sinusoids_f32(
    frequencies: &[f32],
    amplitudes: &[f32],
    phases: &[f32],
    sample_rate: u32,
    length: usize,
) -> Result<Vec<f32>, BindingError> {
    if frequencies.len() != amplitudes.len() || frequencies.len() != phases.len() {
        return Err(napi::Error::from_reason(
            "sinusoid arrays must have matching lengths",
        ));
    }
    let mut tones = Vec::new();
    tones
        .try_reserve_exact(frequencies.len())
        .map_err(|_| napi::Error::from_reason("sinusoid configuration allocation failed"))?;
    tones.extend(frequencies.iter().zip(amplitudes).zip(phases).map(
        |((&frequency_hz, &amplitude), &phase_radians)| Sinusoid {
            frequency_hz,
            amplitude,
            phase_radians,
        },
    ));
    synthesis::sinusoids(&tones, sample_rate, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
/// Complete native sinusoid configuration, including arbitrary field bits.
pub struct SinusoidF64 {
    value: Sinusoid<f64>,
}
impl SinusoidF64 {
    pub fn new(frequency_hz: f64, amplitude: f64, phase_radians: f64) -> Self {
        Self {
            value: Sinusoid {
                frequency_hz,
                amplitude,
                phase_radians,
            },
        }
    }
    pub fn from_values(values: &[f64]) -> Result<Self, BindingError> {
        let [frequency, amplitude, phase]: [f64; 3] = values
            .try_into()
            .map_err(|_| napi::Error::from_reason("sinusoid requires exactly three values"))?;
        Ok(Self::new(frequency, amplitude, phase))
    }
    pub fn frequency_hz(&self) -> f64 {
        self.value.frequency_hz
    }
    pub fn set_frequency_hz(&mut self, value: f64) {
        self.value.frequency_hz = value;
    }
    pub fn amplitude(&self) -> f64 {
        self.value.amplitude
    }
    pub fn set_amplitude(&mut self, value: f64) {
        self.value.amplitude = value;
    }
    pub fn phase_radians(&self) -> f64 {
        self.value.phase_radians
    }
    pub fn set_phase_radians(&mut self, value: f64) {
        self.value.phase_radians = value;
    }
    pub fn values(&self) -> Vec<f64> {
        vec![
            self.value.frequency_hz,
            self.value.amplitude,
            self.value.phase_radians,
        ]
    }
    pub fn set_values(&mut self, values: &[f64]) -> Result<(), BindingError> {
        self.value = Self::from_values(values)?.value;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        Self { value: self.value }
    }
}
pub fn sinusoid_f64(
    tone: &SinusoidF64,
    sample_rate: u32,
    length: usize,
) -> Result<Vec<f64>, BindingError> {
    synthesis::sinusoid(tone.value, sample_rate, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
pub fn sinusoids_f64(
    frequencies: &[f64],
    amplitudes: &[f64],
    phases: &[f64],
    sample_rate: u32,
    length: usize,
) -> Result<Vec<f64>, BindingError> {
    if frequencies.len() != amplitudes.len() || frequencies.len() != phases.len() {
        return Err(napi::Error::from_reason(
            "sinusoid arrays must have matching lengths",
        ));
    }
    let mut tones = Vec::new();
    tones
        .try_reserve_exact(frequencies.len())
        .map_err(|_| napi::Error::from_reason("sinusoid configuration allocation failed"))?;
    tones.extend(frequencies.iter().zip(amplitudes).zip(phases).map(
        |((&frequency_hz, &amplitude), &phase_radians)| Sinusoid {
            frequency_hz,
            amplitude,
            phase_radians,
        },
    ));
    synthesis::sinusoids(&tones, sample_rate, length)
        .map_err(|error| napi::Error::from_reason(error.0))
}
