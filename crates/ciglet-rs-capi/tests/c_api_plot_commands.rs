use ciglet_rs_capi::*;
use std::{ffi::c_void, ptr};

#[derive(Default)]
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
    // SAFETY: The test supplies exclusive context, readable bytes and writable count.
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
        let count = count.min(1);
        sink.bytes
            .extend_from_slice(std::slice::from_raw_parts(bytes, count));
        *written = count;
    }
    0
}
unsafe extern "C" fn flush(context: *mut c_void) -> u32 {
    // SAFETY: The test retains its exclusive context throughout the callback.
    unsafe {
        let sink = &mut *context.cast::<Sink>();
        sink.flushes += 1;
        if sink.failure == 5 {
            return 9;
        }
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
unsafe fn consume(mut owner: *mut CigletRsBytes) -> Vec<u8> {
    let mut length = 0;
    // SAFETY: Live owner and independent writable buffers; release once after copying.
    unsafe {
        assert_eq!(ciglet_rs_bytes_length(owner, &mut length), 0);
        let mut bytes = vec![0; length];
        assert_eq!(
            ciglet_rs_bytes_copy(owner, 0, bytes.as_mut_ptr(), length),
            0
        );
        assert_eq!(ciglet_rs_bytes_release(&mut owner), 0);
        assert!(owner.is_null());
        bytes
    }
}

macro_rules! corpus {
    ($name:ident,$t:ty,$fixture:expr,$line:ident,$line_stream:ident,$image:ident,$image_stream:ident,$array:ident,$rows:ident,$release:ident) => {
        #[test]
        fn $name() {
            let y: Vec<$t> = [
                0.0,
                -0.0,
                1.23456789,
                -0.0000005,
                1e20,
                1e-20,
                <$t>::MAX as f64,
                <$t>::from_bits(1) as f64,
                f64::NAN,
                -f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ]
            .into_iter()
            .map(|v| v as $t)
            .collect();
            let x: Vec<$t> = (0..12).map(|i| (i as f64 * 0.25 - 1.0) as $t).collect();
            let mut memory = Vec::new();
            let mut sink = Sink::default();
            // SAFETY: All input allocations and output slots remain valid; callbacks only mutate the sink.
            unsafe {
                for (i, color) in b"brgkcy?".iter().copied().enumerate() {
                    let (xp, xc) = if i % 2 == 0 {
                        (ptr::null(), 0)
                    } else {
                        (x.as_ptr(), x.len() + 1)
                    };
                    let mut owner = ptr::null_mut();
                    assert_eq!($line(xp, xc, y.as_ptr(), y.len(), color, &mut owner), 0);
                    memory.extend(consume(owner));
                    assert_eq!(
                        $line_stream(stream(&mut sink), xp, xc, y.as_ptr(), y.len(), color),
                        0
                    );
                }
                let mut owner = ptr::null_mut();
                assert_eq!($line(ptr::null(), 0, ptr::null(), 0, b'b', &mut owner), 0);
                memory.extend(consume(owner));
                assert_eq!(
                    $line_stream(stream(&mut sink), ptr::null(), 0, ptr::null(), 0, b'b'),
                    0
                );
                for (r, c) in [(2, 6), (0, 3), (2, 0)] {
                    let mut array = ptr::null_mut();
                    if c == 6 {
                        let table = [y.as_ptr(), y.as_ptr().add(6)];
                        assert_eq!($rows(table.as_ptr(), [6, 6].as_ptr(), 2, &mut array), 0);
                    } else {
                        assert_eq!($array(r, c, ptr::null(), 0, &mut array), 0);
                    }
                    assert_eq!($image(array, &mut owner), 0);
                    memory.extend(consume(owner));
                    assert_eq!($image_stream(stream(&mut sink), array), 0);
                    assert_eq!($release(&mut array), 0);
                }
            }
            assert_eq!(memory, $fixture);
            assert_eq!(sink.bytes, $fixture);
            assert_eq!(sink.flushes, 11);
            assert_eq!(sink.writes, sink.bytes.len());
        }
    };
}
corpus!(
    original_float_transcript,
    f32,
    include_bytes!("../../../tests/fixtures/plotting-float.txt"),
    ciglet_rs_plot_line_bytes_f32,
    ciglet_rs_plot_line_stream_f32,
    ciglet_rs_plot_image_bytes_f32,
    ciglet_rs_plot_image_stream_f32,
    ciglet_rs_array_from_slice_f32,
    ciglet_rs_array_from_rows_f32,
    ciglet_rs_array_release_f32
);
corpus!(
    original_double_transcript,
    f64,
    include_bytes!("../../../tests/fixtures/plotting-double.txt"),
    ciglet_rs_plot_line_bytes_f64,
    ciglet_rs_plot_line_stream_f64,
    ciglet_rs_plot_image_bytes_f64,
    ciglet_rs_plot_image_stream_f64,
    ciglet_rs_array_from_slice_f64,
    ciglet_rs_array_from_rows_f64,
    ciglet_rs_array_release_f64
);

#[test]
fn arbitrary_configuration_and_empty_flush() {
    let input = b"set title 'custom'\n\0\xff";
    let mut sink = Sink::default();
    let mut owner = ptr::null_mut();
    // SAFETY: Independent storage, stable arbitrary bytes and synchronous callbacks.
    unsafe {
        assert_eq!(
            ciglet_rs_plot_configuration_bytes(input.as_ptr(), input.len(), &mut owner),
            0
        );
        assert_eq!(consume(owner), input);
        assert_eq!(
            ciglet_rs_plot_configuration_stream(stream(&mut sink), input.as_ptr(), input.len()),
            0
        );
        assert_eq!(sink.bytes, input);
        assert_eq!(
            ciglet_rs_plot_configuration_bytes(ptr::null(), 0, &mut owner),
            0
        );
        assert!(consume(owner).is_empty());
        let writes = sink.writes;
        assert_eq!(
            ciglet_rs_plot_configuration_stream(stream(&mut sink), ptr::null(), 0),
            0
        );
        assert_eq!(sink.writes, writes);
        assert_eq!(sink.flushes, 2);
    }
}

#[test]
fn invalid_inputs_do_not_write_or_publish() {
    let mut sink = Sink::default();
    let values = [1.0_f64, 2.0];
    let mut owner = ptr::null_mut();
    // SAFETY: Live initialized input and independent slots; invalid null/length cases are checked before use.
    unsafe {
        assert_eq!(
            ciglet_rs_plot_configuration_bytes(b"sentinel".as_ptr(), 8, &mut owner),
            0
        );
        let original = owner;
        assert_eq!(
            ciglet_rs_plot_line_bytes_f64(values.as_ptr(), 1, values.as_ptr(), 2, b'r', &mut owner),
            2
        );
        assert_eq!(
            ciglet_rs_plot_line_bytes_f64(ptr::null(), 1, values.as_ptr(), 2, b'r', &mut owner),
            2
        );
        assert_eq!(ciglet_rs_plot_image_bytes_f64(ptr::null(), &mut owner), 1);
        assert_eq!(
            ciglet_rs_plot_configuration_bytes(ptr::null(), 1, &mut owner),
            1
        );
        assert_eq!(owner, original);
        assert_eq!(consume(owner), b"sentinel");
        assert_eq!(
            ciglet_rs_plot_line_stream_f64(
                stream(&mut sink),
                values.as_ptr(),
                1,
                values.as_ptr(),
                2,
                b'r'
            ),
            2
        );
        assert_eq!(
            ciglet_rs_plot_image_stream_f64(stream(&mut sink), ptr::null()),
            1
        );
        assert_eq!(
            ciglet_rs_plot_configuration_stream(stream(&mut sink), ptr::null(), 1),
            1
        );
        for missing_write in [false, true] {
            let mut descriptor = stream(&mut sink);
            if missing_write {
                descriptor.write = None;
            } else {
                descriptor.flush = None;
            }
            assert_eq!(
                ciglet_rs_plot_line_stream_f64(
                    descriptor,
                    ptr::null(),
                    0,
                    values.as_ptr(),
                    2,
                    b'r'
                ),
                1
            );
            assert_eq!(
                ciglet_rs_plot_configuration_stream(descriptor, ptr::null(), 0),
                1
            );
        }
    }
    assert_eq!(sink.writes, 0);
    assert_eq!(sink.flushes, 0);
}

#[test]
fn partial_write_and_flush_failures_are_reported() {
    // SAFETY: Live independent sink and arrays; each array is released exactly once.
    unsafe {
        for failure in 1..=5 {
            for operation in 0..3 {
                let mut sink = Sink {
                    failure,
                    ..Sink::default()
                };
                let status = match operation {
                    0 => ciglet_rs_plot_configuration_stream(
                        stream(&mut sink),
                        b"0123456789".as_ptr(),
                        10,
                    ),
                    1 => ciglet_rs_plot_line_stream_f32(
                        stream(&mut sink),
                        ptr::null(),
                        0,
                        [1.0].as_ptr(),
                        1,
                        b'b',
                    ),
                    _ => {
                        let mut array = ptr::null_mut();
                        assert_eq!(
                            ciglet_rs_array_from_slice_f32(1, 1, [1.0].as_ptr(), 1, &mut array),
                            0
                        );
                        let status = ciglet_rs_plot_image_stream_f32(stream(&mut sink), array);
                        assert_eq!(ciglet_rs_array_release_f32(&mut array), 0);
                        status
                    }
                };
                assert_eq!(status, 3);
                if failure <= 3 {
                    assert!(sink.bytes.is_empty());
                }
                if failure == 4 {
                    assert_eq!(sink.bytes.len(), 8);
                }
                assert_eq!(sink.flushes, usize::from(failure == 5));
            }
        }
    }
}
