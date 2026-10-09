use crate::random::{self, Policy, Sampler};
use js_sys::Function;
use rand::{Rng, SeedableRng, TryRng};
use rand_chacha::ChaCha8Rng;
use std::{cell::RefCell, convert::Infallible, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum RandomPolicy {
    Stable = 0,
    Legacy = 1,
}
fn policy(code: u32) -> Result<Policy, JsValue> {
    match code {
        0 => Ok(Policy::Stable),
        1 => Ok(Policy::Legacy),
        _ => Err(JsValue::from_str("invalid Gaussian arithmetic policy")),
    }
}
type Failure = Rc<RefCell<Option<JsValue>>>;

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
                // MAX always passes inclusive range rejection for the supported grids.
                // Discard the operation's result after returning normally from Rust.
                if self.failure.borrow().is_some() {
                    return u32::MAX;
                }
                let value = callback.call0(&JsValue::UNDEFINED).and_then(|value| {
                    value
                        .as_f64()
                        .filter(|v| {
                            v.is_finite() && v.fract() == 0.0 && (0.0..=u32::MAX as f64).contains(v)
                        })
                        .map(|v| v as u32)
                        .ok_or_else(|| {
                            JsValue::from_str("random callback must return a u32 integer")
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
fn finish<T>(failure: &Failure, result: Result<T, crate::Error>) -> Result<T, JsValue> {
    if let Some(error) = failure.borrow_mut().take() {
        return Err(error);
    }
    result.map_err(|error| JsValue::from_str(error.0))
}

/// Caller-owned words; callbacks retain their external JavaScript state.
#[wasm_bindgen]
pub struct RandomWordSource {
    rng: RefCell<WordRng>,
}
impl RandomWordSource {
    fn with_rng<T>(
        &self,
        action: impl FnOnce(&mut WordRng) -> Result<T, JsValue>,
    ) -> Result<T, JsValue> {
        let mut rng = self
            .rng
            .try_borrow_mut()
            .map_err(|_| JsValue::from_str("random source is already in use"))?;
        let result = action(&mut rng);
        if let Some(error) = rng.failure.borrow_mut().take() {
            return Err(error);
        }
        result
    }
    fn with_chacha<T>(&self, action: impl FnOnce(&mut ChaCha8Rng) -> T) -> Result<T, JsValue> {
        self.with_rng(|rng| match &mut rng.source {
            Source::ChaCha(rng) => Ok(action(rng)),
            Source::Callback(_) => Err(JsValue::from_str("random source is not ChaCha8")),
        })
    }
}
#[wasm_bindgen]
impl RandomWordSource {
    pub fn from_seed(seed: &[u8]) -> Result<Self, JsValue> {
        let seed: [u8; 32] = seed
            .try_into()
            .map_err(|_| JsValue::from_str("ChaCha8 seed must contain 32 bytes"))?;
        Ok(Self {
            rng: RefCell::new(WordRng {
                source: Source::ChaCha(Box::new(ChaCha8Rng::from_seed(seed))),
                failure: Rc::new(RefCell::new(None)),
            }),
        })
    }
    #[wasm_bindgen(constructor)]
    pub fn new(callback: Function) -> Self {
        Self {
            rng: RefCell::new(WordRng {
                source: Source::Callback(callback),
                failure: Rc::new(RefCell::new(None)),
            }),
        }
    }
    pub fn callback(&self) -> Result<Option<Function>, JsValue> {
        self.with_rng(|rng| {
            Ok(match &rng.source {
                Source::Callback(callback) => Some(callback.clone()),
                Source::ChaCha(_) => None,
            })
        })
    }
    pub fn next_u32(&self) -> Result<u32, JsValue> {
        self.with_rng(|rng| Ok(rng.next_u32()))
    }
    pub fn next_u64(&self) -> Result<u64, JsValue> {
        self.with_rng(|rng| Ok(rng.next_u64()))
    }
    pub fn fill_bytes(&self, count: usize) -> Result<Vec<u8>, JsValue> {
        if count > i32::MAX as usize {
            return Err(JsValue::from_str("random byte budget exceeded"));
        }
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(count)
            .map_err(|_| JsValue::from_str("random byte allocation failed"))?;
        bytes.resize(count, 0);
        self.with_rng(|rng| {
            rng.fill_bytes(&mut bytes);
            Ok(bytes)
        })
    }
    pub fn seed(&self) -> Result<Vec<u8>, JsValue> {
        self.with_chacha(|rng| rng.get_seed().to_vec())
    }
    pub fn stream(&self) -> Result<u64, JsValue> {
        self.with_chacha(|rng| rng.get_stream())
    }
    pub fn set_stream(&self, stream: u64) -> Result<(), JsValue> {
        self.with_chacha(|rng| rng.set_stream(stream))
    }
    /// All 128 input bits are retained; ChaCha itself defines its period wrap.
    pub fn word_position(&self) -> Result<Vec<u8>, JsValue> {
        self.with_chacha(|rng| rng.get_word_pos().to_le_bytes().to_vec())
    }
    pub fn set_word_position(&self, position: &[u8]) -> Result<(), JsValue> {
        let bytes: [u8; 16] = position
            .try_into()
            .map_err(|_| JsValue::from_str("word position must contain 16 bytes"))?;
        self.with_chacha(|rng| rng.set_word_pos(u128::from_le_bytes(bytes)))
    }
    pub fn cloned(&self) -> Result<Self, JsValue> {
        self.with_rng(|rng| {
            Ok(Self {
                rng: RefCell::new(rng.clone()),
            })
        })
    }
}
#[wasm_bindgen]
pub struct RandomSampler {
    sampler: RefCell<Sampler<WordRng>>,
    failure: Failure,
}
impl RandomSampler {
    fn with_sampler<T>(
        &self,
        action: impl FnOnce(&mut Sampler<WordRng>) -> Result<T, crate::Error>,
    ) -> Result<T, JsValue> {
        let mut sampler = self
            .sampler
            .try_borrow_mut()
            .map_err(|_| JsValue::from_str("random sampler is already in use"))?;
        finish(&self.failure, action(&mut sampler))
    }
}
#[wasm_bindgen]
impl RandomSampler {
    #[wasm_bindgen(constructor)]
    pub fn new(source: RandomWordSource, maximum: u32) -> Result<Self, JsValue> {
        let rng = source.rng.into_inner();
        let failure = Rc::clone(&rng.failure);
        Sampler::new(rng, maximum)
            .map(|sampler| Self {
                sampler: RefCell::new(sampler),
                failure,
            })
            .map_err(|error| JsValue::from_str(error.0))
    }
    pub fn from_seed(seed: &[u8]) -> Result<Self, JsValue> {
        Self::new(RandomWordSource::from_seed(seed)?, i32::MAX as u32)
    }
    #[wasm_bindgen(getter)]
    pub fn maximum(&self) -> Result<u32, JsValue> {
        self.with_sampler(|sampler| Ok(sampler.maximum()))
    }
    pub fn draw(&self) -> Result<u32, JsValue> {
        self.with_sampler(|sampler| Ok(sampler.draw()))
    }
    pub fn into_rng(self) -> RandomWordSource {
        RandomWordSource {
            rng: RefCell::new(self.sampler.into_inner().into_rng()),
        }
    }
    pub fn cloned(&self) -> Result<Self, JsValue> {
        // ChaCha state is independent; cloned JavaScript functions share caller state.
        let sampler = self
            .sampler
            .try_borrow()
            .map_err(|_| JsValue::from_str("random sampler is already in use"))?;
        let maximum = sampler.maximum();
        let rng = sampler.clone().into_rng();
        let failure = Rc::clone(&rng.failure);
        Ok(Self {
            sampler: RefCell::new(
                Sampler::new(rng, maximum).map_err(|error| JsValue::from_str(error.0))?,
            ),
            failure,
        })
    }
    pub fn uniform_f32(&self) -> Result<f32, JsValue> {
        self.with_sampler(|sampler| sampler.uniform())
    }
    pub fn uniform_f64(&self) -> Result<f64, JsValue> {
        self.with_sampler(|sampler| sampler.uniform())
    }
    pub fn normal_f32(&self, mean: f32, variance: f32) -> Result<f32, JsValue> {
        self.with_sampler(|sampler| sampler.normal(mean, variance))
    }
    pub fn normal_f64(&self, mean: f64, variance: f64) -> Result<f64, JsValue> {
        self.with_sampler(|sampler| sampler.normal(mean, variance))
    }
    pub fn normal_with_f32(&self, mean: f32, variance: f32, code: u32) -> Result<f32, JsValue> {
        let policy = policy(code)?;
        self.with_sampler(|sampler| sampler.normal_with(mean, variance, policy))
    }
    pub fn normal_with_f64(&self, mean: f64, variance: f64, code: u32) -> Result<f64, JsValue> {
        let policy = policy(code)?;
        self.with_sampler(|sampler| sampler.normal_with(mean, variance, policy))
    }
    pub fn white_noise_f32(&self, amplitude: f32, count: usize) -> Result<Vec<f32>, JsValue> {
        self.with_sampler(|sampler| sampler.white_noise(amplitude, count))
    }
    pub fn white_noise_f64(&self, amplitude: f64, count: usize) -> Result<Vec<f64>, JsValue> {
        self.with_sampler(|sampler| sampler.white_noise(amplitude, count))
    }
}
macro_rules! transformations {
    ($scalar:ty, $uniform:ident, $white:ident, $normal:ident) => {
        #[wasm_bindgen]
        pub fn $uniform(draw: u32, maximum: u32) -> Result<$scalar, JsValue> {
            random::uniform_from_draw(draw, maximum).map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $white(amplitude: $scalar, draw: u32, maximum: u32) -> Result<$scalar, JsValue> {
            random::white_from_draw(amplitude, draw, maximum)
                .map_err(|error| JsValue::from_str(error.0))
        }
        #[wasm_bindgen]
        pub fn $normal(
            mean: $scalar,
            variance: $scalar,
            first: u32,
            second: u32,
            maximum: u32,
            code: u32,
        ) -> Result<$scalar, JsValue> {
            random::normal_from_draws(mean, variance, [first, second], maximum, policy(code)?)
                .map_err(|error| JsValue::from_str(error.0))
        }
    };
}
transformations!(
    f32,
    random_uniform_from_draw_f32,
    random_white_from_draw_f32,
    random_normal_from_draws_f32
);
transformations!(
    f64,
    random_uniform_from_draw_f64,
    random_white_from_draw_f64,
    random_normal_from_draws_f64
);
