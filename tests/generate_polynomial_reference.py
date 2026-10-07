"""Generate original-C polynomial evaluation and root references for both precisions."""
import argparse
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--cc", default="gcc")
    parser.add_argument("--zig", action="store_true", help="Use zig cc instead of a C compiler executable")
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--output-directory", type=Path, help="Write or check another reference directory")
    args = parser.parse_args()
    upstream = args.upstream.resolve()
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="polynomial-reference-") as temporary:
        root = Path(temporary)
        for precision in ["float", "double"]:
            executable = root / ("oracle-" + precision + (".exe" if os.name == "nt" else ""))
            output = root / (precision + ".bin")
            compiler = [args.cc, "cc", "-target", "x86_64-windows-gnu"] if args.zig else [args.cc]
            linker = [] if os.name == "nt" else ["-Wl,--gc-sections"]
            dependencies = [str(upstream / "external/fftsg_h.c"), str(upstream / "external/fast_median.c")] if os.name == "nt" else []
            subprocess.run(compiler + [
                "-O2", "-DFP_TYPE=" + precision,
                "-ffunction-sections", "-fdata-sections", "-I" + str(upstream),
                str(tests / "reference/polynomial_oracle.c"),
                str(upstream / "ciglet.c"), "-lm", "-o", str(executable),
            ] + linker + dependencies, check=True, timeout=120)
            subprocess.run([str(executable), str(output)], check=True, timeout=30)
            directory = args.output_directory if args.output_directory else tests / "fixtures"
            directory.mkdir(parents=True, exist_ok=True)
            target = directory / ("polynomial-" + precision + ".bin")
            content = output.read_bytes()
            if args.check:
                if target.read_bytes() != content:
                    raise ValueError(f"original C reference changed: {precision}")
            else:
                target.write_bytes(content)
            print(f"{precision}: original C polynomial reference verified")


if __name__ == "__main__":
    main()
