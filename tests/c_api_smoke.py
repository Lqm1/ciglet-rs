"""Exercise the real shared library through Python's standard-library ctypes."""

import ctypes
import pathlib
import struct
import sys


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
