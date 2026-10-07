"""Generate original-C Kalman filter and smoother references for both precisions."""
import argparse
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--cc", default="gcc")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    upstream = args.upstream.resolve()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="kalman-reference-") as temporary:
        root = Path(temporary)
        for precision in ["float", "double"]:
            executable = root / ("oracle-" + precision)
            output = root / (precision + ".bin")
            subprocess.run([
                args.cc, "-O2", "-DFP_TYPE=" + precision,
                "-ffunction-sections", "-fdata-sections", "-I" + str(upstream),
                str(tests / "reference/kalman_oracle.c"),
                str(upstream / "ciglet.c"), "-Wl,--gc-sections", "-lm", "-o", str(executable),
            ], check=True)
            subprocess.run([str(executable), str(output)], check=True)
            target = tests / "fixtures" / ("kalman-" + precision + ".bin")
            content = output.read_bytes()
            if args.check:
                if target.read_bytes() != content:
                    raise ValueError(f"original C reference changed: {precision}")
            else:
                target.write_bytes(content)
            print(f"{precision}: original C Kalman reference verified")


if __name__ == "__main__":
    main()
