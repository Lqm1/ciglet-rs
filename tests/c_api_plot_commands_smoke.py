"""Actual ctypes calls against both unchanged C plot transcript fixtures."""
import ctypes as c
from pathlib import Path
import struct
import sys

lib = c.CDLL(sys.argv[1])
P = c.c_void_p
N = c.c_size_t
U = c.c_uint32
WRITE = c.CFUNCTYPE(U, P, P, N, c.POINTER(N))
FLUSH = c.CFUNCTYPE(U, P)


class Stream(c.Structure):
    _fields_ = [("context", P), ("write", WRITE), ("flush", FLUSH)]


def function(name, args):
    fn = getattr(lib, "ciglet_rs_" + name)
    fn.argtypes = args
    fn.restype = U
    return fn


length = function("bytes_length", [P, c.POINTER(N)])
copy = function("bytes_copy", [P, N, P, N])
release = function("bytes_release", [c.POINTER(P)])
configuration = function("plot_configuration_bytes", [P, N, c.POINTER(P)])
configure_stream = function("plot_configuration_stream", [Stream, P, N])


def consume(owner):
    size = N()
    assert length(owner, c.byref(size)) == 0
    buffer = (c.c_ubyte * size.value)()
    assert copy(owner, 0, buffer, size) == 0
    assert release(c.byref(owner)) == 0 and owner.value is None
    return bytes(buffer)


class Sink:
    def __init__(self, failure=0):
        self.bytes = bytearray()
        self.writes = self.flushes = 0
        self.failure = failure
        self.write_callback = WRITE(self.write)
        self.flush_callback = FLUSH(self.flush)
        self.stream = Stream(None, self.write_callback, self.flush_callback)

    def write(self, context, data, count, written):
        self.writes += 1
        if self.failure == 1 or (self.failure == 4 and len(self.bytes) >= 8):
            return 9
        if self.failure in (2, 3):
            written[0] = count + 1 if self.failure == 2 else 0
            return 0
        self.bytes.extend(c.string_at(data, 1))
        written[0] = 1
        return 0

    def flush(self, context):
        self.flushes += 1
        return 9 if self.failure == 5 else 0


for suffix, scalar, fmt, maximum in [
    ("f32", c.c_float, "f", float.fromhex("0x1.fffffep+127")),
    ("f64", c.c_double, "d", sys.float_info.max),
]:
    tiny = struct.unpack("<" + fmt, (1).to_bytes(c.sizeof(scalar), "little"))[0]
    y = (scalar * 12)(0, -0.0, 1.23456789, -0.0000005, 1e20, 1e-20,
                      maximum, tiny, float("nan"), -float("nan"), float("inf"), -float("inf"))
    x = (scalar * 12)(*(i * 0.25 - 1 for i in range(12)))
    line = function("plot_line_bytes_" + suffix, [P, N, P, N, c.c_uint8, c.POINTER(P)])
    line_stream = function("plot_line_stream_" + suffix, [Stream, P, N, P, N, c.c_uint8])
    image = function("plot_image_bytes_" + suffix, [P, c.POINTER(P)])
    image_stream = function("plot_image_stream_" + suffix, [Stream, P])
    array = function("array_from_slice_" + suffix, [N, N, P, N, c.POINTER(P)])
    rows = function("array_from_rows_" + suffix, [P, P, N, c.POINTER(P)])
    array_release = function("array_release_" + suffix, [c.POINTER(P)])
    sink, memory = Sink(), bytearray()
    owner = P()
    for i, color in enumerate(b"brgkcy?"):
        xp, xc = (x, 13) if i % 2 else (None, 0)
        assert line(xp, xc, y, 12, color, c.byref(owner)) == 0
        memory.extend(consume(owner))
        assert line_stream(sink.stream, xp, xc, y, 12, color) == 0
    assert line(None, 0, None, 0, ord("b"), c.byref(owner)) == 0
    memory.extend(consume(owner))
    assert line_stream(sink.stream, None, 0, None, 0, ord("b")) == 0
    for r, columns in [(2, 6), (0, 3), (2, 0)]:
        matrix = P()
        if columns == 6:
            pointers = (P * 2)(c.addressof(y), c.addressof(y) + 6 * c.sizeof(scalar))
            counts = (N * 2)(6, 6)
            assert rows(pointers, counts, 2, c.byref(matrix)) == 0
        else:
            assert array(r, columns, None, 0, c.byref(matrix)) == 0
        assert image(matrix, c.byref(owner)) == 0
        memory.extend(consume(owner))
        assert image_stream(sink.stream, matrix) == 0
        assert array_release(c.byref(matrix)) == 0 and matrix.value is None
    expected = (Path(__file__).parent / "fixtures" / ("plotting-float.txt" if suffix == "f32" else "plotting-double.txt")).read_bytes()
    assert memory == expected == sink.bytes
    assert sink.writes == len(expected) and sink.flushes == 11

data = b"set title 'custom'\n\0\xff"
sink = Sink()
assert configuration(data, len(data), c.byref(owner)) == 0
assert consume(owner) == data
assert configure_stream(sink.stream, data, len(data)) == 0
assert sink.bytes == data
assert configure_stream(sink.stream, None, 0) == 0 and sink.flushes == 2
assert configuration(None, 0, c.byref(owner)) == 0 and consume(owner) == b""
for failure in range(1, 6):
    sink = Sink(failure)
    assert configure_stream(sink.stream, b"0123456789", 10) == 3
    assert sink.flushes == int(failure == 5)
    if failure <= 3:
        assert not sink.bytes
    if failure == 4:
        assert len(sink.bytes) == 8
sink = Sink()
assert line_stream(sink.stream, y, 1, y, 2, ord("r")) == 2
missing = Stream(None, sink.write_callback, FLUSH())
assert configure_stream(missing, None, 0) == 1
assert sink.writes == sink.flushes == 0
