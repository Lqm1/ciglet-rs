"""Generate independent high-precision real W(exp(x)) reference values."""
from decimal import Decimal, localcontext
from pathlib import Path
import argparse
import struct
import subprocess
import tempfile


def bits(value):
    return struct.unpack('<I', struct.pack('<f', value))[0]


def scalar(pattern):
    return struct.unpack('<f', struct.pack('<I', pattern))[0]


def root(value, precision):
    with localcontext() as context:
        context.prec = precision
        x = Decimal.from_float(value)
        if x < -200:
            return Decimal(0)  # exp(-200) is already far below half a binary32 subnormal.
        if x < 0:
            target = x.exp()
            w = target
            for _ in range(80):
                exponential = w.exp()
                updated = w - (w * exponential - target) / (exponential * (1 + w))
                if updated == w:
                    break
                w = updated
        else:
            w = x if x > 1 else Decimal(1)
            for _ in range(80):
                updated = w - (w + w.ln() - x) / (1 + 1 / w)
                if updated == w:
                    break
                w = updated
        residual = abs(w * w.exp() - target) / target if x < 0 else abs(w + w.ln() - x) / max(Decimal(1), abs(x))
        if residual > Decimal(10) ** (-(precision - 5)):
            raise ValueError('High-precision root did not converge')
        return +w


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    args = parser.parse_args()
    # Dense corrected tail, adjacent cutoff/underflow values, and overflow inputs.
    inputs = {bits(-120 + index / 4) for index in range(465)}
    for value in [-16.0, -103.97208, -87.33655]:
        pattern = bits(value)
        inputs.update([pattern - 1, pattern, pattern + 1])
    inputs.update(bits(value) for value in [-200, -1000, -scalar(0x7f7fffff),
        1e12, 1e13, 1e20, 1e30, scalar(0x7f7fffff)])
    records = []
    for pattern in sorted(inputs):
        value = scalar(pattern)
        first = root(value, 80)
        expected = bits(float(first))
        if bits(float(root(value, 120))) != expected:
            raise ValueError('Precision-dependent rounded reference')
        if first != 0:
            with localcontext() as context:
                context.prec = 150
                lower = Decimal(0) if expected == 0 else Decimal.from_float(scalar(expected - 1))
                center = Decimal.from_float(scalar(expected))
                upper = Decimal.from_float(scalar(expected + 1)) if expected < 0x7f7fffff else center * 2 - lower
                if not (lower + center) / 2 <= first <= (center + upper) / 2:
                    raise ValueError('Reference is not nearest binary32 rounding')
        records.append(f'{pattern:08x} {expected:08x}\n')
    output = ''.join(records).encode()
    tests = Path(__file__).resolve().parent
    compiler = [args.cc] + (['cc', '-target', 'x86_64-windows-gnu'] if args.zig else [])
    with tempfile.TemporaryDirectory(prefix='lambert-exp-reference-') as directory:
        executable = Path(directory) / ('oracle.exe' if args.zig else 'oracle')
        subprocess.run(compiler + ['-std=c99', '-O2', '-ffp-contract=off',
            '-I', str(args.upstream), str(tests / 'reference/lambert_exp_oracle.c'),
            '-lm', '-o', str(executable)], check=True, timeout=120)
        source = subprocess.check_output([str(executable)],
            input=''.join(f'{pattern:08x}\n' for pattern in sorted(inputs)).encode(),
            timeout=30).replace(b'\r\n', b'\n')
    if len(source.splitlines()) != len(records):
        raise ValueError('Incomplete source comparison')
    for filename, content in [('lambert-exp-corrected.txt', output),
            ('c-lambert-exp.txt', source)]:
        path = tests / 'fixtures' / filename
        if args.check:
            if path.read_bytes() != content:
                raise ValueError(f'Lambert reference changed: {filename}')
        else:
            path.write_bytes(content)
    print(f'{len(records)} independently rounded Lambert exponential records verified')


if __name__ == '__main__':
    main()
