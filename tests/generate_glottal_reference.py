"""Generate glottal references from the unchanged pinned C library."""
import argparse
import os
import struct
from pathlib import Path
import shutil
import subprocess
import tempfile

def compare_double(expected, actual):
    if len(expected) != len(actual) or expected[:12] != actual[:12]:
        raise ValueError('C glottal reference structure changed')
    cases, = struct.unpack_from('<I',expected,8)
    cursor=12
    maxima=[0.0,0.0,0.0]
    def exact(size):
        nonlocal cursor
        if expected[cursor:cursor+size] != actual[cursor:cursor+size]:
            raise ValueError('C glottal inputs, model fields or dimensions changed')
        cursor+=size
    for case in range(cases):
        exact(8*8)
        rate,count,nf=struct.unpack_from('<III',expected,cursor)
        exact(12+8*nf)
        for field,length in enumerate([nf,nf,count]):
            for _ in range(length):
                first,=struct.unpack_from('<d',expected,cursor)
                second,=struct.unpack_from('<d',actual,cursor)
                error=abs(first-second)/(1+abs(first))
                if first == 0.0:
                    exact(8)
                else:
                    cursor+=8
                maxima[field]=max(maxima[field],error)
                if not error <= [1e-17,3e-15,5e-16][field]:
                    raise ValueError(f'C glottal output changed: case={case} field={field} error={error}')
    if cursor != len(expected): raise ValueError('C glottal reference trailing data')
    print('double cross-host maximum magnitude/phase/wave normalized errors',maxima)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='glottal-') as directory:
        root = Path(directory)
        for name in ['ciglet.h', 'ciglet.c']:
            (root / name).write_bytes((args.upstream / name).read_bytes())
        shutil.copytree(args.upstream / 'external', root / 'external')
        (root / 'oracle.c').write_bytes((tests / 'reference/glottal_oracle.c').read_bytes())
        for precision in ['float', 'double']:
            executable = root / (precision + ('.exe' if os.name == 'nt' else ''))
            output = root / (precision + '.bin')
            compiler = [args.cc, 'cc', '-target', 'x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler + ['-O2', '-DFP_TYPE=' + precision, '-I' + str(root), str(root / 'oracle.c'), str(root / 'ciglet.c'), str(root / 'external/fftsg_h.c'), str(root / 'external/fast_median.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            target = tests / 'fixtures' / ('glottal-' + precision + '.bin')
            if args.check:
                if precision == 'double': compare_double(target.read_bytes(),output.read_bytes())
                elif target.read_bytes() != output.read_bytes():
                    raise ValueError('C glottal reference changed: ' + precision)
            else:
                target.write_bytes(output.read_bytes())
            print(precision, 'unchanged C glottal reference verified')

if __name__ == '__main__':
    main()
