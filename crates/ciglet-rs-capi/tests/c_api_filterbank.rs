use ciglet_rs_capi::*;
use rustfft::FftNum;

type Constructor<T, H> = unsafe extern "C" fn(usize, T, usize, *mut *mut H) -> u32;
type Mel<T, H> = unsafe extern "C" fn(usize, T, usize, T, T, T, T, *mut *mut H) -> u32;
type MelDefault<T, H> = unsafe extern "C" fn(usize, T, usize, T, T, *mut *mut H) -> u32;
type Response<T, H> = unsafe extern "C" fn(*const H, usize, *mut T, usize) -> u32;
type SetChannel<T, H> = unsafe extern "C" fn(*mut H, usize, *const T, usize, usize, usize) -> u32;
type Apply<T, H> = unsafe extern "C" fn(*const H, *const T, usize, u32, *mut T, usize) -> u32;
type ApplyFrames<T, H> =
    unsafe extern "C" fn(*const H, *const T, usize, usize, u32, *mut T, usize) -> u32;
type Support<H> = unsafe extern "C" fn(*const H, usize, *mut usize, usize) -> u32;
struct Api<T, H> {
    empty: Constructor<T, H>,
    mel: Mel<T, H>,
    mel_default: MelDefault<T, H>,
    plp: Constructor<T, H>,
    cloned: unsafe extern "C" fn(*const H, *mut *mut H) -> u32,
    release: unsafe extern "C" fn(*mut *mut H) -> u32,
    shape: unsafe extern "C" fn(*const H, *mut usize, usize) -> u32,
    nyquist: unsafe extern "C" fn(*const H, *mut T) -> u32,
    set_nyquist: unsafe extern "C" fn(*mut H, T) -> u32,
    response: Response<T, H>,
    support: Support<H>,
    set_channel: SetChannel<T, H>,
    apply: Apply<T, H>,
    apply_spectrogram: Apply<T, H>,
    apply_frames: ApplyFrames<T, H>,
}
struct Reader<'a>(&'a [u8], usize);
impl Reader<'_> {
    fn take(&mut self, n: usize) -> &[u8] {
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        a
    }
    fn integer(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }
    fn scalar(&mut self) -> f64 {
        if self.1 == 4 {
            f32::from_le_bytes(self.take(4).try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(self.take(8).try_into().unwrap())
        }
    }
}
fn scalar<T: FftNum>(value: f64) -> T {
    T::from_f64(value).unwrap()
}
fn close<T: Copy + Into<f64>>(actual: &[T], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        let actual = actual.into();
        let error = (actual - expected).abs() / expected.abs().max(1.0);
        assert!(
            actual == expected || error <= tolerance || actual.is_nan() && expected.is_nan(),
            "{actual} vs {expected}: {error}"
        );
    }
}
fn compare<T: FftNum + Into<f64>, H>(bytes: &[u8], width: usize, custom: bool, api: Api<T, H>) {
    let mut reader = Reader(bytes, width);
    assert_eq!(reader.take(4), if custom { b"CFB1" } else { b"FBK1" });
    assert_eq!(reader.integer(), width);
    let records = if custom { 33 } else { 72 };
    assert_eq!(reader.integer(), records);
    for _ in 0..records {
        let kind = reader.integer();
        let bins = reader.integer();
        let channels = reader.integer();
        let metadata = scalar::<T>(reader.scalar());
        let mut bank = std::ptr::null_mut();
        // SAFETY: All slots and buffers below are local independent allocations;
        // the typed bank is owned until release, with exclusive mutations.
        unsafe {
            let status = if custom {
                if kind < 5 {
                    (api.empty)(bins, metadata, channels, &mut bank)
                } else {
                    (api.plp)(bins, metadata, channels, &mut bank)
                }
            } else {
                match kind {
                    0 => (api.empty)(bins, metadata, channels, &mut bank),
                    1 => (api.mel)(
                        bins,
                        metadata,
                        channels,
                        scalar(50.0),
                        metadata,
                        scalar(0.85),
                        scalar(120.0),
                        &mut bank,
                    ),
                    2 => (api.mel_default)(
                        bins,
                        metadata,
                        channels,
                        scalar(50.0),
                        metadata,
                        &mut bank,
                    ),
                    _ => (api.plp)(bins, metadata, channels, &mut bank),
                }
            };
            assert_eq!(status, 0);
            let mut shape = [71; 2];
            assert_eq!((api.shape)(bank, shape.as_mut_ptr(), 2), 0);
            assert_eq!(shape, [bins, channels]);
            let mut stored = T::zero();
            assert_eq!((api.nyquist)(bank, &mut stored), 0);
            assert!(stored == metadata || stored.into().is_nan() && metadata.into().is_nan());
            for channel in 0..channels {
                let start = reader.integer();
                let end = reader.integer();
                let expected: Vec<f64> = (0..bins).map(|_| reader.scalar()).collect();
                if custom && kind < 5 {
                    let response: Vec<T> = expected.iter().copied().map(scalar).collect();
                    assert_eq!(
                        (api.set_channel)(bank, channel, response.as_ptr(), bins, start, end),
                        0
                    );
                }
                let mut support = [71; 2];
                assert_eq!((api.support)(bank, channel, support.as_mut_ptr(), 2), 0);
                assert_eq!(support, [start, end]);
                let mut actual = vec![T::zero(); bins];
                assert_eq!((api.response)(bank, channel, actual.as_mut_ptr(), bins), 0);
                close(
                    &actual,
                    &expected,
                    if custom {
                        0.0
                    } else if width == 4 {
                        2e-7
                    } else {
                        2e-14
                    },
                );
            }
            let spectra: Vec<T> = if custom {
                (0..3 * bins).map(|_| scalar(reader.scalar())).collect()
            } else {
                (0..3)
                    .flat_map(|scenario| {
                        (0..bins).map(move |bin| {
                            scalar(match scenario {
                                0 => 0.0,
                                1 => (bin % 17 + 1) as f64 * 0.125,
                                _ => {
                                    if bin == bins / 3 {
                                        2.0
                                    } else {
                                        0.0
                                    }
                                }
                            })
                        })
                    })
                    .collect()
            };
            let mut copy = std::ptr::null_mut();
            assert_eq!((api.cloned)(bank, &mut copy), 0);
            for compression in [0, 1] {
                let tolerance = if custom {
                    if width == 4 { 2e-7 } else { 2e-15 }
                } else if width == 4 {
                    2e-6
                } else {
                    2e-13
                };
                let mut expected_matrix = Vec::new();
                let mut matrix = vec![T::zero(); 3 * channels];
                assert_eq!(
                    (api.apply_frames)(
                        bank,
                        spectra.as_ptr(),
                        spectra.len(),
                        3,
                        compression,
                        matrix.as_mut_ptr(),
                        matrix.len()
                    ),
                    0
                );
                for frame in 0..3 {
                    let expected: Vec<f64> = (0..channels).map(|_| reader.scalar()).collect();
                    let mut actual = vec![T::zero(); channels];
                    let input = &spectra[frame * bins..(frame + 1) * bins];
                    assert_eq!(
                        (api.apply)(
                            bank,
                            input.as_ptr(),
                            bins,
                            compression,
                            actual.as_mut_ptr(),
                            channels
                        ),
                        0
                    );
                    close(&actual, &expected, tolerance);
                    expected_matrix.extend(expected);
                }
                close(&matrix, &expected_matrix, tolerance);
                let repeated: Vec<f64> = (0..3 * channels).map(|_| reader.scalar()).collect();
                assert_eq!(expected_matrix, repeated);
                let mut inferred = vec![T::zero(); matrix.len()];
                if bins != 0 {
                    assert_eq!(
                        (api.apply_spectrogram)(
                            bank,
                            spectra.as_ptr(),
                            spectra.len(),
                            compression,
                            inferred.as_mut_ptr(),
                            inferred.len()
                        ),
                        0
                    );
                    assert_eq!(inferred, matrix);
                } else {
                    assert_eq!(
                        (api.apply_spectrogram)(
                            bank,
                            std::ptr::null(),
                            0,
                            compression,
                            inferred.as_mut_ptr(),
                            inferred.len()
                        ),
                        3
                    );
                }
                assert_eq!((api.set_nyquist)(copy, scalar(f64::NEG_INFINITY)), 0);
                assert_eq!((api.nyquist)(copy, &mut stored), 0);
                assert_eq!(stored.into(), f64::NEG_INFINITY);
                assert_eq!(
                    (api.apply_frames)(
                        copy,
                        spectra.as_ptr(),
                        spectra.len(),
                        3,
                        compression,
                        inferred.as_mut_ptr(),
                        inferred.len()
                    ),
                    0
                );
                assert_eq!(inferred, matrix);
            }
            assert_eq!((api.release)(&mut copy), 0);
            assert_eq!((api.release)(&mut bank), 0);
            assert!(copy.is_null() && bank.is_null());
            assert_eq!((api.release)(&mut bank), 0);
        }
    }
    assert!(reader.0.is_empty());
}

fn api_f32() -> Api<f32, CigletRsFilterBankF32> {
    Api {
        empty: ciglet_rs_filterbank_empty_f32,
        mel: ciglet_rs_filterbank_mel_f32,
        mel_default: ciglet_rs_filterbank_mel_default_f32,
        plp: ciglet_rs_filterbank_plp_f32,
        cloned: ciglet_rs_filterbank_clone_f32,
        release: ciglet_rs_filterbank_release_f32,
        shape: ciglet_rs_filterbank_shape_f32,
        nyquist: ciglet_rs_filterbank_nyquist_f32,
        set_nyquist: ciglet_rs_filterbank_set_nyquist_f32,
        response: ciglet_rs_filterbank_response_f32,
        support: ciglet_rs_filterbank_support_f32,
        set_channel: ciglet_rs_filterbank_set_channel_f32,
        apply: ciglet_rs_filterbank_apply_f32,
        apply_spectrogram: ciglet_rs_filterbank_apply_spectrogram_f32,
        apply_frames: ciglet_rs_filterbank_apply_frames_f32,
    }
}

#[test]
fn generated_f32_banks_match_original() {
    compare(
        include_bytes!("../../../tests/fixtures/c-filterbank-f32.bin"),
        4,
        false,
        api_f32(),
    );
}

#[test]
fn custom_f32_banks_match_original() {
    compare(
        include_bytes!("../../../tests/fixtures/custom_filterbank-float.bin"),
        4,
        true,
        api_f32(),
    );
}

fn api_f64() -> Api<f64, CigletRsFilterBankF64> {
    Api {
        empty: ciglet_rs_filterbank_empty_f64,
        mel: ciglet_rs_filterbank_mel_f64,
        mel_default: ciglet_rs_filterbank_mel_default_f64,
        plp: ciglet_rs_filterbank_plp_f64,
        cloned: ciglet_rs_filterbank_clone_f64,
        release: ciglet_rs_filterbank_release_f64,
        shape: ciglet_rs_filterbank_shape_f64,
        nyquist: ciglet_rs_filterbank_nyquist_f64,
        set_nyquist: ciglet_rs_filterbank_set_nyquist_f64,
        response: ciglet_rs_filterbank_response_f64,
        support: ciglet_rs_filterbank_support_f64,
        set_channel: ciglet_rs_filterbank_set_channel_f64,
        apply: ciglet_rs_filterbank_apply_f64,
        apply_spectrogram: ciglet_rs_filterbank_apply_spectrogram_f64,
        apply_frames: ciglet_rs_filterbank_apply_frames_f64,
    }
}

#[test]
fn generated_f64_banks_match_original() {
    compare(
        include_bytes!("../../../tests/fixtures/c-filterbank-f64.bin"),
        8,
        false,
        api_f64(),
    );
}

#[test]
fn custom_f64_banks_match_original() {
    compare(
        include_bytes!("../../../tests/fixtures/custom_filterbank-double.bin"),
        8,
        true,
        api_f64(),
    );
}

#[test]
fn custom_supports_bits_overlap_and_atomic_errors() {
    let mut bank = std::ptr::null_mut();
    // SAFETY: Locally owned typed handles and independent initialized buffers;
    // mutable calls exclude all other handle access until they return.
    unsafe {
        assert_eq!(ciglet_rs_filterbank_empty_f64(4, 8000.0, 1, &mut bank), 0);
        let response = [f64::from_bits(0x7ff8000000000042), 0.5, 1.0, f64::INFINITY];
        assert_eq!(
            ciglet_rs_filterbank_set_channel_f64(bank, 0, response.as_ptr(), 4, 1, 3),
            0
        );
        let mut copy = std::ptr::null_mut();
        assert_eq!(ciglet_rs_filterbank_clone_f64(bank, &mut copy), 0);
        let mut copied = [0.0_f64; 4];
        assert_eq!(
            ciglet_rs_filterbank_response_f64(copy, 0, copied.as_mut_ptr(), 4),
            0
        );
        assert_eq!(copied.map(f64::to_bits), response.map(f64::to_bits));
        let metadata = f64::from_bits(0x7ff8000000000071);
        assert_eq!(ciglet_rs_filterbank_set_nyquist_f64(bank, metadata), 0);
        let mut stored = 0.0;
        assert_eq!(ciglet_rs_filterbank_nyquist_f64(bank, &mut stored), 0);
        assert_eq!(stored.to_bits(), metadata.to_bits());
        let mut spectrum = [f64::NAN, 2.0, 3.0, f64::NAN];
        let mut expected = [0.0];
        assert_eq!(
            ciglet_rs_filterbank_apply_f64(bank, spectrum.as_ptr(), 4, 1, expected.as_mut_ptr(), 1),
            0
        );
        assert_eq!(
            ciglet_rs_filterbank_apply_f64(bank, spectrum.as_ptr(), 4, 1, spectrum.as_mut_ptr(), 1),
            0
        );
        assert_eq!(spectrum[0], expected[0]);
        assert_eq!(spectrum[1..3], [2.0, 3.0]);
        let matrix = [0.0, 2.0, 3.0, 0.0, 0.0, f64::NAN, 3.0, 0.0];
        let mut output = [71.0; 2];
        assert_eq!(
            ciglet_rs_filterbank_apply_frames_f64(
                bank,
                matrix.as_ptr(),
                8,
                2,
                0,
                output.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(output, [71.0; 2]);
        assert_eq!(
            ciglet_rs_filterbank_apply_f64(bank, matrix.as_ptr(), 4, 2, output.as_mut_ptr(), 1),
            3
        );
        assert_eq!(
            ciglet_rs_filterbank_apply_f64(bank, matrix.as_ptr(), 4, 0, output.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            ciglet_rs_filterbank_apply_frames_f64(
                bank,
                matrix.as_ptr(),
                8,
                3,
                0,
                output.as_mut_ptr(),
                2
            ),
            3
        );
        assert_eq!(output, [71.0; 2]);
        let invalid = [0.0, f64::NAN, 1.0, 0.0];
        assert_eq!(
            ciglet_rs_filterbank_set_channel_f64(bank, 0, invalid.as_ptr(), 4, 1, 3),
            3
        );
        assert_eq!(
            ciglet_rs_filterbank_set_channel_f64(bank, 0, response.as_ptr(), 4, 3, 1),
            3
        );
        assert_eq!(
            ciglet_rs_filterbank_response_f64(bank, 0, copied.as_mut_ptr(), 4),
            0
        );
        assert_eq!(copied.map(f64::to_bits), response.map(f64::to_bits));
        let retained = bank;
        assert_eq!(
            ciglet_rs_filterbank_mel_default_f64(0, 8000.0, 1, 0.0, 8000.0, &mut bank),
            3
        );
        assert_eq!(bank, retained);
        let mut indices = [71; 2];
        assert_eq!(
            ciglet_rs_filterbank_shape_f64(bank, indices.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            ciglet_rs_filterbank_support_f64(bank, 1, indices.as_mut_ptr(), 2),
            3
        );
        assert_eq!(indices, [71; 2]);
        assert_eq!(ciglet_rs_filterbank_release_f64(&mut copy), 0);
        assert_eq!(ciglet_rs_filterbank_release_f64(&mut bank), 0);
        assert!(copy.is_null() && bank.is_null());
    }
}

#[test]
fn explicit_zero_bin_frames_and_empty_channels() {
    let mut bank = std::ptr::null_mut();
    // SAFETY: Local independent slots/buffers; empty buffers use valid null contracts.
    unsafe {
        assert_eq!(ciglet_rs_filterbank_empty_f32(0, f32::NAN, 1, &mut bank), 0);
        assert_eq!(
            ciglet_rs_filterbank_set_channel_f32(bank, 0, std::ptr::null(), 0, 0, 0),
            0
        );
        assert_eq!(
            ciglet_rs_filterbank_response_f32(bank, 0, std::ptr::null_mut(), 0),
            0
        );
        let mut output = [71.0; 3];
        assert_eq!(
            ciglet_rs_filterbank_apply_frames_f32(
                bank,
                std::ptr::null(),
                0,
                3,
                1,
                output.as_mut_ptr(),
                3
            ),
            0
        );
        assert_eq!(output, [0.0; 3]);
        assert_eq!(
            ciglet_rs_filterbank_apply_spectrogram_f32(
                bank,
                std::ptr::null(),
                0,
                1,
                output.as_mut_ptr(),
                3
            ),
            3
        );
        assert_eq!(ciglet_rs_filterbank_release_f32(&mut bank), 0);
        assert_eq!(ciglet_rs_filterbank_empty_f32(4, 8000.0, 0, &mut bank), 0);
        let input = [f32::NAN; 4];
        assert_eq!(
            ciglet_rs_filterbank_apply_f32(bank, input.as_ptr(), 4, 0, std::ptr::null_mut(), 0),
            0
        );
        assert_eq!(
            ciglet_rs_filterbank_apply_frames_f32(
                bank,
                input.as_ptr(),
                4,
                2,
                0,
                std::ptr::null_mut(),
                0
            ),
            3
        );
        assert_eq!(ciglet_rs_filterbank_release_f32(&mut bank), 0);
        assert_eq!(ciglet_rs_filterbank_release_f32(&mut bank), 0);
    }
}
