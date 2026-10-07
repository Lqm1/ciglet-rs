"""Generate four-lane SSE2 hyperbolic and sigmoid results from the bundled C header."""
import argparse
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
    compiler = [args.cc] + (['cc', '-target', 'x86_64-windows-gnu'] if args.zig else [])
    with tempfile.TemporaryDirectory(prefix='lanes-reference-') as directory:
        executable = Path(directory)/('oracle.exe' if args.zig else 'oracle')
        subprocess.run(compiler+['-std=c99', '-O2', '-ffp-contract=off', '-I', str(args.upstream),
            str(tests/'reference/lanes_hyperbolic_oracle.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
        content = subprocess.check_output([str(executable)], timeout=30).replace(b'\r\n', b'\n')
        if len(content.splitlines()) != 48:
            raise ValueError('Expected 48 four-lane reference records')
        fixture = tests/'fixtures/c-lanes-hyperbolic.txt'
        if args.check:
            if fixture.read_bytes() != content:
                raise ValueError('Bundled lanes outputs changed')
        else:
            fixture.write_bytes(content)
        print('48 bundled C SSE2 lane records verified')


if __name__ == '__main__':
    main()
