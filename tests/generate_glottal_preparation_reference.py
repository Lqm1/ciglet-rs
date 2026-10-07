"""Inspect preparation and residuals in the unchanged pinned LF implementation."""
import argparse
import math
import os
from pathlib import Path
import struct
import subprocess
import tempfile


def describe(payload):
    magic, width, count = struct.unpack_from('<4sII', payload)
    if magic != b'LFP1' or width not in (4, 8) or count != 208:
        raise ValueError('Unexpected LF preparation header')
    if len(payload) != 12 + count * 20 * width:
        raise ValueError('Unexpected LF preparation size')
    records = list(struct.iter_unpack('<' + ('f' if width == 4 else 'd') * 20, payload[12:]))
    finite = [row for row in records if all(math.isfinite(x) for x in row)]
    print('width', width, 'finite records', len(finite), '/', count)
    print('maximum absolute decay residual', max(abs(row[17]) for row in finite))
    print('maximum absolute growth residual', max(abs(row[18]) for row in finite))
    print('maximum scaled growth residual', max(abs(row[18]) / residual_scale(row, 18) for row in finite))
    return records


def residual_scale(row, field):
    if field == 17:
        return 1 + abs(math.exp((row[6] - row[5]) * row[12])) + abs(row[8] * row[12])
    sine, integral, growth, omega, closure, cosine = row[10], row[13], row[14], row[9], row[6], row[11]
    exponential = math.exp(-growth * closure)
    if field == 18:
        return 1 + abs(sine * integral * growth * growth) + abs(sine * growth) + abs(omega * exponential) + abs(omega * omega * sine * integral) + abs(omega * cosine)
    return 1 + abs(2 * sine * integral * growth) + abs(sine) + abs(omega * closure * exponential)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='glottal-preparation-') as directory:
        root = Path(directory)
        source = (args.upstream / 'ciglet.c').read_text()
        endpoint = '  if(fa * fb >= 0) return (a + b) / 2.0;'
        if source.count(endpoint) != 1:
            raise ValueError('Cannot locate unique original root fallback')
        corrected = source.replace(endpoint, '  if(fa == 0) return a;\n  if(fb == 0) return b;\n' + endpoint)
        for precision, policy in [(p, policy) for p in ['float', 'double'] for policy in ['original', 'endpoints']]:
            (root / 'ciglet.c').write_text(source if policy == 'original' else corrected)
            executable = root / (precision + ('.exe' if os.name == 'nt' else ''))
            output = root / (precision + '.bin')
            compiler = [args.cc, 'cc', '-target', 'x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler + ['-O2', '-ffp-contract=off', '-DFP_TYPE=' + precision,
                '-I' + str(root), '-I' + str(args.upstream), str(tests / 'reference/glottal_preparation_oracle.c'),
                str(args.upstream / 'external/fftsg_h.c'), str(args.upstream / 'external/fast_median.c'),
                '-lm', '-o', str(executable)], check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            payload = output.read_bytes()
            records = describe(payload)
            target = tests / 'fixtures' / ('glottal-preparation-' + policy + '-' + precision + '.bin')
            if args.check:
                expected = describe(target.read_bytes())
                maximum = 0.0
                for case, (first, second) in enumerate(zip(expected, records)):
                    for field, (a, b) in enumerate(zip(first, second)):
                        if not math.isfinite(a) or not math.isfinite(b):
                            if math.isnan(a) != math.isnan(b) or (not math.isnan(a) and a != b):
                                raise ValueError(f'Nonfinite classification changed: {case}/{field}')
                        else:
                            scale = residual_scale(first, field) if field >= 17 else 1 + abs(a)
                            error = abs(a - b) / scale
                            maximum = max(maximum, error)
                            if error > (0 if precision == 'float' and field < 17 else 2e-14):
                                raise ValueError(f'Preparation changed: {case}/{field}, {error}')
                print('cross-host maximum normalized difference', maximum)
            else:
                target.write_bytes(payload)


if __name__ == '__main__':
    main()
