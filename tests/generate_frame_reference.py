"""Generate centered-frame fixtures using the unchanged upstream C header."""
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
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="frame-reference-") as temporary:
        root = Path(temporary)
        for precision in ["float", "double"]:
            executable = root / ("oracle-" + precision)
            output = root / (precision + ".bin")
            subprocess.run([
                args.cc, "-O2", "-DFP_TYPE=" + precision,
                "-I" + str(args.upstream.resolve()),
                str(tests / "reference/frame_oracle.c"), "-o", str(executable),
            ], check=True)
            subprocess.run([str(executable), str(output)], check=True)
            content = output.read_bytes()
            target = tests / "fixtures" / ("frame-" + precision + ".bin")
            if args.check:
                if target.read_bytes() != content:
                    raise ValueError("original C frame fixture differs: " + precision)
            else:
                target.write_bytes(content)
            print(precision + ": original C frame fixture verified")


if __name__ == "__main__":
    main()
