"""Derive box integrals and RMS from exact rational sample-cell overlaps."""
import argparse
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path
import struct
from generate_stable_statistics_reference import rounded_bits, verify_rounding_boundaries

def windows(signal, width):
    if not width:
        return signal
    output = []
    for center in range(len(signal)):
        lower, upper = center-width, center+width
        total = Fraction(0)
        if len(signal) == 1:
            total = signal[0] * (upper-lower)
        else:
            total += signal[0] * max(Fraction(0), min(upper, 0)-lower)
            for index in range(1, len(signal)-1):
                overlap = min(upper, index) - max(lower, index-1)
                if overlap > 0:
                    total += signal[index] * overlap
            total += signal[-1] * max(Fraction(0), upper-max(lower, len(signal)-2))
        output.append(total / (2*width))
    return output

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    verify_rounding_boundaries()
    tests = Path(__file__).resolve().parent
    for precision, width in [('float', 4), ('double', 8)]:
        fmt = '<f' if width == 4 else '<d'
        def scalar(data, at):
            return struct.unpack_from(fmt, data, at)[0]
        def cast(value):
            return struct.unpack(fmt, struct.pack(fmt, value))[0]
        original = (tests/'fixtures'/('smoothing-'+precision+'.bin')).read_bytes()
        assert original[:4] == b'SMO1'
        actual_width, count = struct.unpack_from('<II', original, 4)
        assert actual_width == width and count == 60
        at, cases = 12, []
        for _ in range(count):
            n, = struct.unpack_from('<I', original, at)
            at += 4
            half = scalar(original, at)
            at += width
            signal = [scalar(original, at+i*width) for i in range(n)]
            at += 3*n*width
            cases.append((signal, half, int(half)+1))
        assert at == len(original)
        largest = scalar(((0x7f7fffff if width==4 else 0x7fefffffffffffff).to_bytes(width, 'little')), 0)
        tiny = scalar((1).to_bytes(width, 'little'), 0)
        offset = 2.0**(20 if width==4 else 50)
        for n in [1, 2, 3, 9, 65]:
            for half in [0.0, tiny, 0.25, 0.75, 1.0, 1.5, 7.25, 16.5, 2.0**30+0.5]:
                half = cast(half)
                for kind in range(6):
                    signal = []
                    for i in range(n):
                        value = [largest, -largest, largest if i%2 else -largest,
                                 (i%5-2)*tiny, offset+(i%7-3)*0.5,
                                 1.0 if i==n//2 else 0.0][kind]
                        signal.append(cast(value))
                    cases.append((signal, half, int(half)))
        assert len(cases) == 330
        output = bytearray(b'SSW1'+struct.pack('<II',width,len(cases)))
        total_values = 0
        for signal, half, rms_half in cases:
            output.extend(struct.pack('<II', len(signal), rms_half))
            output.extend(struct.pack(fmt, half))
            output.extend(b''.join(struct.pack(fmt, value) for value in signal))
            exact = [Fraction(value) for value in signal]
            means = windows(exact, Fraction(half))
            mean_squares = windows([value*value for value in exact], Fraction(rms_half))
            for value in means:
                output.extend(rounded_bits(value,width).to_bytes(width,'little'))
            for value in mean_squares:
                with localcontext() as context:
                    context.prec = 220
                    root = (Decimal(value.numerator)/Decimal(value.denominator)).sqrt()
                    output.extend(rounded_bits(Fraction(root),width).to_bytes(width,'little'))
            total_values += 2*len(signal)
        assert total_values == 10920
        target = tests/'fixtures'/('stable_smoothing-'+precision+'.bin')
        if args.check:
            if target.read_bytes() != output:
                raise ValueError('Stable smoothing reference changed: '+precision)
        else:
            target.write_bytes(output)
        print(precision, len(cases), 'box-integral cases,', total_values, 'reference outputs')

if __name__ == '__main__':
    main()
