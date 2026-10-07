"""Solve regularized Toeplitz systems by exact-rational Gaussian elimination."""
import argparse
from fractions import Fraction
from pathlib import Path
import struct

from generate_stable_statistics_reference import rounded_bits, verify_rounding_boundaries


def solve(correlation):
    order = len(correlation) - 1
    regularizer = Fraction(1.000001)
    rows = [
        [correlation[abs(row-column)] * (regularizer if row == column else 1)
         for column in range(order)] + [-correlation[row+1]]
        for row in range(order)
    ]
    for column in range(order):
        pivot = next(row for row in range(column, order) if rows[row][column])
        rows[column], rows[pivot] = rows[pivot], rows[column]
        divisor = rows[column][column]
        rows[column] = [value/divisor for value in rows[column]]
        for row in range(order):
            if row != column:
                factor = rows[row][column]
                rows[row] = [value-factor*reference for value, reference in zip(rows[row], rows[column])]
    return [Fraction(1)] + [row[-1] for row in rows]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    verify_rounding_boundaries()
    directory = Path(__file__).resolve().parent/'fixtures'
    for precision, width in [('float', 4), ('double', 8)]:
        fmt = '<f' if width == 4 else '<d'
        def cast(value):
            return struct.unpack(fmt, struct.pack(fmt, value))[0]
        largest = struct.unpack(fmt, (0x7f7fffff if width == 4 else 0x7fefffffffffffff).to_bytes(width, 'little'))[0]
        tiny = struct.unpack(fmt, (1).to_bytes(width, 'little'))[0]
        scales = [1.0, largest, tiny*16, 2.0**(-60 if width == 4 else -500)]
        cases = []
        for order in [0, 1, 2, 4, 8]:
            for scale in scales:
                for ratio in [0.0, 0.5, -0.5, 0.875, 0.999999]:
                    correlation = [cast(scale * ratio**lag) for lag in range(order+1)]
                    cases.append((correlation, solve([Fraction(value) for value in correlation])))
        assert len(cases) == 100
        result = bytearray(b'SLP1' + struct.pack('<II', width, len(cases)))
        values = 0
        for correlation, coefficients in cases:
            result.extend(struct.pack('<I', len(correlation)))
            result.extend(b''.join(struct.pack(fmt, value) for value in correlation))
            result.extend(b''.join(rounded_bits(value, width).to_bytes(width, 'little') for value in coefficients))
            values += len(coefficients)
        target = directory/('stable-prediction-'+precision+'.bin')
        if args.check:
            if target.read_bytes() != result:
                raise ValueError('Stable prediction reference changed: '+precision)
        else:
            target.write_bytes(result)
        print(precision, len(cases), 'exact linear systems,', values, 'coefficient outputs')


if __name__ == '__main__':
    main()
