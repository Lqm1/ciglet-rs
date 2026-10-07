"""Generate scalar special-function results from the bundled C header."""
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
    with tempfile.TemporaryDirectory(prefix='special-reference-') as directory:
        executable = Path(directory)/('oracle.exe' if args.zig else 'oracle')
        subprocess.run(compiler+['-std=c99', '-O2', '-ffp-contract=off', '-I', str(args.upstream),
            str(tests/'reference/special_oracle.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
        content = subprocess.check_output([str(executable)], timeout=30).replace(b'\r\n', b'\n')
        if len(content.splitlines()) != 232:
            raise ValueError('Expected 232 scalar reference records')
        fixture = tests/'fixtures/c-special.txt'
        if args.check:
            if fixture.read_bytes() != content:
                raise ValueError('Bundled special outputs changed')
        else:
            fixture.write_bytes(content)
        print('232 bundled C special functions records verified')


if __name__ == '__main__':
    main()
