#![cfg(feature = "c-api")]
use ciglet_rs::{c_api::*, wave};
use std::{
    ffi::c_void,
    io::{Cursor, Read, Seek, SeekFrom},
    ptr,
};

fn first_wave() -> Vec<u8> {
    let bytes = include_bytes!("fixtures/c-wave-f64.bin");
    let count = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    bytes[16..16 + count].to_vec()
}
struct Context {
    reader: Cursor<Vec<u8>>,
    reads: usize,
    seeks: usize,
    failure: u32,
}
unsafe extern "C" fn read(
    context: *mut c_void,
    bytes: *mut u8,
    count: usize,
    read_count: *mut usize,
) -> u32 {
    // SAFETY: Tests provide live exclusive context and initialized writable callback buffers.
    unsafe {
        let ctx = &mut *context.cast::<Context>();
        ctx.reads += 1;
        if ctx.failure == 1 {
            return 9;
        }
        if ctx.failure == 2 {
            *read_count = count + 1;
            return 0;
        }
        if ctx.failure == 3 {
            *read_count = 0;
            return 0;
        }
        let buffer = std::slice::from_raw_parts_mut(bytes, count.min(1));
        *read_count = ctx.reader.read(buffer).unwrap();
    }
    0
}
unsafe extern "C" fn seek(
    context: *mut c_void,
    origin: u32,
    absolute: u64,
    relative: i64,
    position: *mut u64,
) -> u32 {
    // SAFETY: Tests provide live exclusive context and writable position storage.
    unsafe {
        let ctx = &mut *context.cast::<Context>();
        ctx.seeks += 1;
        if ctx.failure == 4 {
            return 9;
        }
        let from = match origin {
            0 => SeekFrom::Start(absolute),
            1 => SeekFrom::Current(relative),
            2 => SeekFrom::End(relative),
            _ => return 9,
        };
        match ctx.reader.seek(from) {
            Ok(value) => *position = value,
            Err(_) => return 9,
        }
    }
    0
}
macro_rules! corpus {
    ($name:ident,$t:ty,$width:expr,$fixture:expr,$decode:ident,$info:ident,$copy:ident,$clone:ident,$release:ident) => {
        #[test]
        fn $name() {
            let bytes = $fixture;
            assert_eq!(&bytes[..4], b"WAV1");
            assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), $width);
            assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 36);
            let mut cursor = 12;
            for _ in 0..36 {
                let length =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                let input = &bytes[cursor..cursor + length];
                cursor += length;
                let mut owner = ptr::null_mut();
                let mut cloned = ptr::null_mut();
                let mut metadata = CigletRsWaveInfo {
                    sample_rate: 0,
                    bits_per_sample: 0,
                    channels: 0,
                    encoding: 99,
                    sample_count: 0,
                };
                let mut samples = [71.0 as $t; 7];
                // SAFETY: Immutable fixture and initialized local slots/copy buffers.
                unsafe {
                    assert_eq!($decode(input.as_ptr(), input.len(), 0, 7, &mut owner), 0);
                    assert_eq!($info(owner, &mut metadata), 0);
                    assert_eq!(metadata.sample_count, 7);
                    assert_eq!(metadata.sample_rate, 16000);
                    assert_eq!(
                        metadata.channels,
                        u16::from_le_bytes(input[22..24].try_into().unwrap())
                    );
                    assert_eq!(
                        metadata.bits_per_sample,
                        u16::from_le_bytes(input[34..36].try_into().unwrap())
                    );
                    let tag = u16::from_le_bytes(input[20..22].try_into().unwrap());
                    let tag = if tag == 65534 {
                        u16::from_le_bytes(input[44..46].try_into().unwrap())
                    } else {
                        tag
                    };
                    assert_eq!(metadata.encoding, if tag == 1 { 0 } else { 1 });
                    assert_eq!($clone(owner, &mut cloned), 0);
                    assert_eq!($release(&mut owner), 0);
                    assert_eq!($copy(cloned, samples.as_mut_ptr(), 7), 0);
                    assert_eq!($copy(cloned, samples.as_mut_ptr(), 6), 2);
                    assert_eq!($copy(cloned, ptr::null_mut(), 7), 1);
                    assert_eq!($release(&mut cloned), 0);
                    assert_eq!($release(&mut cloned), 0);
                }
                let scale = 2.0f64.powi(metadata.bits_per_sample as i32 - 1);
                let integers = [
                    -scale,
                    -scale / 2.0,
                    -1.0,
                    0.0,
                    1.0,
                    scale / 4.0,
                    scale - 1.0,
                ];
                let floats = [-1.25, -0.5, -0.0000001, 0.0, 0.25, 1.5, 1e-30];
                let extensible = u16::from_le_bytes(input[20..22].try_into().unwrap()) == 65534;
                for i in 0..7 {
                    let legacy = <$t>::from_le_bytes(
                        bytes[cursor..cursor + $width as usize].try_into().unwrap(),
                    );
                    cursor += $width as usize;
                    let expected = if metadata.encoding == 0 {
                        integers[i] / scale
                    } else if metadata.bits_per_sample == 32 {
                        floats[i] as f32 as f64
                    } else {
                        floats[i]
                    };
                    assert_eq!(samples[i].to_bits(), (expected as $t).to_bits());
                    let corrected = metadata.encoding == 0
                        && (extensible
                            || metadata.bits_per_sample == 8
                            || ($width == 4 && metadata.bits_per_sample == 32));
                    if !corrected {
                        assert_eq!(samples[i].to_bits(), legacy.to_bits());
                    }
                }
                // SAFETY: Input remains valid; budget and truncation are numerical errors.
                unsafe {
                    assert_eq!($decode(input.as_ptr(), input.len(), 0, 6, &mut owner), 3);
                    assert!(owner.is_null());
                    assert_eq!(
                        $decode(input.as_ptr(), input.len() - 1, 0, 7, &mut owner),
                        3
                    );
                    assert!(owner.is_null());
                }
            }
            assert_eq!(cursor, bytes.len());
        }
    };
}
corpus!(
    original_float_corpus,
    f32,
    4,
    include_bytes!("fixtures/c-wave-f32.bin"),
    ciglet_rs_wave_decode_f32,
    ciglet_rs_wave_info_f32,
    ciglet_rs_wave_copy_samples_f32,
    ciglet_rs_wave_clone_f32,
    ciglet_rs_wave_release_f32
);
corpus!(
    original_double_corpus,
    f64,
    8,
    include_bytes!("fixtures/c-wave-f64.bin"),
    ciglet_rs_wave_decode_f64,
    ciglet_rs_wave_info_f64,
    ciglet_rs_wave_copy_samples_f64,
    ciglet_rs_wave_clone_f64,
    ciglet_rs_wave_release_f64
);

#[test]
fn borrowed_stream_short_reads_offsets_and_errors() {
    let input = first_wave();
    let mut prefixed = vec![13, 14, 15];
    prefixed.extend(&input);
    let mut ctx = Context {
        reader: Cursor::new(prefixed),
        reads: 0,
        seeks: 0,
        failure: 0,
    };
    ctx.reader.set_position(3);
    let stream = CigletRsReadStream {
        context: ptr::from_mut(&mut ctx).cast(),
        read: Some(read),
        seek: Some(seek),
    };
    let mut owner = ptr::null_mut();
    let mut samples = [0.0f64; 7];
    // SAFETY: Local context remains valid; callbacks do not unwind or alias output.
    unsafe {
        assert_eq!(ciglet_rs_wave_read_stream_f64(stream, 7, &mut owner), 0);
        assert!(ctx.reads > 10 && ctx.seeks > 0);
        assert_eq!(
            ciglet_rs_wave_copy_samples_f64(owner, samples.as_mut_ptr(), 7),
            0
        );
        assert_eq!(
            samples,
            wave::read::<f64, _>(&mut Cursor::new(input), 7)
                .unwrap()
                .samples
                .as_slice()
        );
        assert_eq!(ciglet_rs_wave_release_f64(&mut owner), 0);
        for failure in 1..=4 {
            ctx.reader.set_position(3);
            ctx.failure = failure;
            assert_eq!(ciglet_rs_wave_read_stream_f64(stream, 7, &mut owner), 3);
            assert!(owner.is_null());
        }
        ctx.failure = 0;
        let reads = ctx.reads;
        let seeks = ctx.seeks;
        assert_eq!(
            ciglet_rs_wave_read_stream_f64(stream, 7, ptr::null_mut()),
            1
        );
        assert_eq!(ctx.reads, reads);
        assert_eq!(ctx.seeks, seeks);
        let missing = CigletRsReadStream {
            read: None,
            ..stream
        };
        assert_eq!(ciglet_rs_wave_read_stream_f64(missing, 7, &mut owner), 1);
        assert_eq!(ctx.reads, reads);
        assert_eq!(ctx.seeks, seeks);
    }
}
#[test]
fn memory_positions_and_pointer_rejection() {
    let input = first_wave();
    let mut prefix = vec![13, 14, 15];
    prefix.extend(&input);
    let mut owner = ptr::null_mut();
    // SAFETY: Local buffers and initialized ownership slot; invalid pointers rejected.
    unsafe {
        assert_eq!(
            ciglet_rs_wave_decode_f32(prefix.as_ptr(), prefix.len(), 3, 7, &mut owner),
            0
        );
        assert_eq!(ciglet_rs_wave_release_f32(&mut owner), 0);
        assert_eq!(
            ciglet_rs_wave_decode_f32(input.as_ptr(), input.len(), u64::MAX, 7, &mut owner),
            3
        );
        assert_eq!(
            ciglet_rs_wave_decode_f32(ptr::null(), 1, 0, 7, &mut owner),
            1
        );
        assert_eq!(
            ciglet_rs_wave_decode_f32(input.as_ptr(), usize::MAX, 0, 7, &mut owner),
            2
        );
        assert_eq!(
            ciglet_rs_wave_decode_f32(input.as_ptr(), input.len(), 0, 7, ptr::null_mut()),
            1
        );
        assert!(owner.is_null());
    }
}
#[test]
fn real_files_utf8_and_native_paths() {
    let path = std::env::temp_dir().join(format!("wave-c-api-{}.wav", std::process::id()));
    std::fs::write(&path, first_wave()).unwrap();
    let utf8 = path.to_str().unwrap().as_bytes();
    #[cfg(windows)]
    let native: Vec<u8> = {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str()
            .encode_wide()
            .flat_map(u16::to_le_bytes)
            .collect()
    };
    #[cfg(unix)]
    let native: Vec<u8> = {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    };
    let mut owner = ptr::null_mut();
    // SAFETY: Retained stable path byte vectors and independent ownership slot.
    unsafe {
        for (bytes, encoding) in [(utf8, 0), (native.as_slice(), 1)] {
            let path = CigletRsPath {
                bytes: bytes.as_ptr(),
                byte_count: bytes.len(),
                encoding,
            };
            assert_eq!(ciglet_rs_wave_read_file_f32(path, 7, &mut owner), 0);
            assert_eq!(ciglet_rs_wave_release_f32(&mut owner), 0);
        }
        let invalid = CigletRsPath {
            bytes: utf8.as_ptr(),
            byte_count: utf8.len(),
            encoding: 99,
        };
        assert_eq!(ciglet_rs_wave_read_file_f32(invalid, 7, &mut owner), 3);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn lossless_native_filename_representation() {
    let root = std::env::temp_dir();
    #[cfg(unix)]
    let filename = {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        let mut bytes = format!("wave-native-{}-", std::process::id()).into_bytes();
        bytes.extend([0xff, b'.', b'w', b'a', b'v']);
        OsString::from_vec(bytes)
    };
    #[cfg(windows)]
    let filename =
        std::ffi::OsString::from(format!("wave-native-{}-\u{1f3b5}.wav", std::process::id()));
    let path = root.join(filename);
    std::fs::write(&path, first_wave()).unwrap();
    #[cfg(unix)]
    let bytes = {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    };
    #[cfg(windows)]
    let bytes: Vec<u8> = {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str()
            .encode_wide()
            .flat_map(u16::to_le_bytes)
            .collect()
    };
    let raw = CigletRsPath {
        bytes: bytes.as_ptr(),
        byte_count: bytes.len(),
        encoding: 1,
    };
    let mut owner = ptr::null_mut();
    // SAFETY: Live native filename bytes and exclusively writable ownership slot.
    unsafe {
        assert_eq!(ciglet_rs_wave_read_file_f64(raw, 7, &mut owner), 0);
        assert_eq!(ciglet_rs_wave_release_f64(&mut owner), 0);
    }
    std::fs::remove_file(path).unwrap();
}
