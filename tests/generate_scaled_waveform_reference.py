"""Derive scaled waveform correlations and coefficients from exact rational inputs."""
import argparse
from fractions import Fraction
from pathlib import Path
import struct

from generate_stable_prediction_reference import solve
from generate_stable_statistics_reference import rounded_bits, verify_rounding_boundaries


def parts(value):
    if not value:
        return struct.pack('<di', 0.0, 0)
    magnitude = abs(value)
    exponent = magnitude.numerator.bit_length() - magnitude.denominator.bit_length()
    power = Fraction(2)**exponent
    if magnitude < power:
        exponent -= 1
        power /= 2
    fraction = struct.unpack('<d', rounded_bits(value/power, 8).to_bytes(8, 'little'))[0]
    if abs(fraction) == 2:
        fraction /= 2
        exponent += 1
    return struct.pack('<di', fraction, exponent)


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
        result = bytearray(b'SWV1' + struct.pack('<II', width, 64))
        outputs = 0
        for length in [1, 2, 3, 9]:
            order = min(length-1, 4)
            for scale in [1.0, largest, tiny, 2.0**(-60 if width == 4 else -500)]:
                for family in range(4):
                    samples = [cast(scale * [1.0, (-1.0)**i, float(i == length//2), (i-length//2)/length][family]) for i in range(length)]
                    exact = [Fraction(value) for value in samples]
                    correlation = [sum(left*right for left, right in zip(exact, exact[lag:])) for lag in range(order+1)]
                    coefficients = solve(correlation) if correlation[0] else [Fraction(1)] + [Fraction(0)]*order
                    result.extend(struct.pack('<II', length, order))
                    result.extend(b''.join(struct.pack(fmt, value) for value in samples))
                    result.extend(b''.join(parts(value) for value in correlation))
                    result.extend(b''.join(rounded_bits(value, width).to_bytes(width, 'little') for value in coefficients))
                    outputs += len(correlation)
        assert outputs == 176
        target = directory/('scaled-waveform-'+precision+'.bin')
        if args.check:
            if target.read_bytes() != result:
                raise ValueError('Scaled waveform reference changed: '+precision)
        else:
            target.write_bytes(result)
        print(precision, 64, 'waveforms,', outputs, 'scaled correlations and coefficients')


if __name__ == '__main__':
    main()
