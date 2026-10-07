"""Generate scalar comparisons and all trig precision levels from the pinned header."""
import argparse
import os
from pathlib import Path
import struct
import subprocess
import tempfile

def verify(expected, actual, width):
    if expected == actual:
        return
    if width != 8 or len(expected) != len(actual):
        raise ValueError('Scalar reference bytes changed')
    mutable = bytearray(actual)
    for case in range(64):
        for field in [7, 8, 10, 11]:
            offset = 12 + (case * 12 + field) * 8
            a = struct.unpack_from('<d', expected, offset)[0]
            b = struct.unpack_from('<d', actual, offset)[0]
            bits_a = struct.unpack_from('<Q', expected, offset)[0]
            bits_b = struct.unpack_from('<Q', actual, offset)[0]
            if a == 0.0 or b == 0.0:
                if bits_a != bits_b:
                    raise ValueError('Host libm zero sign changed')
            elif abs(bits_a - bits_b) > 1 or abs(a - b) / (1 + abs(a)) > 2.220446049250313e-16:
                raise ValueError('Host libm change exceeds one ULP')
            mutable[offset:offset + 8] = expected[offset:offset + 8]
    if expected != mutable:
        raise ValueError('Non-libm scalar reference bytes changed')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='scalar-math-reference-') as directory:
        root = Path(directory)
        (root / 'external').mkdir()
        for name in ['ciglet.h', 'external/fastapprox-all.h']:
            (root / name).write_bytes((args.upstream / name).read_bytes())
        (root / 'oracle.c').write_bytes((tests / 'reference/scalar_math_oracle.c').read_bytes())
        for precision, width in [('float', 4), ('double', 8)]:
            executable = root / (precision + ('.exe' if os.name == 'nt' else ''))
            output = root / (precision + '.bin')
            compiler = [args.cc, 'cc', '-target', 'x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler + ['-O2', '-DFP_TYPE=' + precision, '-I' + str(root), str(root / 'oracle.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            target = tests / 'fixtures' / ('scalar-math-' + precision + '.bin')
            if args.check:
                verify(target.read_bytes(), output.read_bytes(), width)
            else:
                target.write_bytes(output.read_bytes())
            print(precision, 'unchanged header scalar reference verified')

if __name__ == '__main__':
    main()
