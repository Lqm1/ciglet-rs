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
    print("C ABI Python smoke test passed")


if __name__ == "__main__":
    main()
