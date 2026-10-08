"""Exercise the real shared library through Python's standard-library ctypes."""

import ctypes
import math
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
        window = getattr(library, f"ciglet_rs_window_{suffix}")
        window.argtypes = [ctypes.c_uint32, ctypes.c_uint32, pointer, size]
        window.restype = ctypes.c_uint32
        weights = (scalar * 8)()
        for kind in range(7):
            for mode in range(2):
                assert window(kind, mode, weights, 8) == 0
                assert all(math.isfinite(value) for value in weights)
                assert window(kind, mode, None, 0) == 0
                assert window(kind, mode, weights, 1) == 0 and weights[0] == 1
        assert window(3, 0, weights, 8) == 0
        accurate = list(weights)
        assert window(3, 1, weights, 8) == 0 and list(weights) == accurate
        weights[0] = 71
        assert window(7, 0, weights, 8) == 3 and weights[0] == 71
        assert window(0, 2, weights, 8) == 3 and weights[0] == 71
        for callback_suffix, callback_scalar in [("f32", ctypes.c_float), ("f64", ctypes.c_double)]:
            callback_type = ctypes.CFUNCTYPE(
                ctypes.c_uint32, ctypes.c_double, ctypes.c_void_p, ctypes.POINTER(callback_scalar)
            )
            state = (ctypes.c_size_t * 3)(0, 0, 0)
            calls = []

            def trig(angle, context, result):
                try:
                    current = ctypes.cast(context, ctypes.POINTER(ctypes.c_size_t))
                    current[0] += 1
                    calls.append(angle)
                    if current[0] == current[1]:
                        return 9
                    result[0] = float("nan") if current[2] else 0
                    return 0
                except Exception:
                    return 9

            callback = callback_type(trig)
            null_callback = callback_type()
            custom = getattr(library, f"ciglet_rs_window_custom_{suffix}_{callback_suffix}")
            custom.argtypes = [ctypes.c_uint32, callback_type, ctypes.c_void_p, pointer, size]
            custom.restype = ctypes.c_uint32
            weights[0] = weights[1] = 71
            assert custom(3, callback, state, weights, 2) == 0
            assert state[0] == 4 and calls == [0, 0, math.pi, 2 * math.pi]
            assert abs(weights[0] - 0.42) < 1e-7 and weights[0] == weights[1]
            state[0], state[1] = 0, 2
            weights[0] = weights[1] = 71
            assert custom(4, callback, state, weights, 2) == 3
            assert state[0] == 2 and weights[0] == weights[1] == 71
            state[0], state[1], state[2] = 0, 0, 1
            assert custom(1, callback, state, weights, 2) == 3
            assert state[0] == 1 and weights[0] == weights[1] == 71
            assert custom(1, null_callback, None, weights, 2) == 3 and weights[0] == 71
            assert custom(0, null_callback, None, weights, 2) == 0
            assert weights[0] == weights[1] == 1
            assert custom(1, null_callback, None, None, 0) == 0
            assert custom(1, null_callback, None, weights, 1) == 0 and weights[0] == 1
        czt = getattr(library, f"ciglet_rs_czt_{suffix}")
        czt.argtypes = [size, scalar, ctypes.c_uint32, pointer, pointer, pointer, pointer]
        czt.restype = ctypes.c_uint32
        signal = (scalar * 3)(3, 0, 0)
        real = (scalar * 3)(71, 71, 71)
        imaginary = (scalar * 3)()
        tolerance = 1e-5 if suffix == "f32" else 1e-12
        assert czt(3, 0, 0, signal, None, real, imaginary) == 0
        assert all(abs(value - 3) < tolerance for value in real)
        assert all(abs(value) < tolerance for value in imaginary)
        assert czt(3, 0, 1, signal, None, real, None) == 0
        assert all(abs(value - 1) < tolerance for value in real)
        assert czt(3, 0, 0, signal, None, real, real) == 2
        real[0] = real[1] = real[2] = 71
        assert czt(3, float("nan"), 0, signal, None, real, None) == 3
        assert czt(3, 0, 2, signal, None, real, None) == 3
        assert list(real) == [71, 71, 71]
        czt_functions = {}
        for operation, arguments in {
            "new": [size, scalar, ctypes.c_uint32, slot],
            "length": [plan_type, ctypes.POINTER(size)],
            "transform": [plan_type, pointer, pointer, size, pointer, pointer],
            "release": [slot],
        }.items():
            function = getattr(library, f"ciglet_rs_czt_plan_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            czt_functions[operation] = function
        plan = plan_type()
        try:
            assert czt_functions["new"](3, 0, 0, ctypes.byref(plan)) == 0
            original = plan.value
            assert czt_functions["new"](3, float("nan"), 0, ctypes.byref(plan)) == 3
            assert plan.value == original
            count = size()
            assert czt_functions["length"](plan, ctypes.byref(count)) == 0 and count.value == 3
            assert czt_functions["transform"](plan, signal, None, 2, real, None) == 2
            assert list(real) == [71, 71, 71]
            for _ in range(3):
                assert czt_functions["transform"](plan, signal, None, 3, real, imaginary) == 0
                assert all(abs(value - 3) < tolerance for value in real)
                assert all(abs(value) < tolerance for value in imaginary)
        finally:
            assert czt_functions["release"](ctypes.byref(plan)) == 0 and plan.value is None
            assert czt_functions["release"](ctypes.byref(plan)) == 0
        assert czt(3, 0, 0, None, None, real, None) == 0
        assert list(real) == [0, 0, 0]
        frequency_functions = {}
        for operation in ["frequency_to_mel", "mel_to_frequency", "frequency_to_bark", "bark_to_frequency", "equal_loudness"]:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [scalar, pointer]
            function.restype = ctypes.c_uint32
            frequency_functions[operation] = function
        mel = scalar()
        frequency = scalar()
        bark = scalar()
        loudness = scalar()
        output = scalar(71)
        assert frequency_functions["frequency_to_mel"](1000, ctypes.byref(mel)) == 0 and mel.value > 0
        assert frequency_functions["mel_to_frequency"](mel, ctypes.byref(frequency)) == 0
        assert abs(frequency.value - 1000) < 1
        assert frequency_functions["frequency_to_bark"](1000, ctypes.byref(bark)) == 0 and bark.value > 0
        assert frequency_functions["bark_to_frequency"](bark, ctypes.byref(frequency)) == 0
        assert abs(frequency.value - 1000) < 0.001
        assert frequency_functions["equal_loudness"](1000, ctypes.byref(loudness)) == 0 and 0 < loudness.value < 1
        assert frequency_functions["frequency_to_mel"](-700, ctypes.byref(output)) == 3 and output.value == 71
        spacing = getattr(library, f"ciglet_rs_mel_space_{suffix}")
        spacing.argtypes = [scalar, scalar, size, pointer, size]
        spacing.restype = ctypes.c_uint32
        grid = (scalar * 3)(71, 71, 71)
        assert spacing(50, 8000, 2, grid, 2) == 2 and list(grid) == [71, 71, 71]
        assert spacing(8000, 50, 2, grid, 3) == 3 and list(grid) == [71, 71, 71]
        assert spacing(50, 8000, 2, grid, 3) == 0 and 0 < grid[0] < grid[1] < grid[2]
        distance_functions = {}
        for operation in ["itakura_saito", "log_itakura_saito", "log_itakura_saito_with"]:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [pointer, size, pointer, size] + ([ctypes.c_uint32] if operation.endswith("_with") else []) + [pointer]
            function.restype = ctypes.c_uint32
            distance_functions[operation] = function
        source = (scalar * 3)(2, 2, 2)
        reference = (scalar * 3)(1, 1, 1)
        tolerance = 1e-6 if suffix == "f32" else 1e-14
        expected = 1 - math.log(2)
        assert distance_functions["itakura_saito"](source, 3, reference, 3, ctypes.byref(output)) == 0
        assert abs(output.value - expected) < tolerance
        assert distance_functions["log_itakura_saito"](source, 3, reference, 3, ctypes.byref(output)) == 0
        assert abs(output.value - math.log(expected)) < tolerance
        explicit = scalar(71)
        assert distance_functions["log_itakura_saito_with"](source, 3, reference, 3, 0, ctypes.byref(explicit)) == 0
        assert explicit.value == output.value
        assert distance_functions["log_itakura_saito_with"](source, 3, reference, 3, 1, ctypes.byref(output)) == 0
        assert math.isfinite(output.value)
        assert distance_functions["itakura_saito"](reference, 3, reference, 3, ctypes.byref(output)) == 0 and output.value == 0
        assert distance_functions["log_itakura_saito"](reference, 3, reference, 3, ctypes.byref(output)) == 0
        assert output.value == float("-inf")
        zero = (scalar * 1)(0)
        assert distance_functions["itakura_saito"](zero, 1, reference, 1, ctypes.byref(output)) == 0
        assert output.value == float("inf")
        output.value = 71
        assert distance_functions["itakura_saito"](zero, 1, zero, 1, ctypes.byref(output)) == 3 and output.value == 71
        assert distance_functions["log_itakura_saito_with"](source, 3, reference, 3, 2, ctypes.byref(output)) == 3 and output.value == 71
        assert distance_functions["itakura_saito"](source, 3, reference, 2, ctypes.byref(output)) == 2 and output.value == 71
        fetch_frame = getattr(library, f"ciglet_rs_fetch_frame_{suffix}")
        fetch_frame.argtypes = [pointer, size, ctypes.c_ssize_t, pointer, size]
        fetch_frame.restype = ctypes.c_uint32
        frame = (scalar * 6)(1, 2, 3, 4, 5, 6)
        assert fetch_frame(frame, 6, 0, frame, 4) == 0
        assert list(frame) == [0, 0, 1, 2, 5, 6]
        frame[0] = frame[1] = frame[2] = 71
        minimum_center = -(1 << (ctypes.sizeof(ctypes.c_ssize_t) * 8 - 1))
        assert fetch_frame(None, 0, minimum_center, frame, 3) == 3
        assert list(frame)[:3] == [71, 71, 71]
        assert fetch_frame(None, 0, 0, frame, 3) == 0 and list(frame)[:3] == [0, 0, 0]
        search = {}
        for operation in ["extremum", "peak"]:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [pointer, size, size, size, ctypes.c_uint32, ctypes.POINTER(size)]
            function.restype = ctypes.c_uint32
            search[operation] = function
        signal = (scalar * 7)(8, 1, 5, 0, 5, 1, 9)
        index = size(71)
        assert search["peak"](signal, 7, 0, 6, 0, ctypes.byref(index)) == 0 and index.value == 0
        assert search["peak"](signal, 7, 1, 5, 0, ctypes.byref(index)) == 0 and index.value == 2
        assert search["peak"](signal, 7, 1, 5, 1, ctypes.byref(index)) == 0 and index.value == 3
        assert search["extremum"](signal, 7, 0, 6, 0, ctypes.byref(index)) == 0 and index.value == 6
        assert search["extremum"](signal, 7, 0, 6, 1, ctypes.byref(index)) == 0 and index.value == 3
        index.value = 71
        assert search["peak"](signal, 7, 5, 3, 0, ctypes.byref(index)) == 3 and index.value == 71
        assert search["extremum"](signal, 7, 0, 7, 0, ctypes.byref(index)) == 3 and index.value == 71
        assert search["peak"](signal, 7, 0, 6, 2, ctypes.byref(index)) == 3 and index.value == 71
        band_cepstra = {}
        for operation in ["band_to_cepstrum", "band_to_cepstrum_legacy", "band_spectrogram_to_cepstra"]:
            function = getattr(library, f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = [pointer, size] + ([size] if operation == "band_spectrogram_to_cepstra" else []) + [size, ctypes.c_uint32, pointer, size]
            function.restype = ctypes.c_uint32
            band_cepstra[operation] = function
        bands = (scalar * 4)(1, 2, 3, 4)
        full = (scalar * 4)()
        selected = (scalar * 2)()
        assert band_cepstra["band_to_cepstrum_legacy"](bands, 4, 1, 1, full, 4) == 0
        assert band_cepstra["band_to_cepstrum"](bands, 4, 1, 1, selected, 2) == 0
        assert list(selected) == list(full)[:2]
        # Strict original cos_2(0) retains its f32 approximation in both modes.
        assert abs(selected[1] - 4.999932050704956) < (5e-7 if suffix == "f32" else 1e-14)
        matrix = (scalar * 8)(1, 2, 3, 4, 1, 2, 3, 4)
        cepstra = (scalar * 4)()
        assert band_cepstra["band_spectrogram_to_cepstra"](matrix, 8, 4, 1, 1, cepstra, 4) == 0
        assert list(cepstra) == list(selected) * 2
        assert band_cepstra["band_to_cepstrum"](bands, 4, 1, 1, bands, 2) == 0
        assert list(bands) == list(selected) + [3, 4]
        sentinel = (scalar * 4)(71, 71, 71, 71)
        matrix[6] = float("nan")
        assert band_cepstra["band_spectrogram_to_cepstra"](matrix, 8, 4, 1, 1, sentinel, 4) == 3
        assert list(sentinel) == [71] * 4
        assert band_cepstra["band_to_cepstrum"](bands, 4, 1, 2, sentinel, 2) == 3 and sentinel[0] == 71
        assert band_cepstra["band_to_cepstrum_legacy"](bands, 4, 1, 1, sentinel, 2) == 2 and sentinel[0] == 71
        assert band_cepstra["band_to_cepstrum"](bands, 4, 0, 0, None, 0) == 0
        assert band_cepstra["band_spectrogram_to_cepstra"](None, 0, 4, 1, 1, None, 0) == 0

        handle_type = ctypes.c_void_p
        slot = ctypes.POINTER(handle_type)
        filterbank_signatures = {
            "empty": [size,scalar,size,slot],
            "mel": [size,scalar,size,scalar,scalar,scalar,scalar,slot],
            "mel_default": [size,scalar,size,scalar,scalar,slot],
            "plp": [size,scalar,size,slot],
            "clone": [handle_type,slot], "release": [slot],
            "shape": [handle_type,ctypes.POINTER(size),size],
            "nyquist": [handle_type,pointer], "set_nyquist": [handle_type,scalar],
            "response": [handle_type,size,pointer,size],
            "support": [handle_type,size,ctypes.POINTER(size),size],
            "set_channel": [handle_type,size,pointer,size,size,size],
            "apply": [handle_type,pointer,size,ctypes.c_uint32,pointer,size],
            "apply_spectrogram": [handle_type,pointer,size,ctypes.c_uint32,pointer,size],
            "apply_frames": [handle_type,pointer,size,size,ctypes.c_uint32,pointer,size],
        }
        bank_functions = {}
        for operation, arguments in filterbank_signatures.items():
            function = getattr(library, f"ciglet_rs_filterbank_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            bank_functions[operation] = function
        bank, copy = handle_type(), handle_type()
        assert bank_functions["empty"](4,8000,1,ctypes.byref(bank)) == 0
        shape = (size*2)(71,71)
        assert bank_functions["shape"](bank,shape,2) == 0 and list(shape) == [4,1]
        metadata = scalar(71)
        assert bank_functions["nyquist"](bank,ctypes.byref(metadata)) == 0 and metadata.value == 8000
        response = (scalar*4)(float("nan"),0.5,1,float("inf"))
        assert bank_functions["set_channel"](bank,0,response,4,1,3) == 0
        assert bank_functions["support"](bank,0,shape,2) == 0 and list(shape) == [1,3]
        copied = (scalar*4)()
        assert bank_functions["response"](bank,0,copied,4) == 0
        assert math.isnan(copied[0]) and list(copied)[1:] == [0.5,1,float("inf")]
        assert bank_functions["clone"](bank,ctypes.byref(copy)) == 0
        assert bank_functions["set_nyquist"](bank,float("nan")) == 0
        assert bank_functions["nyquist"](bank,ctypes.byref(metadata)) == 0 and math.isnan(metadata.value)
        assert bank_functions["nyquist"](copy,ctypes.byref(metadata)) == 0 and metadata.value == 8000
        spectrum = (scalar*4)(float("nan"),2,3,float("nan"))
        expected = scalar(71)
        assert bank_functions["apply"](bank,spectrum,4,1,ctypes.byref(expected),1) == 0 and 1 < expected.value < 2
        matrix = (scalar*8)(float("nan"),2,3,float("nan"),float("nan"),2,3,float("nan"))
        output = (scalar*2)()
        assert bank_functions["apply_spectrogram"](bank,matrix,8,1,output,2) == 0
        assert list(output) == [expected.value]*2
        assert bank_functions["apply_frames"](copy,matrix,8,2,1,output,2) == 0
        assert list(output) == [expected.value]*2
        assert bank_functions["apply"](bank,spectrum,4,1,spectrum,1) == 0 and spectrum[0] == expected.value
        matrix[5] = float("nan")
        output[0] = output[1] = 71
        assert bank_functions["apply_frames"](bank,matrix,8,2,0,output,2) == 3 and list(output) == [71,71]
        retained = bank.value
        assert bank_functions["mel_default"](0,8000,1,0,8000,ctypes.byref(bank)) == 3 and bank.value == retained
        assert bank_functions["release"](ctypes.byref(copy)) == 0 and copy.value is None
        assert bank_functions["release"](ctypes.byref(bank)) == 0 and bank.value is None
        assert bank_functions["release"](ctypes.byref(bank)) == 0
        assert bank_functions["mel"](65,8000,8,50,8000,0.85,120,ctypes.byref(bank)) == 0
        assert bank_functions["release"](ctypes.byref(bank)) == 0
        assert bank_functions["mel_default"](65,8000,8,50,8000,ctypes.byref(bank)) == 0
        assert bank_functions["release"](ctypes.byref(bank)) == 0
        assert bank_functions["plp"](65,8000,8,ctypes.byref(bank)) == 0
        assert bank_functions["release"](ctypes.byref(bank)) == 0
        assert bank_functions["empty"](0,float("nan"),1,ctypes.byref(bank)) == 0
        assert bank_functions["apply_frames"](bank,None,0,2,1,output,2) == 0 and list(output) == [0,0]
        assert bank_functions["apply_spectrogram"](bank,None,0,1,output,2) == 3
        assert bank_functions["release"](ctypes.byref(bank)) == 0
        real_cepstrum_signatures = {
            "new": [size,slot], "release": [slot],
            "shape": [handle_type,ctypes.POINTER(size),size],
            "analyze": [handle_type,pointer,size,ctypes.c_uint32,pointer,size],
            "reconstruct": [handle_type,pointer,size,ctypes.c_uint32,pointer,size],
            "minimum_phase": [handle_type,pointer,size,ctypes.c_uint32,ctypes.c_uint32,pointer,size],
            "spectrogram_to_cepstra": [handle_type,pointer,size,size,ctypes.c_uint32,pointer,size],
            "cepstra_to_spectrogram": [handle_type,pointer,size,size,size,ctypes.c_uint32,pointer,size],
            "from_log_spectrum": [pointer,size,pointer,size],
            "to_log_spectrum": [pointer,size,pointer,size],
            "minimum_phase_default": [pointer,size,pointer,size],
        }
        cepstrum_functions = {}
        for operation,arguments in real_cepstrum_signatures.items():
            function = getattr(library,f"ciglet_rs_real_cepstrum_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            cepstrum_functions[operation] = function
        plan = handle_type()
        assert cepstrum_functions["new"](4,ctypes.byref(plan)) == 0
        assert cepstrum_functions["shape"](plan,shape,2) == 0 and list(shape) == [4,3]
        logs = (scalar*3)(0,0,4)
        coefficients,spectrum,phase,modern = [(scalar*4)() for _ in range(4)]
        assert cepstrum_functions["analyze"](plan,logs,3,0,coefficients,4) == 0
        assert list(coefficients) == [1,-1,1,-1]
        assert cepstrum_functions["reconstruct"](plan,coefficients,4,0,spectrum,4) == 0
        assert list(spectrum) == [0,0,4,0]
        assert cepstrum_functions["minimum_phase"](plan,logs,3,0,0,phase,4) == 0
        assert list(phase) == [0,2,0,-2]
        assert cepstrum_functions["from_log_spectrum"](logs,3,modern,4) == 0 and list(modern) == list(coefficients)
        assert cepstrum_functions["to_log_spectrum"](coefficients,4,modern,4) == 0 and list(modern) == list(spectrum)
        assert cepstrum_functions["minimum_phase_default"](logs,3,modern,4) == 0 and list(modern) == list(phase)
        ignored = (scalar*3)(0,0,float("nan"))
        assert cepstrum_functions["analyze"](plan,ignored,3,1,modern,4) == 0 and list(modern) == [0]*4
        for i in range(4):
            modern[i] = 71
        assert cepstrum_functions["analyze"](plan,ignored,3,0,modern,4) == 3 and list(modern) == [71]*4
        magnitudes = (scalar*6)(1,1,1,1,1,1)
        matrix = (scalar*2)()
        reconstructed = (scalar*6)()
        assert cepstrum_functions["spectrogram_to_cepstra"](plan,magnitudes,6,1,0,matrix,2) == 0
        assert math.isfinite(matrix[0]) and matrix[0] == matrix[1]
        assert cepstrum_functions["cepstra_to_spectrogram"](plan,None,0,2,0,0,reconstructed,6) == 0
        assert list(reconstructed) == [reconstructed[0]]*6 and 0.9 < reconstructed[0] < 1.1
        magnitudes[3] = float("nan")
        matrix[0] = matrix[1] = 71
        assert cepstrum_functions["spectrogram_to_cepstra"](plan,magnitudes,6,1,0,matrix,2) == 3 and list(matrix) == [71]*2
        assert cepstrum_functions["analyze"](plan,logs,3,0,modern,4) == 0 and modern[0] == 1
        retained = plan.value
        assert cepstrum_functions["new"](3,ctypes.byref(plan)) == 3 and plan.value == retained
        assert cepstrum_functions["release"](ctypes.byref(plan)) == 0 and plan.value is None
        assert cepstrum_functions["release"](ctypes.byref(plan)) == 0
        polynomial_signatures = {
            "evaluate": [pointer,pointer,size,scalar,scalar,pointer,pointer],
            "evaluate_real": [pointer,size,scalar,scalar,pointer,pointer],
            "roots": [pointer,pointer,size,pointer,pointer,size,ctypes.POINTER(size),ctypes.POINTER(ctypes.c_uint32),ctypes.POINTER(ctypes.c_double)],
            "roots_real": [pointer,size,pointer,pointer,size,ctypes.POINTER(size),ctypes.POINTER(ctypes.c_uint32),ctypes.POINTER(ctypes.c_double)],
        }
        polynomial_functions = {}
        for operation,arguments in polynomial_signatures.items():
            function = getattr(library,f"ciglet_rs_polynomial_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            polynomial_functions[operation] = function
        coefficients = (scalar*3)(1,0,1)
        imaginary_coefficients = (scalar*3)(0,0,0)
        a,b = scalar(71),scalar(71)
        assert polynomial_functions["evaluate"](coefficients,imaginary_coefficients,3,2,1,ctypes.byref(a),ctypes.byref(b)) == 0
        assert (a.value,b.value) == (4,4)
        assert polynomial_functions["evaluate_real"](coefficients,3,2,1,ctypes.byref(a),ctypes.byref(b)) == 0
        assert (a.value,b.value) == (4,4)
        real,imaginary = (scalar*2)(),(scalar*2)()
        iterations,converged,residual = size(71),ctypes.c_uint32(71),ctypes.c_double(71)
        assert polynomial_functions["roots"](coefficients,None,3,real,imaginary,2,ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
        assert 0 < iterations.value <= 50 and converged.value == 1 and residual.value < 1e-6
        assert all(abs(r) < 1e-6 and abs(abs(i)-1) < 1e-6 for r,i in zip(real,imaginary))
        expected = (list(real),list(imaginary),iterations.value,converged.value,residual.value)
        assert polynomial_functions["roots_real"](coefficients,3,real,imaginary,2,ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
        assert (list(real),list(imaginary),iterations.value,converged.value,residual.value) == expected
        constant = (scalar*1)(1)
        assert polynomial_functions["roots"](None,constant,1,None,None,0,ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
        assert (iterations.value,converged.value,residual.value) == (0,1,0)
        invalid = (scalar*2)(0,1)
        real[0] = imaginary[0] = 71
        iterations.value = converged.value = 71
        residual.value = 71
        assert polynomial_functions["roots_real"](invalid,2,real,imaginary,1,ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 3
        assert (real[0],imaginary[0],iterations.value,converged.value,residual.value) == (71,71,71,71,71)
        assert polynomial_functions["evaluate_real"](coefficients,3,2,1,ctypes.byref(a),ctypes.byref(a)) == 2
        if suffix == "f64":
            nonconvergent = (scalar*9)(1,-36,546,-4536,22449,-67284,118124,-109584,40320)
            real,imaginary = (scalar*8)(),(scalar*8)()
            assert polynomial_functions["roots_real"](nonconvergent,9,real,imaginary,8,ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
            assert iterations.value == 50 and converged.value == 0 and residual.value > 1e-6
        levinson = getattr(library, f"ciglet_rs_prediction_levinson_{suffix}")
        levinson.argtypes = [pointer,size,ctypes.c_uint32,pointer,size]
        levinson.restype = ctypes.c_uint32
        prediction_gain = getattr(library, f"ciglet_rs_prediction_gain_{suffix}")
        prediction_gain.argtypes = [pointer,size,pointer,size,ctypes.c_uint32,pointer]
        prediction_gain.restype = ctypes.c_uint32
        correlation = (scalar*2)(2,0.5)
        coefficients = (scalar*2)()
        gain = scalar(71)
        for policy in (0,1):
            assert levinson(correlation,2,policy,coefficients,2) == 0
            assert coefficients[0] == 1 and -0.251 < coefficients[1] < -0.249
            assert prediction_gain(coefficients,2,correlation,2,policy,ctypes.byref(gain)) == 0
            assert 1 < gain.value < 2
            silence,silent_coefficients = (scalar*3)(),(scalar*3)()
            assert levinson(silence,3,policy,silent_coefficients,3) == 0
            assert list(silent_coefficients) == [1,0,0]
            assert prediction_gain(silent_coefficients,3,silence,3,policy,ctypes.byref(gain)) == 0 and gain.value == 0
        coefficients[0] = coefficients[1] = gain.value = 71
        assert levinson(correlation,2,2,coefficients,2) == 3 and list(coefficients) == [71,71]
        assert prediction_gain(coefficients,2,correlation,1,0,ctypes.byref(gain)) == 2 and gain.value == 71
        assert levinson(correlation,2,0,correlation,2) == 0
        assert correlation[0] == 1 and -0.251 < correlation[1] < -0.249
        scaled_signatures = {
            "from_samples": [pointer,size,size,slot],
            "from_magnitude": [pointer,size,size,ctypes.c_uint32,slot],
            "clone": [handle_type,slot],
            "release": [slot],
            "length": [handle_type,ctypes.POINTER(size)],
            "coefficients": [handle_type,pointer,size],
            "correlation_parts": [handle_type,ctypes.POINTER(ctypes.c_double),ctypes.POINTER(ctypes.c_int32),size],
            "gain_parts": [handle_type,ctypes.POINTER(ctypes.c_double),ctypes.POINTER(ctypes.c_int32)],
            "gain": [handle_type,pointer],
            "ordinary": [handle_type,pointer,pointer,size],
        }
        scaled_functions = {}
        for operation,arguments in scaled_signatures.items():
            function = getattr(library,f"ciglet_rs_scaled_prediction_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            scaled_functions[operation] = function
        samples = (scalar*2)(1,0)
        model,copy = handle_type(),handle_type()
        count = size(71)
        coefficients,correlations = (scalar*2)(),(scalar*2)()
        fractions,exponents = (ctypes.c_double*2)(),(ctypes.c_int32*2)()
        gain_fraction,gain_exponent = ctypes.c_double(71),ctypes.c_int32(71)
        estimate_samples = getattr(library,f"ciglet_rs_prediction_from_samples_{suffix}")
        estimate_samples.argtypes = [pointer,size,size,ctypes.c_uint32,pointer,pointer,size]
        estimate_samples.restype = ctypes.c_uint32
        estimate_magnitude = getattr(library,f"ciglet_rs_prediction_from_magnitude_{suffix}")
        estimate_magnitude.argtypes = [pointer,size,size,ctypes.c_uint32,ctypes.c_uint32,pointer,pointer,size]
        estimate_magnitude.restype = ctypes.c_uint32
        for arithmetic in (0,1):
            assert estimate_samples(samples,2,1,arithmetic,coefficients,correlations,2) == 0
            assert list(coefficients) == [1,0] and list(correlations) == [1,0]
            magnitudes = (scalar*3)(0,0,2)
            for nyquist in (0,1):
                assert estimate_magnitude(magnitudes,3,0,nyquist,arithmetic,coefficients,correlations,1) == 0
                assert coefficients[0] == 1 and correlations[0] == (1 if nyquist == 0 else 0)
        assert scaled_functions["from_samples"](samples,2,1,ctypes.byref(model)) == 0
        assert scaled_functions["length"](model,ctypes.byref(count)) == 0 and count.value == 2
        assert scaled_functions["ordinary"](model,coefficients,correlations,2) == 0
        assert list(coefficients) == [1,0] and list(correlations) == [1,0]
        assert scaled_functions["clone"](model,ctypes.byref(copy)) == 0 and copy.value != model.value
        assert scaled_functions["release"](ctypes.byref(model)) == 0 and model.value is None
        assert scaled_functions["coefficients"](copy,coefficients,2) == 0 and list(coefficients) == [1,0]
        assert scaled_functions["correlation_parts"](copy,fractions,exponents,2) == 0
        assert list(fractions) == [1,0] and list(exponents) == [0,0]
        assert scaled_functions["gain_parts"](copy,ctypes.byref(gain_fraction),ctypes.byref(gain_exponent)) == 0
        assert (gain_fraction.value,gain_exponent.value) == (1,0)
        assert scaled_functions["gain"](copy,ctypes.byref(gain)) == 0 and gain.value == 1
        assert scaled_functions["release"](ctypes.byref(copy)) == 0 and copy.value is None
        maximum = float.fromhex("0x1.fffffep+127" if suffix == "f32" else "0x1.fffffffffffffp+1023")
        samples[0] = maximum
        for index in range(2):
            coefficients[index] = correlations[index] = 71
        assert scaled_functions["from_samples"](samples,2,1,ctypes.byref(model)) == 0
        assert scaled_functions["ordinary"](model,coefficients,correlations,2) == 3
        assert list(coefficients) == [71,71] and list(correlations) == [71,71]
        assert scaled_functions["gain"](model,ctypes.byref(gain)) == 0 and gain.value == maximum
        assert scaled_functions["correlation_parts"](model,fractions,exponents,2) == 0
        assert 1 <= fractions[0] < 2 and exponents[0] > 128
        retained = model.value
        assert scaled_functions["from_samples"](None,0,0,ctypes.byref(model)) == 3 and model.value == retained
        assert scaled_functions["release"](ctypes.byref(model)) == 0 and model.value is None
        assert scaled_functions["release"](ctypes.byref(model)) == 0
        scaled_magnitudes = (scalar*3)(maximum,maximum,maximum)
        assert scaled_functions["from_magnitude"](scaled_magnitudes,3,3,0,ctypes.byref(model)) == 0
        assert scaled_functions["gain"](model,ctypes.byref(gain)) == 0 and gain.value == maximum
        assert scaled_functions["release"](ctypes.byref(model)) == 0 and model.value is None
        prediction_spectrum_signatures = {
            "new": [size,slot],
            "release": [slot],
            "shape": [handle_type,ctypes.POINTER(size),size],
            "estimate": [handle_type,pointer,size,size,ctypes.c_uint32,ctypes.c_uint32,pointer,pointer,size],
            "estimate_scaled": [handle_type,pointer,size,size,ctypes.c_uint32,slot],
            "spectrum": [handle_type,pointer,size,scalar,pointer,size],
        }
        prediction_spectrum_functions = {}
        for operation,arguments in prediction_spectrum_signatures.items():
            function = getattr(library,f"ciglet_rs_prediction_spectrum_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            prediction_spectrum_functions[operation] = function
        plan = handle_type()
        shape = (size*2)(71,71)
        input_values = (scalar*3)(1,1,1)
        coefficients,correlations,spectrum = (scalar*2)(),(scalar*2)(),(scalar*4)()
        assert prediction_spectrum_functions["new"](4,ctypes.byref(plan)) == 0
        assert prediction_spectrum_functions["shape"](plan,shape,2) == 0 and list(shape) == [4,3]
        for arithmetic in (0,1):
            assert prediction_spectrum_functions["estimate"](plan,input_values,3,1,0,arithmetic,coefficients,correlations,2) == 0
            assert list(coefficients) == [1,0] and list(correlations) == [1,0]
        assert prediction_spectrum_functions["spectrum"](plan,coefficients,2,2,spectrum,4) == 0 and list(spectrum) == [2]*4
        pole = (scalar*2)(1,-1)
        for index in range(4):
            spectrum[index] = 71
        assert prediction_spectrum_functions["spectrum"](plan,pole,2,1,spectrum,4) == 3 and list(spectrum) == [71]*4
        assert prediction_spectrum_functions["spectrum"](plan,coefficients,2,2,spectrum,4) == 0 and list(spectrum) == [2]*4
        retained = plan.value
        assert prediction_spectrum_functions["new"](3,ctypes.byref(plan)) == 3 and plan.value == retained
        extreme = (scalar*3)(maximum,maximum,maximum)
        assert prediction_spectrum_functions["estimate_scaled"](plan,extreme,3,3,0,ctypes.byref(model)) == 0
        assert prediction_spectrum_functions["release"](ctypes.byref(plan)) == 0 and plan.value is None
        assert scaled_functions["gain"](model,ctypes.byref(gain)) == 0 and gain.value == maximum
        assert scaled_functions["release"](ctypes.byref(model)) == 0 and model.value is None
        assert prediction_spectrum_functions["release"](ctypes.byref(plan)) == 0
        resonances = getattr(library,f"ciglet_rs_prediction_resonances_{suffix}")
        resonances.argtypes = [pointer,size,pointer,pointer,pointer,size,ctypes.POINTER(size),ctypes.POINTER(size),ctypes.POINTER(ctypes.c_uint32),ctypes.POINTER(ctypes.c_double)]
        resonances.restype = ctypes.c_uint32
        coefficients = (scalar*3)(1,0,1)
        frequencies,real,imaginary = [(scalar*3)(71,71,71) for _ in range(3)]
        count,iterations,converged,residual = size(71),size(71),ctypes.c_uint32(71),ctypes.c_double(71)
        assert resonances(coefficients,3,frequencies,real,imaginary,3,ctypes.byref(count),ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
        assert count.value == 1 and 0 < iterations.value <= 50 and converged.value == 1 and residual.value < 1e-6
        assert abs(frequencies[0]-0.5) < 1e-6 and abs(real[0]) < 1e-6 and abs(imaginary[0]-1) < 1e-6
        assert all(list(plane)[1:] == [71,71] for plane in (frequencies,real,imaginary))
        count.value = iterations.value = converged.value = 71
        residual.value = 71
        assert resonances(coefficients,3,None,None,None,0,ctypes.byref(count),ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 2
        assert (count.value,iterations.value,converged.value,residual.value) == (71,71,71,71)
        constant = (scalar*1)(1)
        assert resonances(constant,1,None,None,None,0,ctypes.byref(count),ctypes.byref(iterations),ctypes.byref(converged),ctypes.byref(residual)) == 0
        assert (count.value,iterations.value,converged.value,residual.value) == (0,0,1,0)
        matrix_signatures = {
            "matrix_from_column_major": [size,size,pointer,size,ctypes.POINTER(handle_type)],
            "matrix_zeros": [size,size,ctypes.POINTER(handle_type)],
            "matrix_identity": [size,ctypes.POINTER(handle_type)],
            "matrix_clone": [handle_type,ctypes.POINTER(handle_type)],
            "matrix_release": [ctypes.POINTER(handle_type)],
            "matrix_shape": [handle_type,ctypes.POINTER(size),size],
            "matrix_copy_column_major": [handle_type,pointer,size],
            "matrix_get": [handle_type,size,size,pointer],
            "matrix_multiply": [handle_type,handle_type,ctypes.POINTER(handle_type)],
            "matrix_multiply_vector": [handle_type,pointer,size,pointer,size],
            "matrix_apply_row_swaps": [handle_type,ctypes.POINTER(size),size],
            "matrix_pivot_rows": [handle_type,ctypes.POINTER(size),size],
            "matrix_apply_swaps": [pointer,size,ctypes.POINTER(size),size,pointer,size],
            "lu_factor": [handle_type,ctypes.c_uint32,ctypes.POINTER(handle_type)],
            "lu_clone": [handle_type,ctypes.POINTER(handle_type)],
            "lu_release": [ctypes.POINTER(handle_type)],
            "lu_shape": [handle_type,ctypes.POINTER(size),size],
            "lu_copy_packed": [handle_type,pointer,size],
            "lu_copy_row_swaps": [handle_type,ctypes.POINTER(size),size],
            "lu_solve": [handle_type,pointer,size,pointer,size],
        }
        matrices = {}
        for operation, arguments in matrix_signatures.items():
            function = getattr(library,f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            matrices[operation] = function
        a,b,product,copy,factor,factor_copy = [handle_type() for _ in range(6)]
        input_values = (scalar*9)(4,2,1,4,2,0,1,2,3)
        values,right,solution,value = (scalar*9)(),(scalar*3)(15,12,10),(scalar*3)(),scalar(71)
        shape,swaps = (size*3)(),(size*3)()
        assert matrices["matrix_from_column_major"](3,3,input_values,9,ctypes.byref(a)) == 0
        assert matrices["matrix_shape"](a,shape,3) == 0 and list(shape) == [3,3,9]
        assert matrices["matrix_get"](a,2,2,ctypes.byref(value)) == 0 and value.value == 3
        assert matrices["matrix_identity"](3,ctypes.byref(b)) == 0
        assert matrices["matrix_multiply"](a,b,ctypes.byref(product)) == 0
        assert matrices["matrix_copy_column_major"](product,values,9) == 0 and list(values) == list(input_values)
        assert matrices["matrix_clone"](a,ctypes.byref(copy)) == 0
        assert matrices["matrix_pivot_rows"](copy,swaps,3) == 0 and list(swaps) == [0,1,2]
        assert matrices["lu_factor"](copy,1,ctypes.byref(factor)) == 3 and factor.value is None
        assert matrices["lu_factor"](a,0,ctypes.byref(factor)) == 0
        assert matrices["lu_clone"](factor,ctypes.byref(factor_copy)) == 0
        assert matrices["lu_release"](ctypes.byref(factor)) == 0 and factor.value is None
        assert matrices["lu_shape"](factor_copy,shape,3) == 0 and list(shape) == [3,3,9]
        assert matrices["lu_copy_packed"](factor_copy,values,9) == 0
        assert matrices["lu_copy_row_swaps"](factor_copy,swaps,3) == 0
        assert matrices["matrix_apply_row_swaps"](copy,swaps,3) == 0
        assert matrices["matrix_apply_swaps"](right,3,swaps,3,solution,3) == 0
        assert matrices["lu_solve"](factor_copy,right,3,solution,3) == 0
        assert all(abs(solution[i]-(i+1)) < 2e-6 for i in range(3))
        assert matrices["matrix_multiply_vector"](a,solution,3,solution,3) == 0
        assert all(abs(solution[i]-right[i]) < 2e-6 for i in range(3))
        assert matrices["lu_release"](ctypes.byref(factor_copy)) == 0 and factor_copy.value is None
        for owner in (a,b,product,copy):
            assert matrices["matrix_release"](ctypes.byref(owner)) == 0 and owner.value is None
        assert matrices["matrix_zeros"](2,0,ctypes.byref(a)) == 0
        assert matrices["matrix_multiply_vector"](a,None,0,solution,2) == 0 and list(solution)[:2] == [0,0]
        assert matrices["matrix_release"](ctypes.byref(a)) == 0 and a.value is None
        assert matrices["matrix_release"](ctypes.byref(a)) == 0
        filtering_signatures = {
            "windowed_fir": [size,ctypes.c_uint32,scalar,scalar,ctypes.c_uint32,pointer,size],
            "fir": [size,scalar,ctypes.c_uint32,ctypes.c_uint32,pointer,size],
            "bandpass_fir": [size,scalar,scalar,ctypes.c_uint32,pointer,size],
            "convolve": [pointer,size,pointer,size,pointer,size],
            "filter": [pointer,size,pointer,size,pointer,size,ctypes.c_uint32,pointer,size],
            "forward_backward_filter": [pointer,size,pointer,size,pointer,size,ctypes.c_uint32,pointer,size],
        }
        filters = {}
        for operation,arguments in filtering_signatures.items():
            function = getattr(library,f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            filters[operation] = function
        impulse,numerator,denominator,output = (scalar*3)(1,0,0),(scalar*2)(0.5,0),(scalar*2)(1,-0.5),(scalar*4)(71,71,71,71)
        assert filters["filter"](impulse,3,numerator,2,denominator,2,0,output,3) == 0
        assert list(output)[:3] == [0.5,0.25,0.125]
        assert filters["filter"](impulse,3,numerator,2,denominator,2,1,output,3) == 0 and list(output)[:3] == [0,0,0]
        signal,coefficients,kernel = (scalar*3)(1,2,3),(scalar*2)(1,2),(scalar*2)(2,-1)
        assert filters["forward_backward_filter"](signal,3,coefficients,2,None,0,0,output,3) == 0 and list(output)[:3] == [9,18,7]
        assert filters["convolve"](signal,3,kernel,2,output,4) == 0 and list(output) == [2,3,4,-3]
        taps,normalized = (scalar*4)(),(scalar*4)()
        for band in range(4):
            assert filters["windowed_fir"](4,band,0.125,0.375,0,taps,4) == 0
            if band < 2:
                assert filters["fir"](4,0.25,band,0,normalized,4) == 0 and list(taps) == list(normalized)
            elif band == 2:
                assert filters["bandpass_fir"](4,0.25,0.75,0,normalized,4) == 0 and list(taps) == list(normalized)
        for index in range(4):
            output[index] = 71
        assert filters["filter"](impulse,3,numerator,2,denominator,2,2,output,3) == 3
        assert filters["windowed_fir"](4,4,0,0,0,output,4) == 3 and list(output) == [71]*4
        assert filters["convolve"](None,0,kernel,2,None,0) == 0
        assert filters["forward_backward_filter"](None,0,None,0,None,0,0,None,0) == 0
        resampling_signatures = {
            "resample_output_length": [size,scalar,ctypes.POINTER(size)],
            "resample": [pointer,size,scalar,pointer,size],
            "resample_with": [pointer,size,scalar,ctypes.c_uint32,pointer,size],
            "resample_with_kernel": [pointer,size,scalar,ctypes.c_uint32,ctypes.c_uint32,pointer,size],
        }
        resamplers = {}
        for operation,arguments in resampling_signatures.items():
            function = getattr(library,f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            resamplers[operation] = function
        input_values,output,count = (scalar*1)(0.75),(scalar*2)(71,71),size(71)
        assert resamplers["resample_output_length"](1,2,ctypes.byref(count)) == 0 and count.value == 2
        assert resamplers["resample"](input_values,1,2,output,2) == 0 and output[0] == 0.75
        assert resamplers["resample_with"](input_values,1,2,1,output,2) == 0 and list(output) == [0,0]
        for boundary in (0,1):
            for kernel in (0,1):
                assert resamplers["resample_with_kernel"](input_values,1,2,boundary,kernel,output,2) == 0
                assert output[0] == (0.75 if boundary == 0 else 0)
        output[0] = output[1] = 71
        assert resamplers["resample_with_kernel"](input_values,1,2,0,2,output,2) == 3 and list(output) == [71,71]
        assert resamplers["resample"](input_values,1,2,output,1) == 2 and list(output) == [71,71]
        count.value = 71
        assert resamplers["resample_output_length"](1,0,ctypes.byref(count)) == 3 and count.value == 71
        assert resamplers["resample"](None,0,2,None,0) == 0
        assert resamplers["resample"](input_values,1,0.25,None,0) == 0
        signed_zero = (scalar*1)(-0.0)
        assert resamplers["resample"](signed_zero,1,1,signed_zero,1) == 0 and math.copysign(1,signed_zero[0]) == -1
        interpolation_signatures = {
            "between": [scalar,scalar,scalar,pointer],
            "linear": [pointer,pointer,size,pointer,size,pointer,size],
            "uniform": [scalar,scalar,pointer,size,pointer,size,pointer,size],
            "sinc": [scalar,scalar,pointer,size,pointer,size,pointer,size],
            "sinc_with": [scalar,scalar,pointer,size,pointer,size,ctypes.c_uint32,pointer,size],
            "sinc_with_kernel": [scalar,scalar,pointer,size,pointer,size,ctypes.c_uint32,ctypes.c_uint32,pointer,size],
            "fill_missing": [pointer,size,scalar,pointer,size],
        }
        interpolators = {}
        for operation,arguments in interpolation_signatures.items():
            function = getattr(library,f"ciglet_rs_interpolate_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            interpolators[operation] = function
        value = scalar(71)
        assert interpolators["between"](-maximum,maximum,0.5,ctypes.byref(value)) == 0 and value.value == 0
        coordinates,values,queries,output = (scalar*3)(0,2,5),(scalar*3)(1,5,-1),(scalar*4)(4,1,-9,10),(scalar*4)(71,71,71,71)
        assert interpolators["linear"](coordinates,values,3,queries,4,output,4) == 0 and list(output) == [1,3,1,-1]
        uniform,positions = (scalar*3)(1,3,5),(scalar*4)(1.5,-1,8,0)
        assert interpolators["uniform"](0,3,uniform,3,positions,4,output,4) == 0 and list(output) == [4,1,5,1]
        impulse,distant = (scalar*2)(7,0),(scalar*3)(0,100,-100)
        assert interpolators["sinc"](0,2,impulse,2,distant,3,output,3) == 0 and list(output)[:3] == [7,0,0]
        for boundary in (0,1):
            expected = 7 if boundary == 0 else 0
            assert interpolators["sinc_with"](0,2,impulse,2,distant,3,boundary,output,3) == 0 and output[0] == expected
            for kernel in (0,1):
                assert interpolators["sinc_with_kernel"](0,2,impulse,2,distant,3,boundary,kernel,output,3) == 0 and list(output)[:3] == [expected,0,0]
        gaps = (scalar*5)(math.nan,2,math.nan,6,math.nan)
        assert interpolators["fill_missing"](gaps,5,math.nan,gaps,5) == 0 and list(gaps) == [2,2,4,6,6]
        for index in range(4):
            output[index] = 71
        assert interpolators["sinc_with_kernel"](0,2,impulse,2,distant,3,0,2,output,3) == 3
        assert interpolators["linear"](coordinates,values,3,queries,4,output,3) == 2 and list(output) == [71]*4
        assert interpolators["uniform"](0,2,impulse,2,None,0,None,0) == 0
        assert interpolators["fill_missing"](None,0,math.nan,None,0) == 0
        smoothing_signatures = {
            "moving_average": [pointer,size,scalar,pointer,size],
            "moving_average_with": [pointer,size,scalar,ctypes.c_uint32,pointer,size],
            "moving_rms": [pointer,size,size,pointer,size],
            "moving_rms_with": [pointer,size,size,ctypes.c_uint32,pointer,size],
            "median_filter": [pointer,size,size,pointer,size],
        }
        smoothers = {}
        for operation,arguments in smoothing_signatures.items():
            function = getattr(library,f"ciglet_rs_{operation}_{suffix}")
            function.argtypes = arguments
            function.restype = ctypes.c_uint32
            smoothers[operation] = function
        signal,output,selected = (scalar*3)(1,2,8),(scalar*4)(71,71,71,71),(scalar*3)()
        assert smoothers["moving_average"](signal,3,1,output,3) == 0 and list(output)[:3] == [1.5,5,8]
        assert smoothers["moving_average_with"](signal,3,1,0,selected,3) == 0 and list(selected) == list(output)[:3]
        assert smoothers["moving_average_with"](signal,3,0,1,selected,3) == 0 and list(selected) == list(signal)
        signed_input = (scalar*2)(-3,4)
        assert smoothers["moving_rms"](signed_input,2,0,output,2) == 0 and list(output)[:2] == [3,4]
        assert smoothers["moving_rms_with"](signed_input,2,0,0,selected,2) == 0 and list(selected)[:2] == [3,4]
        peak = (scalar*1)(maximum)
        assert smoothers["moving_rms"](peak,1,1,output,1) == 0 and output[0] == maximum
        output[0] = 71
        assert smoothers["moving_rms_with"](peak,1,1,1,output,1) == 3 and output[0] == 71
        median = (scalar*4)(4,1,9,2)
        assert smoothers["median_filter"](median,4,2,median,4) == 0 and list(median) == [4,2.5,5,2]
        for index in range(4):
            output[index] = 71
        assert smoothers["moving_average_with"](signal,3,1,2,output,3) == 3
        assert smoothers["median_filter"](median,4,0,output,4) == 3 and list(output) == [71]*4
        assert smoothers["moving_average"](None,0,1,None,0) == 0
        assert smoothers["moving_rms"](None,0,1,None,0) == 0
        assert smoothers["median_filter"](None,0,3,None,0) == 0
        sinc_functions = {}
        for operation in ("aliased", "aliased_with", "derivative", "derivative_with"):
            function = getattr(library, f"ciglet_rs_sinc_{operation}_{suffix}")
            function.argtypes = [scalar, scalar] + ([ctypes.c_uint32] if operation.endswith("with") else []) + [pointer]
            function.restype = ctypes.c_uint32
            sinc_functions[operation] = function
        value, selected, derivative = scalar(71), scalar(71), scalar(71)
        assert sinc_functions["aliased"](2,0,ctypes.byref(value)) == 0 and value.value == 2
        for policy in (0,1):
            assert sinc_functions["aliased_with"](2,0,policy,ctypes.byref(selected)) == 0 and selected.value == 2
        assert sinc_functions["derivative"](2,1,ctypes.byref(derivative)) == 0
        assert abs(derivative.value + math.sin(0.5)) < (1e-7 if suffix == "f32" else 2e-15)
        assert sinc_functions["derivative_with"](2,1,0,ctypes.byref(selected)) == 0 and selected.value == derivative.value
        assert sinc_functions["derivative_with"](3,0,1,ctypes.byref(selected)) == 0 and selected.value == 0
        assert sinc_functions["aliased"](maximum,0,ctypes.byref(value)) == 0 and value.value == maximum
        value.value = selected.value = derivative.value = 71
        assert sinc_functions["aliased"](math.nan,0,ctypes.byref(value)) == 3 and value.value == 71
        assert sinc_functions["derivative"](2,math.inf,ctypes.byref(derivative)) == 3 and derivative.value == 71
        for operation in ("aliased_with", "derivative_with"):
            assert sinc_functions[operation](2,0,2,ctypes.byref(selected)) == 3 and selected.value == 71
        assert sinc_functions["aliased"](2,0,None) == 1
        assert sinc_functions["derivative"](2,0,None) == 1
        synthesize_one = getattr(library, f"ciglet_rs_sinusoid_{suffix}")
        synthesize_one.argtypes = [scalar,scalar,scalar,ctypes.c_uint32,size,pointer,size]
        synthesize_one.restype = ctypes.c_uint32
        synthesize_many = getattr(library, f"ciglet_rs_sinusoids_{suffix}")
        synthesize_many.argtypes = [pointer,pointer,pointer,size,ctypes.c_uint32,size,pointer,size]
        synthesize_many.restype = ctypes.c_uint32
        frequencies,amplitudes,phases,output = (scalar*2)(0,0),(scalar*2)(0.25,-0.5),(scalar*2)(0,0),(scalar*3)(71,71,71)
        assert synthesize_one(0,0.25,0,16000,3,output,3) == 0 and list(output) == [0.25]*3
        assert synthesize_many(frequencies,amplitudes,phases,2,16000,3,output,3) == 0 and list(output) == [-0.25]*3
        assert synthesize_many(None,None,None,0,16000,3,output,3) == 0 and list(output) == [0]*3
        for index in range(3):
            output[index] = 71
        assert synthesize_one(math.nan,1,0,16000,3,output,3) == 3
        assert synthesize_one(0,1,0,16000,3,output,2) == 2
        assert synthesize_many(None,amplitudes,phases,2,16000,3,output,3) == 1
        assert synthesize_many(frequencies,amplitudes,phases,2,0,3,output,3) == 3 and list(output) == [71]*3
        assert synthesize_one(0,1,0,16000,0,None,0) == 0
        assert synthesize_many(None,None,None,0,16000,0,None,0) == 0
        shared = (scalar*3)(0,0,0)
        assert synthesize_many(shared,amplitudes,phases,1,16000,3,shared,3) == 0 and list(shared) == [0.25]*3
        kalman_filter = getattr(library, f"ciglet_rs_kalman_filter_{suffix}")
        kalman_filter.argtypes = [pointer,pointer,pointer,size,pointer,pointer,size,pointer]
        kalman_filter.restype = ctypes.c_uint32
        kalman_initial = getattr(library, f"ciglet_rs_kalman_filter_with_initial_{suffix}")
        kalman_initial.argtypes = [pointer,pointer,pointer,size,scalar,pointer,pointer,size,pointer]
        kalman_initial.restype = ctypes.c_uint32
        kalman_smooth = getattr(library, f"ciglet_rs_kalman_smooth_{suffix}")
        kalman_smooth.argtypes = [pointer,pointer,pointer,size,pointer,size]
        kalman_smooth.restype = ctypes.c_uint32
        measurement,process,noise,means,variances,likelihood = (scalar*1)(1),(scalar*1)(1),(scalar*1)(1),(scalar*2)(71,71),(scalar*2)(71,71),scalar(71)
        assert kalman_filter(measurement,process,noise,1,means,variances,1,ctypes.byref(likelihood)) == 0
        assert means[0] == 1 and variances[0] == 0.5 and math.isfinite(likelihood.value)
        assert kalman_initial(measurement,process,noise,1,0,means,variances,1,ctypes.byref(likelihood)) == 0
        assert means[0] == 0.5 and variances[0] == 0.5 and math.isfinite(likelihood.value)
        posterior_means,posterior_variances,steps = (scalar*2)(0.5,0.2),(scalar*2)(0.5,0.6),(scalar*2)(1,1)
        assert kalman_smooth(posterior_means,posterior_variances,steps,2,means,2) == 0
        assert abs(means[0]-0.4) < (1e-7 if suffix == "f32" else 1e-15) and means[1] == posterior_means[1]
        for index in range(2):
            means[index] = variances[index] = 71
        likelihood.value = 71
        assert kalman_filter(measurement,process,noise,1,means,means,1,ctypes.byref(likelihood)) == 2
        assert kalman_filter(measurement,process,noise,1,means,variances,1,means) == 2
        assert kalman_initial(measurement,process,noise,1,math.nan,means,variances,1,ctypes.byref(likelihood)) == 3
        assert kalman_smooth(measurement,(scalar*1)(-1),process,1,means,1) == 3
        assert list(means) == list(variances) == [71,71] and likelihood.value == 71
        assert kalman_filter(None,None,None,0,None,None,0,ctypes.byref(likelihood)) == 0 and likelihood.value == 0
        assert kalman_initial(None,None,None,0,0,None,None,0,ctypes.byref(likelihood)) == 0 and likelihood.value == 0
        assert kalman_smooth(None,None,None,0,None,0) == 0
        assert kalman_filter(None,None,None,0,None,None,0,None) == 1
        assert kalman_initial(measurement,process,noise,1,0,measurement,variances,1,ctypes.byref(likelihood)) == 0 and measurement[0] == 0.5
        correlogram_compute = getattr(library, f"ciglet_rs_correlogram_compute_{suffix}")
        correlogram_compute.argtypes = [pointer,size,ctypes.POINTER(ctypes.c_int32),ctypes.POINTER(size),size,size,ctypes.c_int32,pointer,size]
        correlogram_compute.restype = ctypes.c_uint32
        correlogram_selected = getattr(library, f"ciglet_rs_correlogram_compute_with_{suffix}")
        correlogram_selected.argtypes = correlogram_compute.argtypes[:-2] + [ctypes.c_uint32,pointer,size]
        correlogram_selected.restype = ctypes.c_uint32
        correlogram_frequency = getattr(library, f"ciglet_rs_correlogram_to_frequency_{suffix}")
        correlogram_frequency.argtypes = [pointer,size,size,size,ctypes.c_int32,pointer,size,pointer,size]
        correlogram_frequency.restype = ctypes.c_uint32
        samples,output,selected,center,length = (scalar*8)(*range(8)),(scalar*5)(71,71,71,71,71),(scalar*5)(),ctypes.c_int32(2),size(4)
        for method in range(4):
            assert correlogram_compute(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,method,output,5) == 0
            assert correlogram_selected(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,method,0,selected,5) == 0 and list(output) == list(selected)
            assert correlogram_selected(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,method,1,selected,5) == 0
        assert correlogram_compute(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,1,output,5) == 0 and list(output) == [0,4,8,12,0]
        periods,frequencies = (scalar*4)(0,10,20,30),(scalar*5)(100,50,40,0,-0.0)
        assert correlogram_frequency(periods,1,4,4,100,frequencies,5,output,5) == 0 and list(output) == [10,20,25,30,0]
        for index in range(5):
            output[index] = 71
        assert correlogram_selected(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,0,2,output,5) == 3
        assert correlogram_compute(samples,8,ctypes.byref(center),ctypes.byref(length),1,5,4,output,5) == 3
        assert correlogram_frequency(periods,1,4,4,0,frequencies,5,output,5) == 3 and list(output) == [71]*5
        assert correlogram_compute(None,0,None,None,0,5,0,None,0) == 0
        assert correlogram_frequency(None,0,4,0,100,frequencies,5,None,0) == 0
        root_callback_type = ctypes.CFUNCTYPE(scalar,scalar,ctypes.c_void_p)
        root_find = getattr(library, f"ciglet_rs_root_find_{suffix}")
        root_find.argtypes = [root_callback_type,ctypes.c_void_p,scalar,scalar,pointer]
        root_find.restype = ctypes.c_uint32
        root_state = {"calls":0,"kind":0}
        root_target = scalar(0.25)
        @root_callback_type
        def root_callback(value, opaque):
            root_state["calls"] += 1
            if root_state["kind"] == 1:
                # Match each native scalar operation before returning through C ABI.
                return scalar(scalar(value*value).value-2).value
            if root_state["kind"] == 2 or (root_state["kind"] == 3 and value == 0):
                return math.nan
            target = ctypes.cast(opaque,pointer).contents.value if opaque else 0.25
            return scalar(value-target).value
        value = scalar(71)
        assert root_find(root_callback,ctypes.byref(root_target),0,1,ctypes.byref(value)) == 0 and value.value == 0.25 and root_state["calls"] >= 2
        root_state["kind"] = 1
        assert root_find(root_callback,None,0,2,ctypes.byref(value)) == 0
        assert abs(value.value*value.value-2) < (1e-6 if suffix == "f32" else 1e-7)
        value.value = 71
        root_state["kind"] = 2
        assert root_find(root_callback,None,0,1,ctypes.byref(value)) == 3 and value.value == 71
        root_state["kind"] = 3
        root_state["calls"] = 0
        root_target.value = 0
        assert root_find(root_callback,ctypes.byref(root_target),-1,1,ctypes.byref(value)) == 3 and value.value == 71 and root_state["calls"] >= 3
        assert root_find(root_callback_type(),None,0,1,ctypes.byref(value)) == 1 and value.value == 71
        assert root_find(root_callback,None,0,1,None) == 1
        assert root_find(root_callback,None,3,1,ctypes.byref(value)) == 3 and value.value == 71
        root_state["kind"] = 0
        assert root_find(root_callback,None,0,1,ctypes.byref(value)) == 0 and value.value == 0.25
        peak_interpolate = getattr(library,f"ciglet_rs_spectral_peak_interpolate_{suffix}")
        peak_interpolate.argtypes = [pointer,size,size,pointer,pointer]
        peak_interpolate.restype = ctypes.c_uint32
        samples,bin_value,magnitude_value = (scalar*5)(math.nan,0,1,0.5,math.nan),scalar(71),scalar(71)
        tolerance = 2e-7 if suffix == "f32" else 1e-15
        assert peak_interpolate(samples,5,2,ctypes.byref(bin_value),ctypes.byref(magnitude_value)) == 0
        assert abs(bin_value.value-13/6) < tolerance and abs(magnitude_value.value-49/48) < tolerance
        assert peak_interpolate((scalar*3)(0,10,10),3,1,ctypes.byref(bin_value),ctypes.byref(magnitude_value)) == 0
        assert bin_value.value == 1.5 and magnitude_value.value == scalar(10.2).value
        bin_value.value = magnitude_value.value = 71
        assert peak_interpolate(samples,5,0,ctypes.byref(bin_value),ctypes.byref(magnitude_value)) == 3
        assert peak_interpolate(samples,5,2,ctypes.byref(bin_value),ctypes.byref(bin_value)) == 2
        assert peak_interpolate(None,0,0,ctypes.byref(bin_value),ctypes.byref(magnitude_value)) == 3
        assert peak_interpolate(samples,5,2,None,ctypes.byref(magnitude_value)) == 1
        assert bin_value.value == magnitude_value.value == 71
        shared = (scalar*3)(0,1,0.5)
        assert peak_interpolate(shared,3,1,shared,ctypes.cast(ctypes.byref(shared,ctypes.sizeof(scalar)),pointer)) == 0
        assert abs(shared[0]-7/6) < tolerance and abs(shared[1]-49/48) < tolerance
        owner_pointer = ctypes.POINTER(ctypes.c_void_p)
        detector_signatures = {
            "create": [scalar,scalar,owner_pointer],
            "from_kernels": [scalar,pointer,pointer,pointer,pointer,size,owner_pointer],
            "clone": [ctypes.c_void_p,owner_pointer],
            "release": [owner_pointer],
            "kernel_count": [ctypes.c_void_p,ctypes.POINTER(size)],
            "center_frequency": [ctypes.c_void_p,pointer],
            "set_center_frequency": [ctypes.c_void_p,scalar],
            "copy_kernels": [ctypes.c_void_p,pointer,pointer,pointer,pointer,size],
            "replace_kernels": [ctypes.c_void_p,pointer,pointer,pointer,pointer,size],
            "estimate": [ctypes.c_void_p,pointer,size,pointer],
            "estimate_with": [ctypes.c_void_p,pointer,size,ctypes.c_uint32,pointer],
        }
        detector = {}
        for operation, signature in detector_signatures.items():
            function = getattr(library,f"ciglet_rs_if_detector_{operation}_{suffix}")
            function.argtypes = signature
            function.restype = ctypes.c_uint32
            detector[operation] = function
        owner,copy,custom = ctypes.c_void_p(),ctypes.c_void_p(),ctypes.c_void_p()
        count,center,value = size(71),scalar(71),scalar(71)
        planes = [(scalar*13)() for _ in range(4)]
        assert detector["create"](0.125,0.3,ctypes.byref(owner)) == 0 and owner.value
        assert detector["kernel_count"](owner,ctypes.byref(count)) == 0 and count.value == 13
        assert detector["copy_kernels"](owner,*planes,13) == 0
        assert all(math.isfinite(element) for plane in planes for element in plane)
        assert detector["center_frequency"](owner,ctypes.byref(center)) == 0 and center.value == 0.125
        assert detector["clone"](owner,ctypes.byref(copy)) == 0 and copy.value and copy.value != owner.value
        assert detector["set_center_frequency"](copy,0.25) == 0
        assert detector["center_frequency"](copy,ctypes.byref(center)) == 0 and center.value == 0.25
        assert detector["center_frequency"](owner,ctypes.byref(center)) == 0 and center.value == 0.125
        kernels = [(scalar*2)(*values) for values in ([1,0],[0,1],[0,-2],[2,0])]
        samples = (scalar*2)(1,2)
        assert detector["from_kernels"](0.125,*kernels,2,ctypes.byref(custom)) == 0 and custom.value
        tolerance = 1e-7 if suffix == "f32" else 2e-16
        assert detector["estimate"](custom,samples,2,ctypes.byref(value)) == 0 and abs(value.value-1/math.pi) < tolerance
        for policy in (0,1):
            assert detector["estimate_with"](custom,samples,2,policy,ctypes.byref(value)) == 0 and abs(value.value-1/math.pi) < tolerance
        assert detector["replace_kernels"](copy,*kernels,2) == 0
        assert detector["kernel_count"](copy,ctypes.byref(count)) == 0 and count.value == 2
        assert detector["kernel_count"](owner,ctypes.byref(count)) == 0 and count.value == 13
        assert detector["set_center_frequency"](copy,math.nan) == 3
        assert detector["center_frequency"](copy,ctypes.byref(center)) == 0 and center.value == 0.25
        assert detector["replace_kernels"](copy,None,None,None,None,0) == 3
        assert detector["kernel_count"](copy,ctypes.byref(count)) == 0 and count.value == 2
        for plane in planes:
            for index in range(13):
                plane[index] = 71
        assert detector["copy_kernels"](copy,planes[0],planes[0],planes[2],planes[3],2) == 2
        assert detector["copy_kernels"](copy,*planes,1) == 2
        assert all(list(plane) == [71]*13 for plane in planes)
        value.value = 71
        assert detector["estimate_with"](custom,samples,2,2,ctypes.byref(value)) == 3 and value.value == 71
        assert detector["estimate"](custom,None,2,ctypes.byref(value)) == 1 and value.value == 71
        assert detector["estimate"](custom,None,0,ctypes.byref(value)) == 0 and value.value == 0
        assert detector["release"](ctypes.byref(copy)) == 0 and copy.value is None
        assert detector["release"](ctypes.byref(copy)) == 0
        assert detector["release"](ctypes.byref(custom)) == 0 and custom.value is None
        assert detector["release"](ctypes.byref(owner)) == 0 and owner.value is None
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
