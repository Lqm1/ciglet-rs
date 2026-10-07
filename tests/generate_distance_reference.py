"""Generate log-Itakura-Saito references from the unchanged pinned header."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='distance-reference-') as directory:
        root = Path(directory)
        (root / 'external').mkdir()
        for name in ['ciglet.h', 'external/fastapprox-all.h']:
            (root / name).write_bytes((args.upstream / name).read_bytes())
        (root / 'oracle.c').write_bytes((tests / 'reference/distance_oracle.c').read_bytes())
        for precision in ['float', 'double']:
            executable = root / (precision + ('.exe' if os.name == 'nt' else ''))
            output = root / (precision + '.bin')
            compiler = [args.cc, 'cc', '-target', 'x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler + ['-O2', '-DFP_TYPE=' + precision, '-I' + str(root), str(root / 'oracle.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            target = tests / 'fixtures' / ('distance-' + precision + '.bin')
            if args.check:
                if target.read_bytes() != output.read_bytes():
                    raise ValueError('C distance reference changed: ' + precision)
            else:
                target.write_bytes(output.read_bytes())
            print(precision, 'unchanged header distance reference verified')

if __name__ == '__main__':
    main()
