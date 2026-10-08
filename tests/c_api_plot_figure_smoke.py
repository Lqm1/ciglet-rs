"""Call all native Figure exports with ctypes and verify actual Gnuplot SVGs."""
import ctypes as c
from pathlib import Path
import sys

lib = c.CDLL(sys.argv[1])
program = sys.argv[2].encode("utf-8")
directory = Path(sys.argv[3])
P = c.c_void_p
N = c.c_size_t
U = c.c_uint32


class OsPath(c.Structure):
    _fields_ = [("bytes", P), ("byte_count", N), ("encoding", U)]


class Environment(c.Structure):
    _fields_ = [("key", OsPath), ("value", c.POINTER(OsPath))]


class Options(c.Structure):
    _fields_ = [("program", OsPath), ("arguments", c.POINTER(OsPath)),
                ("argument_count", N), ("environment", c.POINTER(Environment)),
                ("environment_count", N), ("clear_environment", U),
                ("directory", c.POINTER(OsPath)), ("stdout_mode", U),
                ("stdout_file", c.POINTER(OsPath)), ("stderr_mode", U),
                ("stderr_file", c.POINTER(OsPath))]


class Status(c.Structure):
    _fields_ = [("success", U), ("has_code", U), ("code", c.c_int32),
                ("raw_status", U), ("raw_format", U)]


def function(name, arguments):
    fn = getattr(lib, "ciglet_rs_" + name)
    fn.argtypes, fn.restype = arguments, U
    return fn


def path(data):
    return OsPath(c.cast(c.c_char_p(data), P), len(data), 0)


open_default = function("figure_open", [c.POINTER(P)])
open_with = function("figure_open_with", [OsPath, c.POINTER(P)])
spawn = function("figure_spawn", [Options, c.POINTER(P)])
process_id = function("figure_id", [P, c.POINTER(U)])
configuration = function("figure_configuration", [P, P, N])
close = function("figure_close", [c.POINTER(P), c.POINTER(Status)])
close_output = function("figure_close_output", [c.POINTER(P), c.POINTER(Status), c.POINTER(P), c.POINTER(P)])
release = function("figure_release", [c.POINTER(P)])
byte_length = function("bytes_length", [P, c.POINTER(N)])
byte_release = function("bytes_release", [c.POINTER(P)])
for mode in range(3):
    figure = P()
    if mode == 0:
        assert open_default(c.byref(figure)) == 0
    elif mode == 1:
        assert open_with(path(program), c.byref(figure)) == 0
    else:
        value = path(b"Python caller")
        environment = Environment(path(b"CIGLET_ABI_CALLER"), c.pointer(value))
        cwd = path(str(directory).encode("utf-8"))
        options = Options(program=path(program), environment=c.pointer(environment),
                          environment_count=1, directory=c.pointer(cwd), stdout_mode=2, stderr_mode=2)
        assert spawn(options, c.byref(figure)) == 0
    identifier = U()
    assert process_id(figure, c.byref(identifier)) == 0 and identifier.value > 0
    original = figure.value
    assert close(c.byref(figure), None) == 1 and figure.value == original
    assert configuration(figure, None, 1) == 1
    assert configuration(figure, None, 0) == 0
    commands = b"set terminal svg size 320,240\n"
    assert configuration(figure, commands, len(commands)) == 0
    files = []
    for suffix, scalar in [("f32", c.c_float), ("f64", c.c_double)]:
        line = function("figure_line_" + suffix, [P, P, N, P, N, c.c_uint8])
        image = function("figure_image_" + suffix, [P, P])
        array = function("array_from_slice_" + suffix, [N, N, P, N, c.POINTER(P)])
        array_release = function("array_release_" + suffix, [c.POINTER(P)])
        for kind in ["line", "image"]:
            filename = directory / f"python-figure-{mode}-{kind}-{suffix}.svg"
            escaped = str(filename).replace("\\", "/").replace('"', '\\"')
            commands = f'set output "{escaped}"\n'.encode("utf-8")
            assert configuration(figure, commands, len(commands)) == 0
            if kind == "line":
                values = (scalar * 4)(0, 1, 0, -1)
                assert line(figure, None, 0, values, 4, ord("r")) == 0
            else:
                values = (scalar * 6)(0, 1, 2, 3, 4, 5)
                owner = P()
                assert array(2, 3, values, 6, c.byref(owner)) == 0
                assert image(figure, owner) == 0
                assert array_release(c.byref(owner)) == 0 and owner.value is None
            files.append((kind, filename))
    assert configuration(figure, b"unset output\n", 13) == 0
    state = Status()
    if mode < 2:
        assert close(c.byref(figure), c.byref(state)) == 0
    else:
        out, err = P(), P()
        assert close_output(c.byref(figure), c.byref(state), c.byref(out), c.byref(err)) == 0
        for owner in [out, err]:
            length = N(99)
            assert byte_length(owner, c.byref(length)) == 0 and length.value == 0
            assert byte_release(c.byref(owner)) == 0 and owner.value is None
    assert figure.value is None
    assert (state.success, state.has_code, state.code, state.raw_status) == (1, 1, 0, 0)
    assert state.raw_format == int(sys.platform == "win32")
    for kind, filename in files:
        content = filename.read_text(encoding="utf-8")
        assert "<svg" in content
        if kind == "line":
            assert "rgb(255,   0,   0)" in content
        else:
            group = content.split('id="gnuplot_plot_1"')[1].split("</g>")[0]
            assert "<image " in group or group.count("<polygon ") == 6
        filename.unlink()
figure = P()
assert open_with(path(program), c.byref(figure)) == 0
assert configuration(figure, b"set terminal unknown\n", 21) == 0
assert release(c.byref(figure)) == 0 and figure.value is None
assert release(c.byref(figure)) == 0
assert open_with(path(b"missing-program-abi-figure"), c.byref(figure)) == 3 and figure.value is None
