use ciglet_rs::statistics::Scalar;
use ciglet_rs_capi::*;
use std::ptr;
type Regular<T, O> = unsafe extern "C" fn(
    *const T,
    usize,
    CigletRsStftGrid,
    u32,
    CigletRsStftAnalysisOptions,
    O,
) -> u32;
type Legacy<T, O> = unsafe extern "C" fn(
    *const T,
    usize,
    CigletRsStftGrid,
    i32,
    CigletRsStftAnalysisOptions,
    O,
) -> u32;
struct Api<T, H, A, O, S, Q, R, I> {
    create: unsafe extern "C" fn(usize, u32, u32, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    analyze: unsafe extern "C" fn(*const H, A, CigletRsStftAnalysisOptions, O) -> u32,
    synthesize: unsafe extern "C" fn(*const H, S, Q, R) -> u32,
    regular: Regular<T, O>,
    legacy: Legacy<T, O>,
    regular_synthesis: unsafe extern "C" fn(S, CigletRsStftGrid, T, u32, R) -> u32,
    frames: fn(*const T, usize, *const i32, *const usize, usize) -> A,
    planes: fn(*mut T, usize, *mut T, usize, *mut I) -> O,
    spectrum: fn(*const T, *const T, usize) -> S,
    synthesis_options: fn(usize, i32, usize, T, u32) -> Q,
    samples: fn(*mut T, usize) -> R,
    info: fn(usize, usize, u32, T, T) -> I,
    view: fn(I) -> (usize, usize, u32, T, T),
}
struct Owner<'a, T, H, A, O, S, Q, R, I> {
    raw: *mut H,
    api: &'a Api<T, H, A, O, S, Q, R, I>,
}
impl<T, H, A, O, S, Q, R, I> Drop for Owner<'_, T, H, A, O, S, Q, R, I> {
    fn drop(&mut self) {
        // SAFETY: Unique initialized slot owns the matching handle or is null.
        assert_eq!(unsafe { (self.api.release)(&mut self.raw) }, 0);
    }
}
fn create<T, H, A, O, S, Q, R, I>(
    api: &Api<T, H, A, O, S, Q, R, I>,
    length: usize,
    window: u32,
    precision: u32,
) -> Owner<'_, T, H, A, O, S, Q, R, I> {
    let mut owner = Owner {
        raw: ptr::null_mut(),
        api,
    };
    // SAFETY: Independent initialized exclusive ownership slot.
    assert_eq!(
        unsafe { (api.create)(length, window, precision, &mut owner.raw) },
        0
    );
    owner
}
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        head
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self, width: usize) -> f64 {
        if width == 4 {
            f64::from(f32::from_le_bytes(self.bytes(4).try_into().unwrap()))
        } else {
            f64::from_le_bytes(self.bytes(8).try_into().unwrap())
        }
    }
}
fn analysis_reference<T: Scalar, H, A: Copy, O: Copy, S, Q, R, I: Copy>(
    bytes: &[u8],
    width: usize,
    api: &Api<T, H, A, O, S, Q, R, I>,
) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"STF1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 480);
    for record in 0..480 {
        let window = reader.integer() as u32;
        let precision = reader.integer() as u32;
        let subtract_mean = reader.integer() as u32;
        let variable = reader.integer() != 0;
        let scenario = reader.integer();
        let mask = reader.integer() as u32;
        let expected_norm = reader.scalar(width);
        let expected_weight = reader.scalar(width);
        let signal: Vec<T> = (0_i32..27)
            .map(|i| {
                T::from_f64(match scenario {
                    0 => {
                        if i == 11 {
                            0.75
                        } else {
                            0.
                        }
                    }
                    1 => f64::from(i % 9 - 4) * 0.125,
                    _ => 2.,
                })
                .unwrap()
            })
            .collect();
        let lengths = if variable { [8, 12, 16, 20] } else { [16; 4] };
        let centers = [-3, 5, 13, 21];
        let opts = CigletRsStftAnalysisOptions {
            subtract_mean,
            magnitude: mask & 1,
            phase: (mask >> 1) & 1,
            normalization_hop: if variable { 0 } else { 8 },
        };
        let mut magnitude = vec![T::zero(); if mask & 1 != 0 { 68 } else { 0 }];
        let mut phase = vec![T::zero(); if mask & 2 != 0 { 68 } else { 0 }];
        let mut info = (api.info)(71, 71, 71, T::zero(), T::zero());
        let arrays = (api.frames)(signal.as_ptr(), 27, centers.as_ptr(), lengths.as_ptr(), 4);
        let output = (api.planes)(
            magnitude.as_mut_ptr(),
            magnitude.len(),
            phase.as_mut_ptr(),
            phase.len(),
            &mut info,
        );
        let owner = create(api, 32, window, precision);
        // SAFETY: Live owner, stable initialized inputs and disjoint exclusive outputs.
        assert_eq!(unsafe { (api.analyze)(owner.raw, arrays, opts, output) }, 0);
        let (frames, bins, available, weight, normalization) = (api.view)(info);
        assert_eq!((frames, bins), (4, 17));
        assert_eq!(available, mask | 4 | if variable { 0 } else { 8 });
        if !variable {
            assert!((weight.into() - expected_weight).abs() < 1e-14);
            assert!((normalization.into() - expected_norm).abs() < 1e-14);
        }
        let mut full_magnitude = vec![T::zero(); 68];
        let mut full_phase = vec![T::zero(); 68];
        let full_output = (api.planes)(
            full_magnitude.as_mut_ptr(),
            68,
            full_phase.as_mut_ptr(),
            68,
            &mut info,
        );
        let full_opts = CigletRsStftAnalysisOptions {
            magnitude: 1,
            phase: 1,
            ..opts
        };
        // SAFETY: Same retained input and independent full output planes.
        assert_eq!(
            unsafe { (api.analyze)(owner.raw, arrays, full_opts, full_output) },
            0
        );
        for index in 0..68 {
            if mask & 1 != 0 {
                let expected = reader.scalar(width);
                let actual = magnitude[index].into();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.)
                        < if width == 4 { 2e-6 } else { 2e-14 },
                    "magnitude {record}/{index}"
                );
                assert_eq!(magnitude[index], full_magnitude[index]);
            }
            if mask & 2 != 0 {
                let expected = reader.scalar(width);
                let actual = phase[index].into();
                assert_eq!(phase[index], full_phase[index]);
                if full_magnitude[index].into() < 1e-6 {
                    continue;
                }
                let start = index / 17 * 17;
                let scale = full_magnitude[start..start + 17]
                    .iter()
                    .map(|&x| x.into())
                    .fold(1_f64, f64::max);
                let weighted =
                    2. * ((actual - expected) * 0.5).sin().abs() * full_magnitude[index].into()
                        / scale;
                assert!(
                    weighted < if width == 4 { 2e-6 } else { 2e-14 },
                    "phase {record}/{index}"
                );
            }
        }
    }
    assert!(reader.0.is_empty());
}
fn synthesis_reference<T: Scalar, H, A, O, S: Copy, Q: Copy, R: Copy, I>(
    bytes: &[u8],
    width: usize,
    api: &Api<T, H, A, O, S, Q, R, I>,
) {
    let mut reader = Reader(bytes);
    assert_eq!(reader.bytes(4), b"IST1");
    assert_eq!(reader.integer(), width);
    assert_eq!(reader.integer(), 288);
    let mut total = 0;
    for record in 0..288 {
        let length = reader.integer();
        let frames = reader.integer();
        let fade = reader.integer();
        let offset = reader.integer() as i32;
        let _scenario = reader.integer();
        let normalization = T::from_f64(reader.scalar(width)).unwrap();
        let mut magnitude = Vec::new();
        let mut phase = Vec::new();
        for _ in 0..frames * (length / 2 + 1) {
            magnitude.push(T::from_f64(reader.scalar(width)).unwrap());
            phase.push(T::from_f64(reader.scalar(width)).unwrap());
        }
        let count = reader.integer();
        let expected: Vec<_> = (0..count).map(|_| reader.scalar(width)).collect();
        let mut samples = vec![T::zero(); count];
        let owner = create(api, length, 3, 0);
        let input = (api.spectrum)(magnitude.as_ptr(), phase.as_ptr(), magnitude.len());
        let options = (api.synthesis_options)(4, offset, fade, normalization, 1);
        let output = (api.samples)(samples.as_mut_ptr(), count);
        // SAFETY: Retained live owner, stable spectral inputs and exclusive output.
        assert_eq!(
            unsafe { (api.synthesize)(owner.raw, input, options, output) },
            0
        );
        for (index, (&actual, &expected)) in samples.iter().zip(&expected).enumerate() {
            assert!(
                (actual.into() - expected).abs() / expected.abs().max(1.)
                    < if width == 4 { 2e-7 } else { 2e-15 },
                "inverse {record}/{index}"
            );
        }
        let first = samples.clone();
        // SAFETY: Same owner/input and caller-exclusive output buffer.
        assert_eq!(
            unsafe { (api.synthesize)(owner.raw, input, options, output) },
            0
        );
        assert_eq!(samples, first);
        total += count;
    }
    assert!(reader.0.is_empty());
    eprintln!("STFT C boundary width={width}: {total} original inverse samples checked");
}

fn features<T: Scalar, H, A: Copy, O: Copy, S: Copy, Q: Copy, R: Copy, I: Copy>(
    api: &Api<T, H, A, O, S, Q, R, I>,
) {
    let v = |x| T::from_f64(x).unwrap();
    let mut opts = CigletRsStftAnalysisOptions {
        subtract_mean: 71,
        magnitude: 71,
        phase: 71,
        normalization_hop: 71,
    };
    // SAFETY: Independent initialized exclusive options output slot.
    assert_eq!(
        unsafe { ciglet_rs_stft_analysis_options_default(&mut opts) },
        0
    );
    assert_eq!(
        (
            opts.subtract_mean,
            opts.magnitude,
            opts.phase,
            opts.normalization_hop
        ),
        (0, 1, 1, 0)
    );
    let signal: Vec<T> = (0..32).map(|i| v(f64::from(i) * 0.0625)).collect();
    let grid = CigletRsStftGrid {
        hop: 8,
        frames: 4,
        window_factor: 2,
        zero_padding_factor: 2,
    };
    let centers = [0, 8, 16, 24];
    let lengths = [16; 4];
    let mut magnitude = vec![T::zero(); 68];
    let mut phase = vec![T::zero(); 68];
    let mut info = (api.info)(71, 71, 71, v(71.), v(71.));
    let output = (api.planes)(
        magnitude.as_mut_ptr(),
        68,
        phase.as_mut_ptr(),
        68,
        &mut info,
    );
    // SAFETY: Stable initialized inputs and disjoint exclusive arrays/metadata.
    unsafe {
        for precision in [0, 1] {
            assert_eq!(
                (api.regular)(signal.as_ptr(), 32, grid, precision, opts, output),
                0
            );
            let expected = (magnitude.clone(), phase.clone(), (api.view)(info));
            let owner = create(api, 32, 3, precision);
            let input = (api.frames)(signal.as_ptr(), 32, centers.as_ptr(), lengths.as_ptr(), 4);
            let explicit = CigletRsStftAnalysisOptions {
                normalization_hop: 8,
                ..opts
            };
            assert_eq!((api.analyze)(owner.raw, input, explicit, output), 0);
            assert_eq!(
                (&magnitude, &phase, (api.view)(info)),
                (&expected.0, &expected.1, expected.2)
            );
        }
        for level in [i32::MIN, 2, 3, i32::MAX] {
            assert_eq!(
                (api.legacy)(signal.as_ptr(), 32, grid, level, opts, output),
                0
            );
            let expected = (magnitude.clone(), phase.clone(), (api.view)(info));
            assert_eq!(
                (api.regular)(
                    signal.as_ptr(),
                    32,
                    grid,
                    u32::from(level < 3),
                    opts,
                    output
                ),
                0
            );
            assert_eq!(
                (&magnitude, &phase, (api.view)(info)),
                (&expected.0, &expected.1, expected.2)
            );
        }
        let normalization = (api.view)(info).4;
        assert_ne!(normalization, T::zero());
        let owner = create(api, 32, 3, 0);
        let spectra = (api.spectrum)(magnitude.as_ptr(), phase.as_ptr(), 68);
        let mut samples = vec![T::zero(); 32];
        let wave = (api.samples)(samples.as_mut_ptr(), 32);
        for nyquist in [0, 1] {
            assert_eq!(
                (api.regular_synthesis)(spectra, grid, normalization, nyquist, wave),
                0
            );
            let expected = samples.clone();
            let configuration = (api.synthesis_options)(8, 0, 32, normalization, nyquist);
            assert_eq!((api.synthesize)(owner.raw, spectra, configuration, wave), 0);
            assert_eq!(samples, expected);
        }
        let empty = (api.frames)(signal.as_ptr(), 32, ptr::null(), ptr::null(), 0);
        let empty_output = (api.planes)(ptr::null_mut(), 0, ptr::null_mut(), 0, &mut info);
        assert_eq!((api.analyze)(owner.raw, empty, opts, empty_output), 0);
        assert_eq!((api.view)(info), (0, 17, 3, T::zero(), T::zero()));
        for mask in 0..4 {
            let opts = CigletRsStftAnalysisOptions {
                magnitude: mask & 1,
                phase: (mask >> 1) & 1,
                ..opts
            };
            let mc = if mask & 1 != 0 { 68 } else { 0 };
            let pc = if mask & 2 != 0 { 68 } else { 0 };
            let output = (api.planes)(
                if mc == 0 {
                    ptr::null_mut()
                } else {
                    magnitude.as_mut_ptr()
                },
                mc,
                if pc == 0 {
                    ptr::null_mut()
                } else {
                    phase.as_mut_ptr()
                },
                pc,
                &mut info,
            );
            assert_eq!((api.regular)(signal.as_ptr(), 32, grid, 0, opts, output), 0);
            assert_eq!((api.view)(info).2, mask | 12);
        }
        let owner = create(api, 8, 0, 0);
        let odd_signal = [v(1.), v(2.), v(3.)];
        let center = [1];
        let length = [3];
        let input = (api.frames)(odd_signal.as_ptr(), 3, center.as_ptr(), length.as_ptr(), 1);
        let mut m = [T::zero(); 5];
        let mut p = [T::zero(); 5];
        let output = (api.planes)(m.as_mut_ptr(), 5, p.as_mut_ptr(), 5, &mut info);
        assert_eq!((api.analyze)(owner.raw, input, opts, output), 0);
        assert_eq!(m[0], v(6.));
        assert_eq!((api.view)(info), (1, 5, 7, v(3.), T::zero()));
        let explicit = CigletRsStftAnalysisOptions {
            normalization_hop: 2,
            ..opts
        };
        assert_eq!((api.analyze)(owner.raw, input, explicit, output), 0);
        assert_eq!((api.view)(info), (1, 5, 15, v(3.), v(2.)));
        let mean = CigletRsStftAnalysisOptions {
            subtract_mean: 1,
            ..opts
        };
        assert_eq!((api.analyze)(owner.raw, input, mean, output), 0);
        assert_eq!(m[0], T::zero());
    }
}

fn errors<T: Scalar, H, A: Copy, O: Copy, S: Copy, Q: Copy, R: Copy, I: Copy>(
    api: &Api<T, H, A, O, S, Q, R, I>,
    nonfinite: T,
) {
    let v = |x| T::from_f64(x).unwrap();
    let owner = create(api, 8, 0, 0);
    let mut slot = ptr::null_mut();
    let mut signal = [T::zero(); 40];
    signal[32..].fill(nonfinite);
    let centers = [0, 36];
    let lengths = [8, 8];
    let input = (api.frames)(signal.as_ptr(), 40, centers.as_ptr(), lengths.as_ptr(), 2);
    let mut magnitude = [v(71.); 10];
    let mut phase = [v(71.); 10];
    let mut info = (api.info)(71, 71, 71, v(71.), v(71.));
    let output = (api.planes)(
        magnitude.as_mut_ptr(),
        10,
        phase.as_mut_ptr(),
        10,
        &mut info,
    );
    let opts = CigletRsStftAnalysisOptions {
        subtract_mean: 1,
        magnitude: 1,
        phase: 1,
        normalization_hop: 0,
    };
    // SAFETY: Invalid ranges/options/overlaps are checked before access; otherwise
    // retained live owners and independent initialized arrays/metadata are used.
    unsafe {
        assert_eq!((api.analyze)(owner.raw, input, opts, output), 3);
        assert_eq!(magnitude, [v(71.); 10]);
        assert_eq!(phase, [v(71.); 10]);
        assert_eq!((api.view)(info), (71, 71, 71, v(71.), v(71.)));
        for (length, window, precision) in [(0, 0, 0), (7, 0, 0), (8, 7, 0), (8, 0, 2)] {
            assert_eq!((api.create)(length, window, precision, &mut slot), 3);
            assert!(slot.is_null());
        }
        let bad_opts = CigletRsStftAnalysisOptions {
            magnitude: 2,
            ..opts
        };
        assert_eq!((api.analyze)(owner.raw, input, bad_opts, output), 3);
        assert_eq!((api.analyze)(ptr::null(), input, opts, output), 1);
        let wrong = (api.planes)(magnitude.as_mut_ptr(), 9, phase.as_mut_ptr(), 10, &mut info);
        assert_eq!((api.analyze)(owner.raw, input, opts, wrong), 2);
        let overlap = (api.planes)(
            magnitude.as_mut_ptr(),
            10,
            magnitude.as_mut_ptr().add(1),
            10,
            &mut info,
        );
        assert_eq!((api.analyze)(owner.raw, input, opts, overlap), 2);
        let missing = (api.planes)(
            magnitude.as_mut_ptr(),
            10,
            phase.as_mut_ptr(),
            10,
            ptr::null_mut(),
        );
        assert_eq!((api.analyze)(owner.raw, input, opts, missing), 1);
        let bad = (&mut info as *mut I).cast::<u8>().add(1).cast::<I>();
        assert_eq!(
            (api.analyze)(
                owner.raw,
                input,
                opts,
                (api.planes)(magnitude.as_mut_ptr(), 10, phase.as_mut_ptr(), 10, bad)
            ),
            1
        );
        let bad_input = (api.frames)(ptr::null(), 40, centers.as_ptr(), lengths.as_ptr(), 2);
        assert_eq!((api.analyze)(owner.raw, bad_input, opts, output), 1);
        let bad_input = (api.frames)(
            signal.as_ptr(),
            usize::MAX,
            centers.as_ptr(),
            lengths.as_ptr(),
            2,
        );
        assert_eq!((api.analyze)(owner.raw, bad_input, opts, output), 2);
        let invalid_lengths = [8, 9];
        let bad_input = (api.frames)(
            signal.as_ptr(),
            40,
            centers.as_ptr(),
            invalid_lengths.as_ptr(),
            2,
        );
        assert_eq!((api.analyze)(owner.raw, bad_input, opts, output), 3);
        let invalid_grid = CigletRsStftGrid {
            hop: 0,
            frames: 2,
            window_factor: 1,
            zero_padding_factor: 1,
        };
        assert_eq!(
            (api.regular)(signal.as_ptr(), 40, invalid_grid, 0, opts, output),
            3
        );
        let grid = CigletRsStftGrid {
            hop: 4,
            frames: 2,
            window_factor: 2,
            zero_padding_factor: 1,
        };
        assert_eq!((api.regular)(signal.as_ptr(), 40, grid, 2, opts, output), 3);
        let mut backing = [71_u64; 16];
        let saved = backing;
        assert!(std::mem::align_of::<I>() <= std::mem::align_of::<u64>());
        let shared_info = backing.as_mut_ptr().cast::<I>();
        let shared_plane = backing.as_mut_ptr().cast::<T>();
        assert_eq!(
            (api.analyze)(
                owner.raw,
                input,
                opts,
                (api.planes)(shared_plane, 10, phase.as_mut_ptr(), 10, shared_info)
            ),
            2
        );
        assert_eq!(backing, saved);
        assert_eq!(
            (api.analyze)(
                owner.raw,
                input,
                opts,
                (api.planes)(magnitude.as_mut_ptr(), 10, shared_plane, 10, shared_info)
            ),
            2
        );
        assert_eq!(backing, saved);
        let source = [T::zero(); 10];
        let spectra = (api.spectrum)(source.as_ptr(), source.as_ptr(), 10);
        let mut wave = [v(71.); 8];
        let output = (api.samples)(wave.as_mut_ptr(), 8);
        for options in [
            (4, 0, 0, v(0.), 0),
            (4, 0, 9, v(1.), 0),
            (4, 0, 0, v(1.), 2),
        ] {
            assert_eq!(
                (api.synthesize)(
                    owner.raw,
                    spectra,
                    (api.synthesis_options)(options.0, options.1, options.2, options.3, options.4),
                    output
                ),
                3
            );
        }
        assert_eq!(
            (api.synthesize)(
                owner.raw,
                spectra,
                (api.synthesis_options)(4, 0, 0, v(1.), 0),
                (api.samples)(wave.as_mut_ptr(), 7)
            ),
            2
        );
        assert_eq!((api.regular_synthesis)(spectra, grid, v(1.), 0, output), 3);
        assert_eq!(wave, [v(71.); 8]);
        assert_eq!(magnitude, [v(71.); 10]);
        assert_eq!(phase, [v(71.); 10]);
        assert_eq!((api.view)(info), (71, 71, 71, v(71.), v(71.)));
        assert_eq!((api.release)(&mut slot), 0);
        assert_eq!((api.release)(&mut slot), 0);
        assert_eq!(ciglet_rs_stft_analysis_options_default(ptr::null_mut()), 1);
    }
}

fn alias_and_concurrent_reuse<T: Scalar, H, A: Copy, O: Copy, S: Copy, Q, R, I: Copy>(
    api: &Api<T, H, A, O, S, Q, R, I>,
) {
    let v = |x| T::from_f64(x).unwrap();
    let owner = create(api, 32, 3, 0);
    let signal: [T; 32] = std::array::from_fn(|i| v(i as f64 * 0.125 - 2.));
    let centers = [0, 8, 16, 24];
    let lengths = [16; 4];
    let options = CigletRsStftAnalysisOptions {
        subtract_mean: 1,
        magnitude: 1,
        phase: 1,
        normalization_hop: 8,
    };
    let mut magnitude = [T::zero(); 68];
    let mut phase = [T::zero(); 68];
    let mut info = (api.info)(0, 0, 0, T::zero(), T::zero());
    // SAFETY: Live owner and checked independent stable arrays/metadata.
    assert_eq!(
        unsafe {
            (api.analyze)(
                owner.raw,
                (api.frames)(signal.as_ptr(), 32, centers.as_ptr(), lengths.as_ptr(), 4),
                options,
                (api.planes)(
                    magnitude.as_mut_ptr(),
                    68,
                    phase.as_mut_ptr(),
                    68,
                    &mut info,
                ),
            )
        },
        0
    );
    let mut shared = [T::zero(); 68];
    shared[..32].copy_from_slice(&signal);
    let mut alias_phase = [T::zero(); 68];
    // SAFETY: Input/output alias is permitted; output planes and metadata are disjoint.
    assert_eq!(
        unsafe {
            (api.analyze)(
                owner.raw,
                (api.frames)(shared.as_ptr(), 32, centers.as_ptr(), lengths.as_ptr(), 4),
                options,
                (api.planes)(
                    shared.as_mut_ptr(),
                    68,
                    alias_phase.as_mut_ptr(),
                    68,
                    &mut info,
                ),
            )
        },
        0
    );
    assert_eq!(shared, magnitude);
    assert_eq!(alias_phase, phase);
    let normalization = (api.view)(info).4;
    let mut wave = [T::zero(); 32];
    // SAFETY: Live owner and independent spectral inputs/sample output.
    assert_eq!(
        unsafe {
            (api.synthesize)(
                owner.raw,
                (api.spectrum)(magnitude.as_ptr(), phase.as_ptr(), 68),
                (api.synthesis_options)(8, 0, 32, normalization, 0),
                (api.samples)(wave.as_mut_ptr(), 32),
            )
        },
        0
    );
    // SAFETY: Shared spectral input/sample output allocation is retained through
    // calculation; all input borrows end before the first output write.
    assert_eq!(
        unsafe {
            (api.synthesize)(
                owner.raw,
                (api.spectrum)(shared.as_ptr(), phase.as_ptr(), 68),
                (api.synthesis_options)(8, 0, 32, normalization, 0),
                (api.samples)(shared.as_mut_ptr(), 32),
            )
        },
        0
    );
    assert_eq!(shared[..32], wave);
    let expected_weight = (api.view)(info).3;
    let address = owner.raw as usize;
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(move || {
                for _ in 0..50 {
                    let mut actual_magnitude = [T::zero(); 68];
                    let mut actual_phase = [T::zero(); 68];
                    let mut actual_info = (api.info)(0, 0, 0, T::zero(), T::zero());
                    // SAFETY: Scoped workers finish before owner release. The
                    // owner's mutex serializes scratch; worker outputs are private.
                    assert_eq!(
                        unsafe {
                            (api.analyze)(
                                address as *const H,
                                (api.frames)(
                                    signal.as_ptr(),
                                    32,
                                    centers.as_ptr(),
                                    lengths.as_ptr(),
                                    4,
                                ),
                                options,
                                (api.planes)(
                                    actual_magnitude.as_mut_ptr(),
                                    68,
                                    actual_phase.as_mut_ptr(),
                                    68,
                                    &mut actual_info,
                                ),
                            )
                        },
                        0
                    );
                    assert_eq!(actual_magnitude, magnitude);
                    assert_eq!(actual_phase, phase);
                    assert_eq!(
                        (api.view)(actual_info),
                        (4, 17, 15, expected_weight, normalization)
                    );
                }
            });
        }
    });
}

const FLOAT: Api<
    f32,
    CigletRsStftF32,
    CigletRsStftAnalysisInputF32,
    CigletRsStftAnalysisOutputF32,
    CigletRsStftSpectrumF32,
    CigletRsStftSynthesisOptionsF32,
    CigletRsStftSamplesF32,
    CigletRsStftInfoF32,
> = Api {
    create: ciglet_rs_stft_create_f32,
    release: ciglet_rs_stft_release_f32,
    analyze: ciglet_rs_stft_analyze_f32,
    synthesize: ciglet_rs_stft_synthesize_f32,
    regular: ciglet_rs_stft_analyze_regular_f32,
    legacy: ciglet_rs_stft_analyze_regular_legacy_f32,
    regular_synthesis: ciglet_rs_stft_synthesize_regular_f32,
    frames: |signal, signal_count, centers, window_lengths, frames| CigletRsStftAnalysisInputF32 {
        signal,
        signal_count,
        centers,
        window_lengths,
        frames,
    },
    planes: |magnitude, magnitude_count, phase, phase_count, info| CigletRsStftAnalysisOutputF32 {
        magnitude,
        magnitude_count,
        phase,
        phase_count,
        info,
    },
    spectrum: |magnitude, phase, count| CigletRsStftSpectrumF32 {
        magnitude,
        phase,
        count,
    },
    synthesis_options: |hop, offset, fade_samples, normalization, nyquist| {
        CigletRsStftSynthesisOptionsF32 {
            hop,
            offset,
            fade_samples,
            normalization,
            nyquist,
        }
    },
    samples: |samples, count| CigletRsStftSamplesF32 { samples, count },
    info: |frames, bins, available, window_weight, normalization| CigletRsStftInfoF32 {
        frames,
        bins,
        available,
        window_weight,
        normalization,
    },
    view: |value| {
        (
            value.frames,
            value.bins,
            value.available,
            value.window_weight,
            value.normalization,
        )
    },
};
#[test]
fn float_original_analysis() {
    analysis_reference(
        include_bytes!("../../../tests/fixtures/c-stft-f32.bin"),
        4,
        &FLOAT,
    );
}
#[test]
fn float_original_synthesis() {
    synthesis_reference(
        include_bytes!("../../../tests/fixtures/c-istft-f32.bin"),
        4,
        &FLOAT,
    );
}
#[test]
fn float_features_and_regular_equivalence() {
    features(&FLOAT);
}
#[test]
fn float_atomic_errors() {
    errors(&FLOAT, f32::NAN);
}

const DOUBLE: Api<
    f64,
    CigletRsStftF64,
    CigletRsStftAnalysisInputF64,
    CigletRsStftAnalysisOutputF64,
    CigletRsStftSpectrumF64,
    CigletRsStftSynthesisOptionsF64,
    CigletRsStftSamplesF64,
    CigletRsStftInfoF64,
> = Api {
    create: ciglet_rs_stft_create_f64,
    release: ciglet_rs_stft_release_f64,
    analyze: ciglet_rs_stft_analyze_f64,
    synthesize: ciglet_rs_stft_synthesize_f64,
    regular: ciglet_rs_stft_analyze_regular_f64,
    legacy: ciglet_rs_stft_analyze_regular_legacy_f64,
    regular_synthesis: ciglet_rs_stft_synthesize_regular_f64,
    frames: |signal, signal_count, centers, window_lengths, frames| CigletRsStftAnalysisInputF64 {
        signal,
        signal_count,
        centers,
        window_lengths,
        frames,
    },
    planes: |magnitude, magnitude_count, phase, phase_count, info| CigletRsStftAnalysisOutputF64 {
        magnitude,
        magnitude_count,
        phase,
        phase_count,
        info,
    },
    spectrum: |magnitude, phase, count| CigletRsStftSpectrumF64 {
        magnitude,
        phase,
        count,
    },
    synthesis_options: |hop, offset, fade_samples, normalization, nyquist| {
        CigletRsStftSynthesisOptionsF64 {
            hop,
            offset,
            fade_samples,
            normalization,
            nyquist,
        }
    },
    samples: |samples, count| CigletRsStftSamplesF64 { samples, count },
    info: |frames, bins, available, window_weight, normalization| CigletRsStftInfoF64 {
        frames,
        bins,
        available,
        window_weight,
        normalization,
    },
    view: |value| {
        (
            value.frames,
            value.bins,
            value.available,
            value.window_weight,
            value.normalization,
        )
    },
};
#[test]
fn double_original_analysis() {
    analysis_reference(
        include_bytes!("../../../tests/fixtures/c-stft-f64.bin"),
        8,
        &DOUBLE,
    );
}
#[test]
fn double_original_synthesis() {
    synthesis_reference(
        include_bytes!("../../../tests/fixtures/c-istft-f64.bin"),
        8,
        &DOUBLE,
    );
}
#[test]
fn double_features_and_regular_equivalence() {
    features(&DOUBLE);
}
#[test]
fn double_atomic_errors() {
    errors(&DOUBLE, f64::NAN);
}

macro_rules! overflow_equivalence {
    ($name:ident, $scalar:ty, $api:ident) => {
        #[test]
        fn $name() {
            use ciglet_rs::{
                stft::{AnalysisOptions, StftPlan},
                window::{Precision, Window},
            };
            let signal = [<$scalar>::MAX; 8];
            let centers = [4];
            let lengths = [8];
            let mut native = StftPlan::new(8, Window::Boxcar, Precision::Accurate).unwrap();
            let expected = native
                .analyze(
                    &signal,
                    &centers,
                    &lengths,
                    AnalysisOptions {
                        subtract_mean: true,
                        ..AnalysisOptions::default()
                    },
                )
                .unwrap();
            let owner = create(&$api, 8, 0, 0);
            let mut magnitude = [71.; 5];
            let mut phase = [71.; 5];
            let mut info = ($api.info)(71, 71, 71, 71., 71.);
            // SAFETY: Live owner, readable input arrays, and disjoint writable outputs.
            assert_eq!(
                unsafe {
                    ($api.analyze)(
                        owner.raw,
                        ($api.frames)(signal.as_ptr(), 8, centers.as_ptr(), lengths.as_ptr(), 1),
                        CigletRsStftAnalysisOptions {
                            subtract_mean: 1,
                            magnitude: 1,
                            phase: 1,
                            normalization_hop: 0,
                        },
                        ($api.planes)(magnitude.as_mut_ptr(), 5, phase.as_mut_ptr(), 5, &mut info),
                    )
                },
                0
            );
            assert!(magnitude.iter().any(|value| !value.is_finite()));
            for (actual, expected) in magnitude.iter().chain(&phase).zip(
                expected
                    .magnitude
                    .unwrap()
                    .iter()
                    .chain(expected.phase.unwrap().iter()),
            ) {
                if expected.is_nan() {
                    assert!(actual.is_nan());
                } else {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
            }
            assert_eq!(($api.view)(info), (1, 5, 7, 8., 0.));
        }
    };
}
overflow_equivalence!(float_native_overflow_equivalence, f32, FLOAT);
overflow_equivalence!(double_native_overflow_equivalence, f64, DOUBLE);

#[test]
fn float_alias_and_concurrent_reuse() {
    alias_and_concurrent_reuse(&FLOAT);
}
#[test]
fn double_alias_and_concurrent_reuse() {
    alias_and_concurrent_reuse(&DOUBLE);
}
