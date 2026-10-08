"""Exercise the real shared library through Python's standard-library ctypes."""

import ctypes
import pathlib
import struct
import sys


def check_owned_arrays(library, suffix, scalar):
    pointer = ctypes.POINTER(scalar)
    size = ctypes.c_size_t
    handle_type = ctypes.c_void_p
    slot = ctypes.POINTER(handle_type)
    signatures = {
        "filled": [size, size, scalar, slot],
        "from_slice": [size, size, pointer, size, slot],
        "from_rows": [ctypes.POINTER(pointer), ctypes.POINTER(size), size, slot],
        "clone": [handle_type, slot],
        "transposed": [handle_type, slot],
        "release": [slot],
        "shape": [handle_type, ctypes.POINTER(size), size],
        "copy_flat": [handle_type, pointer, size],
        "copy_row": [handle_type, size, pointer, size],
        "get": [handle_type, size, size, pointer],
        "set": [handle_type, size, size, scalar],
        "write_flat": [handle_type, pointer, size],
        "write_row": [handle_type, size, pointer, size],
        "reshape": [handle_type, size, size],
        "transpose": [handle_type],
    }
    functions = {}
    for operation, arguments in signatures.items():
        function = getattr(library, f"ciglet_rs_array_{operation}_{suffix}")
        function.argtypes = arguments
        function.restype = ctypes.c_uint32
        functions[operation] = function
    objects = [handle_type() for _ in range(5)]
    handle, duplicate, transposed, row_array, filled = objects
    values = (scalar * 6)(1, 2, 3, 4, 5, 6)
    dimensions = (size * 3)()
    result = scalar()
    output = (scalar * 6)()
    try:
        assert functions["from_slice"](2, 3, values, 6, ctypes.byref(handle)) == 0
        assert functions["shape"](handle, dimensions, 3) == 0
        assert list(dimensions) == [2, 3, 6]
        values[0] = -100
        assert functions["get"](handle, 0, 0, ctypes.byref(result)) == 0
        assert result.value == 1
        assert functions["clone"](handle, ctypes.byref(duplicate)) == 0
        assert handle.value != duplicate.value
        assert functions["set"](duplicate, 0, 0, -1) == 0
        assert functions["get"](handle, 0, 0, ctypes.byref(result)) == 0
        assert result.value == 1
        assert functions["transposed"](handle, ctypes.byref(transposed)) == 0
        assert functions["copy_flat"](transposed, output, 6) == 0
        assert list(output) == [1, 4, 2, 5, 3, 6]
        assert functions["reshape"](handle, 3, 2) == 0
        assert functions["transpose"](handle) == 0
        assert functions["copy_flat"](handle, output, 6) == 0
        assert list(output) == [1, 3, 5, 2, 4, 6]
        row = (scalar * 3)(-2, -3, -4)
        copied_row = (scalar * 3)()
        assert functions["write_row"](handle, 0, row, 3) == 0
        assert functions["copy_row"](handle, 0, copied_row, 3) == 0
        assert list(copied_row) == [-2, -3, -4]
        values[0] = 1
        assert functions["write_flat"](handle, values, 6) == 0
        assert functions["copy_flat"](handle, output, 6) == 0
        assert list(output) == [1, 2, 3, 4, 5, 6]
        assert functions["reshape"](handle, 4, 2) == 3
        assert functions["shape"](handle, dimensions, 3) == 0
        assert list(dimensions) == [2, 3, 6]
        first = (scalar * 3)(1, 2, 3)
        second = (scalar * 3)(4, 5, 6)
        rows = (pointer * 2)(first, second)
        counts = (size * 2)(3, 2)
        assert functions["from_rows"](rows, counts, 2, ctypes.byref(row_array)) == 3
        assert row_array.value is None
        counts[1] = 3
        assert functions["from_rows"](rows, counts, 2, ctypes.byref(row_array)) == 0
        assert functions["copy_flat"](row_array, output, 6) == 0
        assert list(output) == [1, 2, 3, 4, 5, 6]
        assert functions["filled"](2, 2, -0.0, ctypes.byref(filled)) == 0
        zeros = (scalar * 4)()
        assert functions["copy_flat"](filled, zeros, 4) == 0
        encoding = "f" if suffix == "f32" else "d"
        assert all(struct.pack(encoding, value) == struct.pack(encoding, -0.0) for value in zeros)
        address = filled.value
        maximum_size = (1 << (ctypes.sizeof(size) * 8)) - 1
        assert functions["filled"](maximum_size, 2, 0, ctypes.byref(filled)) == 2
        assert filled.value == address
        assert functions["release"](ctypes.byref(row_array)) == 0
        assert row_array.value is None
        assert functions["from_rows"](None, None, 0, ctypes.byref(row_array)) == 0
        assert functions["shape"](row_array, dimensions, 3) == 0
        assert list(dimensions) == [0, 0, 0]
    finally:
        for object_handle in objects:
            assert functions["release"](ctypes.byref(object_handle)) == 0
            assert object_handle.value is None
            assert functions["release"](ctypes.byref(object_handle)) == 0


def main():
    library = ctypes.CDLL(str(pathlib.Path(sys.argv[1]).resolve()))
    library.ciglet_rs_abi_version.argtypes = []
    library.ciglet_rs_abi_version.restype = ctypes.c_uint32
    assert library.ciglet_rs_abi_version() == 1
    for suffix, scalar, encoding in (
        ("f32", ctypes.c_float, "f"),
        ("f64", ctypes.c_double, "d"),
    ):
        for operation, expected in (("minimum", 2.0), ("maximum", 3.0)):
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [scalar, scalar, ctypes.POINTER(scalar)]
            function.restype = ctypes.c_uint32
            output = scalar(9.0)
            assert function(3.0, 2.0, ctypes.byref(output)) == 0
            assert output.value == expected
            assert function(0.0, -0.0, ctypes.byref(output)) == 0
            assert struct.pack(encoding, output.value) == struct.pack(encoding, -0.0)
            assert function(1.0, 2.0, None) == 1
        for operation, expected in (
            ("difference", [1.0, 2.0, 3.0]),
            ("cumulative_sum", [1.0, 4.0, 10.0]),
            ("reverse", [6.0, 3.0, 1.0]),
        ):
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                                 ctypes.POINTER(scalar), ctypes.c_size_t]
            function.restype = ctypes.c_uint32
            values = (scalar * 3)(1.0, 3.0, 6.0)
            assert function(values, 3, values, 3) == 0
            assert list(values) == expected
            assert function(None, 0, None, 0) == 0
            assert function(values, 3, values, 2) == 2
            assert list(values) == expected
        reductions = (("sum", 10.0), ("sum_squares", 46.0), ("mean", 10.0 / 3.0),
                      ("variance", 38.0 / 9.0), ("median", 3.0),
                      ("array_minimum", 1.0), ("array_maximum", 6.0))
        for operation, expected in reductions:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t, ctypes.POINTER(scalar)]
            function.restype = ctypes.c_uint32
            values = (scalar * 3)(1.0, 3.0, 6.0)
            output = scalar(-71.0)
            assert function(values, 3, ctypes.byref(output)) == 0
            assert abs(output.value - expected) <= 1e-6 * max(1.0, abs(expected))
        for operation, expected in (("mean", 10.0 / 3.0), ("variance", 38.0 / 9.0)):
            function = getattr(library, f"ciglet_rs_{operation}_with_{suffix}")
            function.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                                 ctypes.c_uint32, ctypes.POINTER(scalar)]
            function.restype = ctypes.c_uint32
            for policy in (0, 1):
                output = scalar(-71.0)
                assert function(values, 3, policy, ctypes.byref(output)) == 0
                assert abs(output.value - expected) <= 2e-6 * max(1.0, abs(expected))
            output = scalar(-71.0)
            assert function(values, 3, 2, ctypes.byref(output)) == 3
            assert output.value == -71.0
        left = (scalar * 3)(1.0, 2.0, 3.0)
        right = (scalar * 3)(2.0, 4.0, 6.0)
        for operation, expected in (("dot", 28.0), ("covariance", 4.0 / 3.0), ("correlation", 1.0)):
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                                 ctypes.POINTER(scalar), ctypes.c_size_t, ctypes.POINTER(scalar)]
            function.restype = ctypes.c_uint32
            output = scalar(-71.0)
            assert function(left, 3, right, 3, ctypes.byref(output)) == 0
            assert abs(output.value - expected) <= 1e-6 * max(1.0, abs(expected))
            output = scalar(-71.0)
            assert function(left, 3, right, 2, ctypes.byref(output)) == 2
            assert output.value == -71.0
        for operation, expected in (("covariance", 4.0 / 3.0), ("correlation", 1.0)):
            function = getattr(library, f"ciglet_rs_{operation}_with_{suffix}")
            function.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                                 ctypes.POINTER(scalar), ctypes.c_size_t,
                                 ctypes.c_uint32, ctypes.POINTER(scalar)]
            function.restype = ctypes.c_uint32
            for policy in (0, 1):
                output = scalar(-71.0)
                assert function(left, 3, right, 3, policy, ctypes.byref(output)) == 0
                assert abs(output.value - expected) <= 1e-6 * max(1.0, abs(expected))
            output = scalar(-71.0)
            assert function(left, 3, right, 3, 2, ctypes.byref(output)) == 3
            assert output.value == -71.0
        cross = getattr(library, f"ciglet_rs_cross_correlation_{suffix}")
        cross.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                         ctypes.POINTER(scalar), ctypes.c_size_t,
                         ctypes.POINTER(scalar), ctypes.c_size_t]
        cross.restype = ctypes.c_uint32
        output = (scalar * 4)(-71.0, -71.0, -71.0, -71.0)
        assert cross(left, 3, right, 3, output, 4) == 0
        assert list(output) == [28.0, 16.0, 6.0, 0.0]
        assert cross(None, 0, None, 0, output, 4) == 0
        assert list(output) == [0.0] * 4
        sorting = getattr(library, f"ciglet_rs_sort_{suffix}")
        sorting.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                            ctypes.POINTER(scalar), ctypes.c_size_t,
                            ctypes.POINTER(ctypes.c_size_t), ctypes.c_size_t]
        sorting.restype = ctypes.c_uint32
        values = (scalar * 5)(3.0, -0.0, 0.0, 3.0, 1.0)
        positions = (ctypes.c_size_t * 5)()
        assert sorting(values, 5, values, 5, positions, 5) == 0
        assert list(positions) == [1, 2, 4, 0, 3]
        assert list(values) == [-0.0, 0.0, 1.0, 3.0, 3.0]
        assert struct.pack(encoding, values[0]) == struct.pack(encoding, -0.0)
        assert struct.pack(encoding, values[1]) == struct.pack(encoding, 0.0)
        assert sorting(None, 0, None, 0, None, 0) == 0
        selection = getattr(library, f"ciglet_rs_select_nth_{suffix}")
        selection.argtypes = [ctypes.POINTER(scalar), ctypes.c_size_t,
                              ctypes.c_size_t, ctypes.POINTER(scalar)]
        selection.restype = ctypes.c_uint32
        values = (scalar * 5)(3.0, 1.0, 5.0, 2.0, 4.0)
        selected = scalar(-71.0)
        assert selection(values, 5, 2, ctypes.byref(selected)) == 0
        assert selected.value == 3.0 and values[2] == 3.0
        assert all(value <= 3.0 for value in values[:2])
        assert all(value >= 3.0 for value in values[3:])
        before = list(values)
        selected = scalar(-71.0)
        assert selection(values, 5, 5, ctypes.byref(selected)) == 3
        assert selected.value == -71.0 and list(values) == before
        grid = getattr(library, f"ciglet_rs_linear_grid_{suffix}")
        grid.argtypes = [scalar, scalar, ctypes.POINTER(scalar), ctypes.c_size_t]
        grid.restype = ctypes.c_uint32
        values = (scalar * 3)()
        assert grid(-1.0, 1.0, values, 3) == 0
        assert list(values) == [-1.0, 0.0, 1.0]
        check_owned_arrays(library, suffix, scalar)
        pointer = ctypes.POINTER(scalar)
        size = ctypes.c_size_t
        real = (scalar * 4)(1, 2, 3, 4)
        imaginary = (scalar * 4)()
        fft = getattr(library, f"ciglet_rs_fft_{suffix}")
        fft.argtypes = [size, pointer, pointer, ctypes.c_uint32, pointer, pointer]
        fft.restype = ctypes.c_uint32
        assert fft(4, real, None, 0, real, imaginary) == 0
        assert list(real) == [10, -2, -2, -2]
        assert list(imaginary) == [0, 2, 0, -2]
        assert fft(4, real, imaginary, 1, real, imaginary) == 0
        assert list(real) == [1, 2, 3, 4]
        assert fft(4, real, None, 2, real, imaginary) == 3
        assert fft(4, real, None, 0, real, real) == 2
        direct = getattr(library, f"ciglet_rs_inverse_dft_{suffix}")
        direct.argtypes = [size, pointer, pointer, pointer, pointer]
        direct.restype = ctypes.c_uint32
        impulse = (scalar * 3)(3, 0, 0)
        output = (scalar * 3)()
        assert direct(3, impulse, None, output, None) == 0
        assert list(output) == [1, 1, 1]
        shift = getattr(library, f"ciglet_rs_fft_shift_{suffix}")
        shift.argtypes = [pointer, size, pointer, size]
        shift.restype = ctypes.c_uint32
        odd = (scalar * 5)(1, 2, 3, 4, 5)
        assert shift(odd, 5, odd, 5) == 0
        assert list(odd) == [4, 5, 1, 2, 3]
        plan_type = ctypes.c_void_p
        slot = ctypes.POINTER(plan_type)
        functions = {}
        for operation, arguments in {
            "new": [size, slot], "length": [plan_type, ctypes.POINTER(size)],
            "transform": [plan_type, pointer, pointer, size, ctypes.c_uint32, pointer, pointer],
            "release": [slot],
        }.items():
            function = getattr(library, f"ciglet_rs_fft_plan_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            functions[operation] = function
        plan = plan_type()
        try:
            assert functions["new"](4, ctypes.byref(plan)) == 0
            count = size()
            assert functions["length"](plan, ctypes.byref(count)) == 0 and count.value == 4
            unit = (scalar * 4)(1, 0, 0, 0)
            for _ in range(3):
                assert functions["transform"](plan, unit, None, 4, 0, real, imaginary) == 0
                assert list(real) == [1, 1, 1, 1] and list(imaginary) == [0, 0, 0, 0]
            assert functions["transform"](plan, real, imaginary, 4, 1, real, imaginary) == 0
            assert list(real) == [1, 0, 0, 0]
            assert functions["transform"](plan, unit, None, 3, 0, real, imaginary) == 2
        finally:
            assert functions["release"](ctypes.byref(plan)) == 0 and plan.value is None
            assert functions["release"](ctypes.byref(plan)) == 0
        phase_functions = {}
        for operation in ["magnitude", "argument", "polar_real", "polar_imaginary"]:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [pointer, size, pointer, size, pointer, size]
            function.restype = ctypes.c_uint32
            phase_functions[operation] = function
        real = (scalar * 2)(3, 0)
        imaginary = (scalar * 2)(4, 0)
        out = (scalar * 2)(71, 71)
        assert phase_functions["magnitude"](real, 2, imaginary, 2, real, 2) == 0
        assert list(real) == [5, 0]
        real[0] = 1
        assert phase_functions["argument"](real, 2, imaginary, 2, out, 2) == 0
        assert 1 < out[0] < 2
        angles = (scalar * 2)(0, 0)
        assert phase_functions["polar_real"](real, 2, angles, 2, out, 2) == 0
        assert abs(out[0] - 1) < 0.001
        assert phase_functions["polar_imaginary"](real, 2, angles, 2, out, 2) == 0
        assert abs(out[0]) < 0.001
        for operation, arguments in {
            "phase_wrap": [scalar, pointer],
            "phase_difference": [scalar, scalar, pointer],
            "phase_unwrap": [pointer, size, pointer, size],
            "complete_spectrum": [pointer, size, ctypes.c_uint32, ctypes.c_uint32, pointer, size],
            "dct": [pointer, size, pointer, size],
        }.items():
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            phase_functions[operation] = function
        assert phase_functions["phase_wrap"](7, out) == 0 and 0 < out[0] < 1
        assert phase_functions["phase_difference"](1, 0, out) == 0 and abs(out[0] - 1) < 0.001
        jump = (scalar * 2)(3, -3)
        assert phase_functions["phase_unwrap"](jump, 2, jump, 2) == 0 and 3 < jump[1] < 4
        spectrum = (scalar * 6)(1, 2, 3, 4, 99, 99)
        assert phase_functions["complete_spectrum"](spectrum, 6, 1, 0, spectrum, 6) == 0
        assert list(spectrum) == [1, 2, 3, 4, -3, -2]
        assert phase_functions["complete_spectrum"](spectrum, 6, 1, 1, spectrum, 6) == 0
        assert spectrum[3] == 3
        assert phase_functions["complete_spectrum"](spectrum, 6, 2, 0, spectrum, 6) == 3
        singleton = (scalar * 1)(2)
        assert phase_functions["dct"](singleton, 1, singleton, 1) == 0
        assert abs(singleton[0] - 2) < 0.001
        assert phase_functions["dct"](None, 0, None, 0) == 3
        assert phase_functions["phase_unwrap"](None, 0, None, 0) == 0
        out[0] = out[1] = 71
        real[1] = float("nan")
        assert phase_functions["magnitude"](real, 2, imaginary, 2, out, 2) == 3
        assert list(out) == [71, 71]
        assert phase_functions["magnitude"](real, 2, imaginary, 1, out, 2) == 2
    sequence = library.ciglet_rs_integer_sequence
    sequence.argtypes = [ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_int32), ctypes.c_size_t]
    sequence.restype = ctypes.c_uint32
    integers = (ctypes.c_int32 * 3)(91, 91, 91)
    assert sequence(2147483647, 1, integers, 3) == 3
    assert list(integers) == [91, 91, 91]
    assert sequence(-2, 3, integers, 3) == 0
    assert list(integers) == [-2, 1, 4]
    print("C ABI Python smoke test passed")


if __name__ == "__main__":
    main()
