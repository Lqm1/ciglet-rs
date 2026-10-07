"""Capture unchanged POSIX C plotting commands in the default C locale."""
import argparse
from pathlib import Path
import subprocess
import tempfile

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--target-32', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='plotting-reference-') as directory:
        root = Path(directory)
        (root / 'external').mkdir()
        for name in ['ciglet.h', 'external/fastapprox-all.h']:
            (root / name).write_bytes((args.upstream / name).read_bytes())
        (root / 'oracle.c').write_bytes((tests / 'reference/plotting_oracle.c').read_bytes())
        for precision in ['float', 'double']:
            executable = root / precision
            output = root / (precision + '.txt')
            target_flags = ['-m32'] if args.target_32 else []
            subprocess.run([args.cc, *target_flags, '-O2', '-DFP_TYPE=' + precision,
                '-I' + str(root), str(root / 'oracle.c'), '-lm', '-o', str(executable)],
                check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            target = tests / 'fixtures' / ('plotting-' + precision + '.txt')
            if args.check:
                if target.read_bytes() != output.read_bytes():
                    raise ValueError('C plotting reference changed: ' + precision)
            else:
                target.write_bytes(output.read_bytes())
            print(precision, 'unchanged POSIX plotting reference verified', flush=True)

if __name__ == '__main__':
    main()
