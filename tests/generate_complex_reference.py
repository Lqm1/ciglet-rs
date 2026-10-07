"""Generate original C complex and approximation references."""
import argparse
import os
from pathlib import Path
import subprocess
import math
import struct
import tempfile

def verify(expected, actual, precision):
    if expected == actual:
        return 0.0
    # The measured GCC/Zig difference is confined to double scalar exp.
    if precision != 'double' or len(expected) != len(actual) or expected[:12] != actual[:12]:
        raise ValueError('C complex reference layout or float results changed')
    width, count = struct.unpack_from('<II', expected, 4)
    if width != 8 or count != 64:
        raise ValueError('C complex reference header changed')
    cursor = 12 + count * 22 * width
    if expected[:cursor+4] != actual[:cursor+4]:
        raise ValueError('C complex inputs or arithmetic changed')
    scalars = struct.unpack_from('<I', expected, cursor)[0]
    cursor += 4
    maximum = 0.0
    for case in range(scalars):
        for field in range(10):
            left = expected[cursor:cursor+width]
            right = actual[cursor:cursor+width]
            if left != right:
                if field != 7:
                    raise ValueError(f'C scalar reference changed: case {case}, field {field}')
                a = struct.unpack('<d',left)[0]
                b = struct.unpack('<d',right)[0]
                error = abs(a-b)/(1+abs(a))
                distance = abs(int.from_bytes(left,'little')-int.from_bytes(right,'little'))
                if not math.isfinite(error) or error > 2.220446049250313e-16 or distance > 1 or a == 0 or b == 0:
                    raise ValueError('C scalar exp exceeded the measured comparison bound')
                maximum = max(maximum,error)
            cursor += width
    if cursor != len(expected):
        raise ValueError('C complex reference payload length changed')
    return maximum

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='complex-reference-') as temporary:
        root = Path(temporary)
        (root/'external').mkdir()
        for name in ['ciglet.h', 'external/fastapprox-all.h']:
            (root/name).write_bytes((args.upstream/name).read_bytes())
        (root/'oracle.c').write_bytes((tests/'reference/complex_oracle.c').read_bytes())
        (root/'bounds.c').write_bytes((tests/'reference/complex_bounds_oracle.c').read_bytes())
        for precision in ['float', 'double']:
            executable = root/(precision+('.exe' if os.name == 'nt' else ''))
            output = root/(precision+'.bin')
            compiler = [args.cc,'cc','-target','x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-I'+str(root),str(root/'oracle.c'),'-lm','-o',str(executable)],check=True,timeout=120)
            subprocess.run([str(executable),str(output)],check=True,timeout=30)
            target = (args.output or tests/'fixtures')/('complex-'+precision+'.bin')
            content = output.read_bytes()
            if args.check:
                difference = verify(target.read_bytes(),content,precision)
                print(precision,'scalar exp maximum normalized C platform difference',difference)
            else:
                target.write_bytes(content)
            print(precision,'C complex reference verified')
            bounds = root/('bounds-'+precision+('.exe' if os.name == 'nt' else ''))
            subprocess.run(compiler+['-O2','-DFP_TYPE='+precision,'-I'+str(root),str(root/'bounds.c'),'-lm','-o',str(bounds)],check=True,timeout=120)
            subprocess.run([str(bounds)],check=True,timeout=30)

if __name__ == '__main__':
    main()
