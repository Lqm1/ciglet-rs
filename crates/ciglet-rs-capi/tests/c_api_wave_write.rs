use ciglet_rs_capi::*;
use std::{ffi::c_void, ptr};

struct Sink {
    bytes: Vec<u8>,
    writes: usize,
    flushes: usize,
    failure: u32,
}
unsafe extern "C" fn write(
    context: *mut c_void,
    bytes: *const u8,
    count: usize,
    written: *mut usize,
) -> u32 {
    // SAFETY: Test supplies live exclusive context, readable source and writable count.
    unsafe {
        let sink = &mut *context.cast::<Sink>();
        sink.writes += 1;
        match sink.failure {
            1 => return 9,
            2 => {
                *written = count + 1;
                return 0;
            }
            3 => {
                *written = 0;
                return 0;
            }
            4 if sink.bytes.len() >= 8 => return 9,
            _ => {}
        }
        let n = count.min(1);
        sink.bytes
            .extend_from_slice(std::slice::from_raw_parts(bytes, n));
        *written = n;
    }
    0
}
unsafe extern "C" fn flush(context: *mut c_void) -> u32 {
    // SAFETY: Live exclusive test context.
    unsafe {
        (*context.cast::<Sink>()).flushes += 1;
    }
    0
}
fn stream(sink: &mut Sink) -> CigletRsWriteStream {
    CigletRsWriteStream {
        context: ptr::from_mut(sink).cast(),
        write: Some(write),
        flush: Some(flush),
    }
}
fn options() -> CigletRsWaveWriteOptions {
    CigletRsWaveWriteOptions {
        bits_per_sample: 8,
        pcm8_quantization: 0,
        pad_odd_data: 1,
    }
}
fn path_bytes(path: &std::path::Path) -> Vec<u8> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str()
            .encode_wide()
            .flat_map(u16::to_le_bytes)
            .collect()
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }
}
macro_rules! corpus {
    ($name:ident,$t:ty,$width:expr,$fixture:expr,$encode:ident,$stream:ident,$file:ident) => {
        #[test]
        fn $name() {
            let bytes = $fixture;
            assert_eq!(&bytes[..4], b"WVW1");
            assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), $width);
            assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 12);
            let mut cursor = 12;
            for case in 0..12 {
                let mut integers = [0; 3];
                for v in &mut integers {
                    *v = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
                    cursor += 4;
                }
                let [bits, rate, count] = integers;
                let count = count as usize;
                let samples: Vec<$t> = (0..count)
                    .map(|_| {
                        let v = <$t>::from_le_bytes(
                            bytes[cursor..cursor + $width as usize].try_into().unwrap(),
                        );
                        cursor += $width as usize;
                        v
                    })
                    .collect();
                let length =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                let expected = &bytes[cursor..cursor + length];
                cursor += length;
                let mut settings = options();
                let mut owner = ptr::null_mut();
                let mut cloned = ptr::null_mut();
                let mut size = 0;
                let mut sink = Sink {
                    bytes: vec![],
                    writes: 0,
                    flushes: 0,
                    failure: 0,
                };
                let path = std::env::temp_dir().join(format!(
                    "wave-write-{}-{}-{case}.wav",
                    stringify!($name),
                    std::process::id()
                ));
                let native = path_bytes(&path);
                let raw = CigletRsPath {
                    bytes: native.as_ptr(),
                    byte_count: native.len(),
                    encoding: 1,
                };
                // SAFETY: Stable sample/path allocations and exclusive slots; callbacks retain their valid context.
                unsafe {
                    assert_eq!(
                        ciglet_rs_wave_write_options_legacy(bits as u16, &mut settings),
                        0
                    );
                    assert_eq!(
                        $encode(samples.as_ptr(), count, rate, settings, &mut owner),
                        0
                    );
                    assert_eq!(ciglet_rs_bytes_length(owner, &mut size), 0);
                    assert_eq!(size, length);
                    assert_eq!(ciglet_rs_bytes_clone(owner, &mut cloned), 0);
                    assert_eq!(ciglet_rs_bytes_release(&mut owner), 0);
                    let mut output = vec![71; size];
                    assert_eq!(
                        ciglet_rs_bytes_copy(cloned, 0, output.as_mut_ptr(), size),
                        0
                    );
                    assert_eq!(output, expected);
                    assert_eq!(ciglet_rs_bytes_copy(cloned, size, ptr::null_mut(), 0), 0);
                    output.fill(71);
                    assert_eq!(
                        ciglet_rs_bytes_copy(cloned, 1, output.as_mut_ptr(), size),
                        2
                    );
                    assert!(output.iter().all(|&v| v == 71));
                    assert_eq!(
                        ciglet_rs_bytes_copy(cloned, 1, output.as_mut_ptr(), size - 1),
                        0
                    );
                    assert_eq!(&output[..size - 1], &expected[1..]);
                    assert_eq!(ciglet_rs_bytes_release(&mut cloned), 0);
                    assert_eq!(ciglet_rs_bytes_release(&mut cloned), 0);
                    assert_eq!(
                        $stream(stream(&mut sink), samples.as_ptr(), count, rate, settings),
                        0
                    );
                    assert_eq!(sink.bytes, expected);
                    assert_eq!(sink.flushes, 0);
                    assert_eq!(sink.writes, size);
                    assert_eq!($file(raw, samples.as_ptr(), count, rate, settings), 0);
                    assert_eq!(std::fs::read(&path).unwrap(), expected);
                }
                std::fs::remove_file(path).unwrap();
            }
            assert_eq!(cursor, bytes.len());
        }
    };
}
corpus!(
    original_float_bytes,
    f32,
    4,
    include_bytes!("../../../tests/fixtures/c-wave-write-f32.bin"),
    ciglet_rs_wave_encode_f32,
    ciglet_rs_wave_write_stream_f32,
    ciglet_rs_wave_write_file_f32
);
corpus!(
    original_double_bytes,
    f64,
    8,
    include_bytes!("../../../tests/fixtures/c-wave-write-f64.bin"),
    ciglet_rs_wave_encode_f64,
    ciglet_rs_wave_write_stream_f64,
    ciglet_rs_wave_write_file_f64
);

#[test]
fn native_defaults_centered_clipping_and_padding() {
    let mut settings = options();
    let mut owner = ptr::null_mut();
    let mut size = 0;
    // SAFETY: Stable samples and local exclusively writable slots.
    unsafe {
        assert_eq!(ciglet_rs_wave_write_options_default(&mut settings), 0);
        assert_eq!(
            settings,
            CigletRsWaveWriteOptions {
                bits_per_sample: 16,
                pcm8_quantization: 0,
                pad_odd_data: 1
            }
        );
        settings.bits_per_sample = 8;
        let samples = [-2.0f32, -1.0, 0.0, 1.0, 2.0];
        assert_eq!(
            ciglet_rs_wave_encode_f32(samples.as_ptr(), 5, 16000, settings, &mut owner),
            0
        );
        assert_eq!(ciglet_rs_bytes_length(owner, &mut size), 0);
        assert_eq!(size, 50);
        let mut result = [0u8; 6];
        assert_eq!(ciglet_rs_bytes_copy(owner, 44, result.as_mut_ptr(), 6), 0);
        assert_eq!(result, [0, 0, 128, 255, 255, 0]);
        assert_eq!(ciglet_rs_bytes_release(&mut owner), 0);
        assert_eq!(
            ciglet_rs_wave_encode_f64(ptr::null(), 0, 16000, settings, &mut owner),
            0
        );
        assert_eq!(ciglet_rs_bytes_length(owner, &mut size), 0);
        assert_eq!(size, 44);
        assert_eq!(ciglet_rs_bytes_release(&mut owner), 0);
    }
    assert_eq!(size_of::<CigletRsWaveWriteOptions>(), 12);
    assert_eq!(
        std::mem::offset_of!(CigletRsWaveWriteOptions, pcm8_quantization),
        4
    );
    assert_eq!(
        std::mem::offset_of!(CigletRsWaveWriteOptions, pad_odd_data),
        8
    );
}
#[test]
fn borrowed_partial_output_and_callback_errors() {
    let samples = [0.0f64, 0.5, -0.5];
    for failure in 1..=4 {
        let mut sink = Sink {
            bytes: vec![],
            writes: 0,
            flushes: 0,
            failure,
        };
        // SAFETY: Stable samples and valid non-unwinding synchronous callback/context.
        assert_eq!(
            unsafe {
                ciglet_rs_wave_write_stream_f64(
                    stream(&mut sink),
                    samples.as_ptr(),
                    3,
                    16000,
                    options(),
                )
            },
            3
        );
        assert_eq!(sink.flushes, 0);
        assert_eq!(sink.bytes.len(), if failure == 4 { 8 } else { 0 });
    }
}
#[test]
fn invalid_parameters_precede_stream_or_file_mutation() {
    let samples = [0.0f64, f64::NAN];
    let mut sink = Sink {
        bytes: vec![71],
        writes: 0,
        flushes: 0,
        failure: 0,
    };
    let path = std::env::temp_dir().join(format!("wave-write-invalid-{}.wav", std::process::id()));
    std::fs::write(&path, [71u8]).unwrap();
    let native = path_bytes(&path);
    let raw = CigletRsPath {
        bytes: native.as_ptr(),
        byte_count: native.len(),
        encoding: 1,
    };
    let mut owner = ptr::null_mut();
    // SAFETY: Checked live context, stable arrays/paths and local ownership slot.
    unsafe {
        assert_eq!(
            ciglet_rs_wave_encode_f64(samples.as_ptr(), 2, 16000, options(), &mut owner),
            3
        );
        assert!(owner.is_null());
        assert_eq!(
            ciglet_rs_wave_write_stream_f64(
                stream(&mut sink),
                samples.as_ptr(),
                2,
                16000,
                options()
            ),
            3
        );
        assert_eq!(
            ciglet_rs_wave_write_file_f64(raw, samples.as_ptr(), 2, 16000, options()),
            3
        );
        for invalid in [
            CigletRsWaveWriteOptions {
                bits_per_sample: 12,
                ..options()
            },
            CigletRsWaveWriteOptions {
                pcm8_quantization: 99,
                ..options()
            },
            CigletRsWaveWriteOptions {
                pad_odd_data: 2,
                ..options()
            },
        ] {
            assert_eq!(
                ciglet_rs_wave_write_stream_f64(
                    stream(&mut sink),
                    samples.as_ptr(),
                    1,
                    16000,
                    invalid
                ),
                3
            );
            assert_eq!(
                ciglet_rs_wave_write_file_f64(raw, samples.as_ptr(), 1, 16000, invalid),
                3
            );
        }
        assert_eq!(
            ciglet_rs_wave_write_stream_f64(stream(&mut sink), ptr::null(), 1, 16000, options()),
            1
        );
        assert_eq!(
            ciglet_rs_wave_write_stream_f64(
                stream(&mut sink),
                samples.as_ptr(),
                usize::MAX,
                16000,
                options()
            ),
            2
        );
        let missing = CigletRsWriteStream {
            write: None,
            ..stream(&mut sink)
        };
        assert_eq!(
            ciglet_rs_wave_write_stream_f64(missing, samples.as_ptr(), 1, 16000, options()),
            1
        );
        assert_eq!(
            ciglet_rs_wave_encode_f64(samples.as_ptr(), 1, 16000, options(), ptr::null_mut()),
            1
        );
        assert_eq!(ciglet_rs_wave_write_options_default(ptr::null_mut()), 1);
        assert_eq!(ciglet_rs_bytes_length(ptr::null(), ptr::null_mut()), 1);
    }
    assert_eq!(sink.bytes, [71]);
    assert_eq!(sink.writes, 0);
    assert_eq!(sink.flushes, 0);
    assert_eq!(std::fs::read(&path).unwrap(), [71]);
    std::fs::remove_file(path).unwrap();
}
