"""Capture unchanged C and derive independent exact-rational population moments."""
import argparse
from decimal import Decimal, localcontext
from fractions import Fraction
import os
from pathlib import Path
import struct
import subprocess
import tempfile

def rounded_bits(value, width):
    """Round a rational directly to binary32/64, ties to even, without double rounding."""
    precision, minimum, maximum = (24, -126, 127) if width == 4 else (53, -1022, 1023)
    sign = int(value < 0) << (width * 8 - 1)
    value = abs(value)
    if not value:
        return sign
    numerator, denominator = value.numerator, value.denominator
    exponent = numerator.bit_length() - denominator.bit_length()
    if exponent >= 0:
        exponent -= numerator < (denominator << exponent)
    else:
        exponent -= (numerator << -exponent) < denominator
    quantum = max(exponent, minimum) - precision + 1
    if quantum < 0:
        numerator <<= -quantum
    else:
        denominator <<= quantum
    integer, remainder = divmod(numerator, denominator)
    if 2 * remainder > denominator or 2 * remainder == denominator and integer % 2:
        integer += 1
    if integer >= 1 << precision:
        integer >>= 1
        quantum += 1
    if integer < 1 << (precision - 1):
        return sign | integer
    exponent = quantum + precision - 1
    if exponent > maximum:
        return sign | ((2 * maximum + 1) << (precision - 1))
    return sign | ((exponent - minimum + 1) << (precision - 1)) | (integer - (1 << (precision - 1)))

def reference(content):
    assert content[:4] == b'SMC1'
    width, count = struct.unpack_from('<II', content, 4)
    scalar_format = '<f' if width == 4 else '<d'
    cursor = 12
    output = bytearray(b'SMT1' + struct.pack('<II', width, count))
    for _ in range(count):
        n, = struct.unpack_from('<I', content, cursor)
        cursor += 4
        begin = cursor - 4
        x = [Fraction(struct.unpack_from(scalar_format, content, cursor + width*i)[0]) for i in range(n)]
        cursor += n * width
        y = [Fraction(struct.unpack_from(scalar_format, content, cursor + width*i)[0]) for i in range(n)]
        cursor += n * width
        cursor += 4 * (4 + width)
        output.extend(content[begin:cursor])
        mx, my = sum(x) / n, sum(y) / n
        vx = sum((v-mx)**2 for v in x) / n
        vy = sum((v-my)**2 for v in y) / n
        covariance = sum((a-mx)*(b-my) for a,b in zip(x,y)) / n
        values = [mx, vx, covariance]
        if vx and vy:
            with localcontext() as context:
                context.prec = 220
                product = vx * vy
                divisor = (Decimal(product.numerator) / Decimal(product.denominator)).sqrt()
                correlation = Decimal(covariance.numerator) / Decimal(covariance.denominator) / divisor
                values.append(Fraction(correlation))
        else:
            values.append(None)
        for value in values:
            bits = rounded_bits(value, width) if value is not None else 0
            raw = bits.to_bytes(width, 'little')
            numeric = struct.unpack(scalar_format, raw)[0]
            valid = value is not None and abs(numeric) != float('inf')
            output.extend(struct.pack('<I', int(valid)))
            output.extend(raw if valid else bytes(width))
    assert cursor == len(content)
    return output

def verify_rounding_boundaries():
    for width, precision, minimum, maximum, bias in [(4, 24, -126, 127, 127), (8, 53, -1022, 1023, 1023)]:
        one = bias << (precision - 1)
        assert rounded_bits(Fraction(1) + Fraction(1, 1 << precision), width) == one
        assert rounded_bits(Fraction(1) + Fraction(3, 1 << precision), width) == one + 2
        assert rounded_bits(-Fraction(1) - Fraction(1, 1 << precision), width) == (1 << (width*8-1)) | one
        tiny = Fraction(1, 1 << (precision - 1 - minimum))
        assert rounded_bits(tiny / 2, width) == 0
        assert rounded_bits(3 * tiny / 2, width) == 2
        assert rounded_bits(((1 << (precision - 1)) - Fraction(1, 2)) * tiny, width) == 1 << (precision - 1)
        quantum = Fraction(1 << (maximum - precision + 1))
        largest = ((1 << precision) - 1) * quantum
        infinity = (2 * maximum + 1) << (precision - 1)
        assert rounded_bits(largest + quantum / 4, width) == infinity - 1
        assert rounded_bits(largest + quantum / 2, width) == infinity

def main():
    verify_rounding_boundaries()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--upstream', type=Path, required=True)
    parser.add_argument('--cc', default='gcc')
    parser.add_argument('--zig', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix='stable-statistics-') as directory:
        root = Path(directory)
        (root / 'external').mkdir()
        for name in ['ciglet.h', 'external/fastapprox-all.h']:
            (root / name).write_bytes((args.upstream / name).read_bytes())
        (root / 'oracle.c').write_bytes((tests / 'reference/stable_statistics_oracle.c').read_bytes())
        for precision in ['float', 'double']:
            executable = root / (precision + ('.exe' if os.name == 'nt' else ''))
            output = root / (precision + '.bin')
            compiler = [args.cc, 'cc', '-target', 'x86_64-windows-gnu'] if args.zig else [args.cc]
            subprocess.run(compiler + ['-O2', '-DFP_TYPE=' + precision, '-I' + str(root), str(root / 'oracle.c'), '-lm', '-o', str(executable)], check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            target = tests / 'fixtures' / ('stable_statistics-' + precision + '.bin')
            content = reference(output.read_bytes())
            if args.check:
                if target.read_bytes() != content:
                    raise ValueError('Stable statistics reference changed: ' + precision)
            else:
                target.write_bytes(content)
            print(precision, 'C arithmetic and exact-rational moment reference verified')

if __name__ == '__main__':
    main()
