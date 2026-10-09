use ciglet_rs::{
    random::{self, Generator, Policy},
    statistics::Scalar,
};
use rand::{Rng, TryRng};
use std::convert::Infallible;

#[derive(Debug)]
struct ZeroRng {
    draws: usize,
}
impl TryRng for ZeroRng {
    type Error = Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        self.draws += 1;
        Ok(0)
    }
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        self.draws += 2;
        Ok(0)
    }
    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Infallible> {
        self.draws += bytes.len().div_ceil(4);
        bytes.fill(0);
        Ok(())
    }
}

#[test]
fn named_stream_and_pathological_zero_source() {
    let mut rng = Generator::from_seed([0; 32]).into_rng();
    // First four words of the zero-key, zero-counter ChaCha8 block.
    assert_eq!(
        [
            rng.next_u32(),
            rng.next_u32(),
            rng.next_u32(),
            rng.next_u32()
        ],
        [0x2fef003e, 0xd6405f89, 0xe8b85b7f, 0xa1a5091f]
    );
    let mut zero = random::Sampler::new(ZeroRng { draws: 0 }, i32::MAX as u32).unwrap();
    assert!(zero.normal(0.0_f64, 1.0).is_err());
    assert_eq!(zero.into_rng().draws, 128);
    let mut zero = random::Sampler::new(ZeroRng { draws: 0 }, i32::MAX as u32).unwrap();
    assert!(
        zero.normal_with(0.0_f64, 1.0, Policy::Legacy)
            .unwrap()
            .is_finite()
    );
    assert_eq!(zero.into_rng().draws, 2);
    assert!(random::Sampler::new(ZeroRng { draws: 0 }, 0).is_err());
}

fn reference<T: Scalar>(bytes: &[u8], width: usize) {
    assert_eq!(&bytes[..4], b"RND1");
    let mut cursor = 4;
    let integer = |cursor: &mut usize| {
        let value = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        value
    };
    assert_eq!(integer(&mut cursor) as usize, width);
    assert_eq!(integer(&mut cursor), 128);
    for case in 0..128 {
        let maximum = integer(&mut cursor);
        let draws = [integer(&mut cursor), integer(&mut cursor)];
        let mut values = Vec::new();
        for _ in 0..7 {
            let value = if width == 4 {
                f32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap())
            };
            cursor += width;
            values.push(T::from_f64(value).unwrap());
        }
        let actual = [
            random::uniform_from_draw::<T>(draws[0], maximum),
            random::normal_from_draws(values[0], values[1], draws, maximum, Policy::Legacy),
            random::white_from_draw(values[2], draws[0], maximum),
            random::white_from_draw(values[2], draws[1], maximum),
        ];
        for (field, (actual, expected)) in actual.into_iter().zip(&values[3..]).enumerate() {
            let expected: f64 = (*expected).into();
            if expected.is_finite() {
                assert_eq!(
                    actual.unwrap().into().to_bits(),
                    expected.to_bits(),
                    "case {case} field {field}"
                );
            } else {
                assert!(actual.is_err(), "case {case} field {field}");
            }
        }
    }
    assert_eq!(cursor, bytes.len());
}

#[test]
fn unchanged_c_transformations() {
    reference::<f32>(
        include_bytes!("../../../tests/fixtures/random-float.bin"),
        4,
    );
    reference::<f64>(
        include_bytes!("../../../tests/fixtures/random-double.bin"),
        8,
    );
}

#[test]
fn analytic_domains_and_extreme_variance() {
    assert_eq!(random::uniform_from_draw::<f64>(0, 8).unwrap(), -1.0);
    assert_eq!(random::uniform_from_draw::<f32>(8, 8).unwrap(), 1.0);
    assert_eq!(random::uniform_from_draw::<f64>(4, 8).unwrap(), 0.0);
    let value = random::normal_from_draws(0.0, 1.0, [4, 0], 8, Policy::Stable).unwrap();
    assert!((value - (2.0_f64 * 2.0_f64.ln()).sqrt()).abs() < 1e-15);
    assert_eq!(
        random::normal_from_draws(-3.0, 0.0, [0, 8], 8, Policy::Stable).unwrap(),
        -3.0
    );
    assert!(random::normal_from_draws(0.0, 1.0, [0, 8], 8, Policy::Stable).is_err());
    assert!(random::normal_from_draws(0.0, -1.0, [1, 1], 8, Policy::Legacy).is_err());
    assert!(random::normal_from_draws(f64::NAN, 1.0, [1, 1], 8, Policy::Stable).is_err());
    let value = random::normal_from_draws(0.0, f64::MAX, [1, 0], 8, Policy::Stable).unwrap();
    assert!(value.is_finite() && value > 1e154);
    assert_eq!(random::white_from_draw(f64::MAX, 8, 8).unwrap(), f64::MAX);
    assert_eq!(random::white_from_draw(-2.0, 0, 8).unwrap(), 2.0);
    for maximum in [0, u32::MAX] {
        assert!(random::uniform_from_draw::<f64>(0, maximum).is_err());
    }
    assert!(random::uniform_from_draw::<f64>(9, 8).is_err());
    assert!(random::white_from_draw(f64::INFINITY, 0, 8).is_err());
}

#[test]
fn seeded_consumption_clone_and_validation() {
    let mut generator = Generator::from_seed([37; 32]);
    let mut reference = generator.clone();
    assert!(generator.white_noise(1.0_f64, 0).unwrap().is_empty());
    assert!(generator.white_noise(f64::NAN, 5).is_err());
    assert!(generator.white_noise(1.0, i32::MAX as usize + 1).is_err());
    assert!(generator.normal(0.0, -1.0).is_err());
    assert_eq!(generator.draw(), reference.draw());
    assert_eq!(generator.normal(3.0_f64, 0.0).unwrap(), 3.0);
    reference.draw();
    reference.draw();
    assert_eq!(generator.draw(), reference.draw());
    let noise = generator.white_noise(0.0_f64, 16).unwrap();
    assert!(noise.iter().all(|&x| x == 0.0));
    for _ in 0..16 {
        reference.draw();
    }
    assert_eq!(generator.draw(), reference.draw());
    let mut reference = generator.clone();
    let draws = [reference.draw(), reference.draw()];
    assert_eq!(
        generator.normal_with(-1.0, 0.5, Policy::Legacy).unwrap(),
        random::normal_from_draws(-1.0, 0.5, draws, generator.maximum(), Policy::Legacy).unwrap()
    );
    assert_eq!(generator.draw(), reference.draw());
}

#[test]
fn independent_distribution_moments() {
    let count = 100_000;
    let mut generator = Generator::from_seed([13; 32]);
    let uniform = generator.white_noise(1.0_f64, count).unwrap();
    let mean = uniform.iter().sum::<f64>() / count as f64;
    let second = uniform.iter().map(|x| x * x).sum::<f64>() / count as f64;
    // Greater than six standard errors, with a small deterministic margin.
    assert!(mean.abs() < 0.012 && (second - 1.0 / 3.0).abs() < 0.008);
    let mut sum = 0.0;
    let mut squares = 0.0;
    for _ in 0..count {
        let value = generator.normal(2.0_f64, 4.0).unwrap();
        sum += value;
        squares += (value - 2.0).powi(2);
    }
    assert!((sum / count as f64 - 2.0).abs() < 0.04);
    assert!((squares / count as f64 - 4.0).abs() < 0.12);
}
