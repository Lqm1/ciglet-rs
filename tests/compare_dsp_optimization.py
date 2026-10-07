"""Measure original C optimizer effects without replacing strict DSP fixtures."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import struct
import subprocess
import tempfile


class Reader:
    def __init__(self, content):
        self.content = content
        self.offset = 0

    def take(self, size):
        end = self.offset + size
        if size < 0 or end > len(self.content):
            raise ValueError("Truncated oracle output")
        result = self.content[self.offset:end]
        self.offset = end
        return result

    def integer(self):
        return struct.unpack("<I", self.take(4))[0]

    def scalars(self, width, count):
        return struct.unpack("<" + ("f" if width == 4 else "d") * count,
                             self.take(width * count))


def decode(content, family, width):
    reader = Reader(content)
    signature = b"KLM1" if family == "kalman" else b"FRS1"
    if reader.take(4) != signature or reader.integer() != width:
        raise ValueError("Unexpected oracle format")
    inputs, outputs = [], {}
    if family == "kalman":
        if reader.integer() != 108:
            raise ValueError("Unexpected Kalman case count")
        outputs = {name: [] for name in ("means", "variances", "smoothed", "likelihood")}
        for _ in range(108):
            count = reader.integer()
            if count not in (1, 2, 9, 32):
                raise ValueError("Unexpected Kalman sample count")
            inputs.append((count, reader.take(width * (1 + 3 * count))))
            for name in ("means", "variances", "smoothed"):
                outputs[name].extend(reader.scalars(width, count))
            outputs["likelihood"].extend(reader.scalars(width, 1))
    else:
        counts = tuple(reader.integer() for _ in range(3))
        if counts != (300, 60, 105):
            raise ValueError("Unexpected filter/resample case counts")
        outputs = {name: [] for name in ("fir", "convolution", "resampling")}
        for _ in range(counts[0]):
            count, window, kind = (reader.integer() for _ in range(3))
            if count not in (2, 3, 8, 31, 32):
                raise ValueError("Unexpected FIR tap count")
            inputs.append((count, window, kind, reader.take(2 * width)))
            outputs["fir"].extend(reader.scalars(width, count))
        for _ in range(counts[1]):
            signal, kernel = reader.integer(), reader.integer()
            if signal not in (1, 2, 5, 32, 65) or kernel not in (1, 2, 7, 32):
                raise ValueError("Unexpected convolution dimensions")
            inputs.append((signal, kernel, reader.take(width * (signal + kernel))))
            outputs["convolution"].extend(reader.scalars(width, signal + kernel - 1))
        for _ in range(counts[2]):
            count = reader.integer()
            if count not in (1, 2, 5, 32, 65):
                raise ValueError("Unexpected resampling input length")
            values = reader.take(width * (count + 1))
            length = reader.integer()
            if length > 130:
                raise ValueError("Unexpected resampling output length")
            inputs.append((count, values, length))
            outputs["resampling"].extend(reader.scalars(width, length))
    if reader.offset != len(content):
        raise ValueError("Trailing oracle output")
    return inputs, outputs


def compare(reference, actual, width):
    if len(reference) != len(actual):
        raise ValueError("Output length changed")
    result = {"values": len(reference), "changed_bits": 0,
              "maximum_absolute_error": 0.0, "maximum_normalized_error": 0.0,
              "first_difference": None, "maximum_difference": None}
    encoding = "<f" if width == 4 else "<d"
    for index, (old, new) in enumerate(zip(reference, actual)):
        if not math.isfinite(old) or not math.isfinite(new):
            raise ValueError("Nonfinite DSP output")
        if struct.pack(encoding, old) != struct.pack(encoding, new):
            result["changed_bits"] += 1
            if result["first_difference"] is None:
                result["first_difference"] = {"index": index, "strict": old, "optimized": new}
        error = abs(old - new)
        if error > result["maximum_absolute_error"]:
            result["maximum_difference"] = {"index": index, "strict": old, "optimized": new}
        result["maximum_absolute_error"] = max(result["maximum_absolute_error"], error)
        result["maximum_normalized_error"] = max(
            result["maximum_normalized_error"], error / max(1.0, abs(old)))
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--cc", nargs="+", default=["gcc"])
    parser.add_argument("--target", help="optional Clang/Zig target triple")
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    if args.target:
        args.cc += ["-target", args.target]
    upstream = args.upstream.resolve()
    tests = Path(__file__).resolve().parent
    profiles = {"strict": ["-O2"], "ofast": ["-Ofast"],
                "fast_math": ["-O3", "-ffast-math"]}
    report = {"compiler": subprocess.run(args.cc + ["--version"], check=True,
              capture_output=True, text=True).stdout.splitlines()[0],
              "source_sha256": hashlib.sha256((upstream / "ciglet.c").read_bytes()).hexdigest(),
              "contraction": "disabled", "profiles": {}, "results": []}
    with tempfile.TemporaryDirectory(prefix="ciglet-dsp-optimization-") as temporary:
        root = Path(temporary)
        for name, flags in profiles.items():
            macros = subprocess.run(args.cc + flags + ["-ffp-contract=off", "-dM", "-E", "-x", "c", "-"],
                                    input="", check=True, capture_output=True, text=True).stdout
            selected = [line for line in macros.splitlines()
                        if line.startswith(("#define __FAST_MATH__ ", "#define __FINITE_MATH_ONLY__ "))]
            report["profiles"][name] = {"flags": flags, "macros": selected}
            if name == "fast_math" and ("#define __FAST_MATH__ 1" not in selected
                                        or "#define __FINITE_MATH_ONLY__ 1" not in selected):
                raise ValueError("Explicit fast-math flags are inactive")
        for family, driver in (("kalman", "kalman_oracle.c"),
                               ("filter_resample", "filter_resample_oracle.c")):
            for precision, width in (("float", 4), ("double", 8)):
                filename = (f"kalman-{precision}.bin" if family == "kalman"
                            else f"c-filter-resample-f{width * 8}.bin")
                original = (tests / "fixtures" / filename).read_bytes()
                original_inputs, original_outputs = decode(original, family, width)
                strict_outputs = None
                for profile, flags in profiles.items():
                    executable = root / (family + "-" + precision + "-" + profile)
                    output = executable.with_suffix(".bin")
                    subprocess.run(args.cc + flags + ["-ffp-contract=off", "-DFP_TYPE=" + precision,
                        "-ffunction-sections", "-fdata-sections", "-I" + str(upstream),
                        str(tests / "reference" / driver), str(upstream / "ciglet.c"),
                        str(upstream / "external/fftsg_h.c"),
                        str(upstream / "external/fast_median.c"),
                        "-Wl,--gc-sections", "-lm", "-o", str(executable)], check=True)
                    subprocess.run([str(executable), str(output)], check=True)
                    content = output.read_bytes()
                    inputs, outputs = decode(content, family, width)
                    if inputs != original_inputs:
                        raise ValueError("Inputs or discrete dimensions changed")
                    baseline = None
                    if profile == "strict":
                        baseline = {name: compare(original_outputs[name], values, width)
                                    for name, values in outputs.items()}
                        # Existing Rust tests use exact Kalman values and a 2e-15
                        # normalized bound for binary64 FIR/resampling libm variation.
                        for metrics in baseline.values():
                            limit = 2e-15 if family == "filter_resample" and width == 8 else 0.0
                            if metrics["maximum_normalized_error"] > limit:
                                raise ValueError(f"Strict reference changed: {family}/{precision}")
                        strict_outputs = outputs
                    report["results"].append({"family": family, "precision": precision,
                        "profile": profile, "input_bytes_and_dimensions_identical": True,
                        "strict_fixture_comparison": baseline,
                        "outputs": {name: compare(strict_outputs[name], values, width)
                                    for name, values in outputs.items()}})
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"Verified strict fixtures and recorded twelve DSP builds in {args.report}")


if __name__ == "__main__":
    main()
