use crate::callbacks::Function;
use crate::random::{self, Policy, Sampler};
use pyo3::PyErr as BindingError;
use rand::{Rng, SeedableRng, TryRng};
use rand_chacha::ChaCha8Rng;
use std::{cell::RefCell, convert::Infallible, rc::Rc};
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "ciglet_rs")]
pub enum RandomPolicy {
    Stable = 0,
    Legacy = 1,
}
fn policy(code: u32) -> Result<Policy, BindingError> {
    match code {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid Gaussian arithmetic policy",
        )),
    }
}
type Failure = Rc<RefCell<Option<BindingError>>>;
#[derive(Clone)]
enum Source {
    ChaCha(Box<ChaCha8Rng>),
    Callback(Function),
}
struct WordRng {
    source: Source,
    failure: Failure,
}
impl Clone for WordRng {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            failure: Rc::new(RefCell::new(None)),
        }
    }
}
impl WordRng {
    fn word(&mut self) -> u32 {
        match &mut self.source {
            Source::ChaCha(rng) => rng.next_u32(),
            Source::Callback(callback) => {
                if self.failure.borrow().is_some() {
                    return u32::MAX;
                }
                let value = callback.call0(&()).and_then(|value| {
                    value
                        .as_f64()
                        .filter(|v| {
                            v.is_finite() && v.fract() == 0.0 && (0.0..=u32::MAX as f64).contains(v)
                        })
                        .map(|v| v as u32)
                        .ok_or_else(|| {
                            pyo3::exceptions::PyValueError::new_err(
                                "random callback must return a u32 integer",
                            )
                        })
                });
                match value {
                    Ok(word) => word,
                    Err(error) => {
                        *self.failure.borrow_mut() = Some(error);
                        u32::MAX
                    }
                }
            }
        }
    }
}
impl TryRng for WordRng {
    type Error = Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.word())
    }
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        match &mut self.source {
            Source::ChaCha(rng) => Ok(rng.next_u64()),
            Source::Callback(_) => Ok(self.word() as u64 | ((self.word() as u64) << 32)),
        }
    }
    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Infallible> {
        match &mut self.source {
            Source::ChaCha(rng) => rng.fill_bytes(bytes),
            Source::Callback(_) => {
                for chunk in bytes.chunks_mut(4) {
                    chunk.copy_from_slice(&self.word().to_le_bytes()[..chunk.len()]);
                }
            }
        }
        Ok(())
    }
}
fn finish<T>(failure: &Failure, result: Result<T, crate::Error>) -> Result<T, BindingError> {
    if let Some(error) = failure.borrow_mut().take() {
        return Err(error);
    }
    result.map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
/// Caller-owned words; callbacks retain their external JavaScript state.
pub struct RandomWordSource {
    rng: RefCell<WordRng>,
}
impl RandomWordSource {
    fn with_rng<T>(
        &self,
        action: impl FnOnce(&mut WordRng) -> Result<T, BindingError>,
    ) -> Result<T, BindingError> {
        let mut rng = self.rng.try_borrow_mut().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("random source is already in use")
        })?;
        let result = action(&mut rng);
        if let Some(error) = rng.failure.borrow_mut().take() {
            return Err(error);
        }
        result
    }
    fn with_chacha<T>(&self, action: impl FnOnce(&mut ChaCha8Rng) -> T) -> Result<T, BindingError> {
        self.with_rng(|rng| match &mut rng.source {
            Source::ChaCha(rng) => Ok(action(rng)),
            Source::Callback(_) => Err(pyo3::exceptions::PyValueError::new_err(
                "random source is not ChaCha8",
            )),
        })
    }
}
impl RandomWordSource {
    pub fn from_seed(seed: &[u8]) -> Result<Self, BindingError> {
        let seed: [u8; 32] = seed.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("ChaCha8 seed must contain 32 bytes")
        })?;
        Ok(Self {
            rng: RefCell::new(WordRng {
                source: Source::ChaCha(Box::new(ChaCha8Rng::from_seed(seed))),
                failure: Rc::new(RefCell::new(None)),
            }),
        })
    }
    pub fn new(callback: Function) -> Self {
        Self {
            rng: RefCell::new(WordRng {
                source: Source::Callback(callback),
                failure: Rc::new(RefCell::new(None)),
            }),
        }
    }
    pub fn callback(&self) -> Result<Option<Function>, BindingError> {
        self.with_rng(|rng| {
            Ok(match &rng.source {
                Source::Callback(callback) => Some(callback.clone()),
                Source::ChaCha(_) => None,
            })
        })
    }
    pub fn next_u32(&self) -> Result<u32, BindingError> {
        self.with_rng(|rng| Ok(rng.next_u32()))
    }
    pub fn next_u64(&self) -> Result<u64, BindingError> {
        self.with_rng(|rng| Ok(rng.next_u64()))
    }
    pub fn fill_bytes(&self, count: usize) -> Result<Vec<u8>, BindingError> {
        if count > i32::MAX as usize {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "random byte budget exceeded",
            ));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(count).map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("random byte allocation failed")
        })?;
        bytes.resize(count, 0);
        self.with_rng(|rng| {
            rng.fill_bytes(&mut bytes);
            Ok(bytes)
        })
    }
    pub fn seed(&self) -> Result<Vec<u8>, BindingError> {
        self.with_chacha(|rng| rng.get_seed().to_vec())
    }
    pub fn stream(&self) -> Result<u64, BindingError> {
        self.with_chacha(|rng| rng.get_stream())
    }
    pub fn set_stream(&self, stream: u64) -> Result<(), BindingError> {
        self.with_chacha(|rng| rng.set_stream(stream))
    }
    /// All 128 input bits are retained; ChaCha itself defines its period wrap.
    pub fn word_position(&self) -> Result<Vec<u8>, BindingError> {
        self.with_chacha(|rng| rng.get_word_pos().to_le_bytes().to_vec())
    }
    pub fn set_word_position(&self, position: &[u8]) -> Result<(), BindingError> {
        let bytes: [u8; 16] = position.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("word position must contain 16 bytes")
        })?;
        self.with_chacha(|rng| rng.set_word_pos(u128::from_le_bytes(bytes)))
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        self.with_rng(|rng| {
            Ok(Self {
                rng: RefCell::new(rng.clone()),
            })
        })
    }
}
pub struct RandomSampler {
    sampler: RefCell<Sampler<WordRng>>,
    failure: Failure,
}
impl RandomSampler {
    fn with_sampler<T>(
        &self,
        action: impl FnOnce(&mut Sampler<WordRng>) -> Result<T, crate::Error>,
    ) -> Result<T, BindingError> {
        let mut sampler = self.sampler.try_borrow_mut().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("random sampler is already in use")
        })?;
        finish(&self.failure, action(&mut sampler))
    }
}
impl RandomSampler {
    pub fn new(source: RandomWordSource, maximum: u32) -> Result<Self, BindingError> {
        let rng = source.rng.into_inner();
        let failure = Rc::clone(&rng.failure);
        Sampler::new(rng, maximum)
            .map(|sampler| Self {
                sampler: RefCell::new(sampler),
                failure,
            })
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
    }
    pub fn from_seed(seed: &[u8]) -> Result<Self, BindingError> {
        Self::new(RandomWordSource::from_seed(seed)?, i32::MAX as u32)
    }
    pub fn maximum(&self) -> Result<u32, BindingError> {
        self.with_sampler(|sampler| Ok(sampler.maximum()))
    }
    pub fn draw(&self) -> Result<u32, BindingError> {
        self.with_sampler(|sampler| Ok(sampler.draw()))
    }
    pub fn into_rng(self) -> RandomWordSource {
        RandomWordSource {
            rng: RefCell::new(self.sampler.into_inner().into_rng()),
        }
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        let sampler = self.sampler.try_borrow().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("random sampler is already in use")
        })?;
        let maximum = sampler.maximum();
        let rng = sampler.clone().into_rng();
        let failure = Rc::clone(&rng.failure);
        Ok(Self {
            sampler: RefCell::new(
                Sampler::new(rng, maximum)
                    .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))?,
            ),
            failure,
        })
    }
    pub fn uniform_f32(&self) -> Result<f32, BindingError> {
        self.with_sampler(|sampler| sampler.uniform())
    }
    pub fn uniform_f64(&self) -> Result<f64, BindingError> {
        self.with_sampler(|sampler| sampler.uniform())
    }
    pub fn normal_f32(&self, mean: f32, variance: f32) -> Result<f32, BindingError> {
        self.with_sampler(|sampler| sampler.normal(mean, variance))
    }
    pub fn normal_f64(&self, mean: f64, variance: f64) -> Result<f64, BindingError> {
        self.with_sampler(|sampler| sampler.normal(mean, variance))
    }
    pub fn normal_with_f32(
        &self,
        mean: f32,
        variance: f32,
        code: u32,
    ) -> Result<f32, BindingError> {
        let policy = policy(code)?;
        self.with_sampler(|sampler| sampler.normal_with(mean, variance, policy))
    }
    pub fn normal_with_f64(
        &self,
        mean: f64,
        variance: f64,
        code: u32,
    ) -> Result<f64, BindingError> {
        let policy = policy(code)?;
        self.with_sampler(|sampler| sampler.normal_with(mean, variance, policy))
    }
    pub fn white_noise_f32(&self, amplitude: f32, count: usize) -> Result<Vec<f32>, BindingError> {
        self.with_sampler(|sampler| sampler.white_noise(amplitude, count))
    }
    pub fn white_noise_f64(&self, amplitude: f64, count: usize) -> Result<Vec<f64>, BindingError> {
        self.with_sampler(|sampler| sampler.white_noise(amplitude, count))
    }
}
pub fn random_uniform_from_draw_f32(draw: u32, maximum: u32) -> Result<f32, BindingError> {
    random::uniform_from_draw(draw, maximum)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn random_white_from_draw_f32(
    amplitude: f32,
    draw: u32,
    maximum: u32,
) -> Result<f32, BindingError> {
    random::white_from_draw(amplitude, draw, maximum)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn random_normal_from_draws_f32(
    mean: f32,
    variance: f32,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> Result<f32, BindingError> {
    random::normal_from_draws(mean, variance, [first, second], maximum, policy(code)?)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn random_uniform_from_draw_f64(draw: u32, maximum: u32) -> Result<f64, BindingError> {
    random::uniform_from_draw(draw, maximum)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn random_white_from_draw_f64(
    amplitude: f64,
    draw: u32,
    maximum: u32,
) -> Result<f64, BindingError> {
    random::white_from_draw(amplitude, draw, maximum)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
pub fn random_normal_from_draws_f64(
    mean: f64,
    variance: f64,
    first: u32,
    second: u32,
    maximum: u32,
    code: u32,
) -> Result<f64, BindingError> {
    random::normal_from_draws(mean, variance, [first, second], maximum, policy(code)?)
        .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.0))
}
