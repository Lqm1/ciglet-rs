"""Reproduce original linear and explicitly corrected sinc/missing C references."""
import argparse
import difflib
from pathlib import Path
import subprocess
import tempfile


def run(arguments):
    result = subprocess.run([str(arg) for arg in arguments], capture_output=True)
    if result.returncode:
        message = result.stderr.decode(errors="replace")
        raise RuntimeError(f"{arguments[0]} exited {result.returncode}: {message}")


def retain(path, content, check):
    if check:
        if path.read_bytes() != content:
            raise ValueError(f"reference changed: {path.name}")
    else:
        path.write_bytes(content)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--cc", default="gcc")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    upstream = args.upstream.resolve()
    tests = Path(__file__).resolve().parent
    reference = tests / "reference"
    original_c = (upstream / "ciglet.c").read_text()
    original_h = (upstream / "ciglet.h").read_text()
    head, body = original_c.split("FP_TYPE* cig_sincinterpu", 1)
    allocation = "FP_TYPE* y = malloc(nx * sizeof(FP_TYPE));"
    if allocation not in body:
        raise ValueError("upstream sinc allocation changed")
    source = head + "FP_TYPE* cig_sincinterpu" + body.replace(
        allocation, "FP_TYPE* y = calloc(nx, sizeof(FP_TYPE));", 1)
    condition = "x[i] != blank || (isnan(blank) && (! isnan(x[i])))"
    if original_h.count(condition) != 1:
        raise ValueError("upstream missing-value condition changed")
    header = original_h.replace(condition,
        "((! isnan(blank)) && x[i] != blank) || (isnan(blank) && (! isnan(x[i])))")
    count = "interp1(i_sample, x_sample, n + 2, yi, nx)"
    if header.count(count) != 1:
        raise ValueError("upstream missing-value count changed")
    header = header.replace(count, "interp1(i_sample, x_sample, n + 1, yi, nx)")
    patch = "".join(difflib.unified_diff(original_c.splitlines(True), source.splitlines(True),
                                       fromfile="a/ciglet.c", tofile="b/ciglet.c"))
    patch += "".join(difflib.unified_diff(original_h.splitlines(True), header.splitlines(True),
                                        fromfile="a/ciglet.h", tofile="b/ciglet.h"))
    retain(reference / "interpolation-corrections.patch", patch.encode(), args.check)
    head, body = source.split("FP_TYPE* cig_sincinterpu", 1)
    if "ix_base + j > 0" not in body:
        raise ValueError("upstream sinc boundary changed")
    inclusive = head + "FP_TYPE* cig_sincinterpu" + body.replace("ix_base + j > 0", "ix_base + j >= 0", 1)
    patch = "".join(difflib.unified_diff(source.splitlines(True), inclusive.splitlines(True),
                                       fromfile="a/ciglet.c", tofile="b/ciglet.c"))
    retain(reference / "interpolation-include-first.patch", patch.encode(), args.check)
    with tempfile.TemporaryDirectory(prefix="interpolation-reference-") as temporary:
        root = Path(temporary)
        (root / "ciglet.c").write_text(source)
        (root / "ciglet-including.c").write_text(inclusive)
        (root / "ciglet.h").write_text(header)
        for precision in ["float", "double"]:
            builds = [
                ("linear", upstream / "ciglet.c", reference / "interpolation-linear_oracle.c", [upstream]),
                ("extended", root / "ciglet.c", reference / "interpolation-extended_oracle.c", [root, upstream]),
                ("extended-include", root / "ciglet-including.c", reference / "interpolation-extended_oracle.c", [root, upstream]),
            ]
            for kind, implementation, driver, includes in builds:
                executable = root / (kind + "-" + precision)
                output = root / (kind + "-" + precision + ".bin")
                run([args.cc, "-O2", "-DFP_TYPE=" + precision, "-ffunction-sections", "-fdata-sections",
                     *["-I" + str(path) for path in includes], driver, implementation,
                     "-Wl,--gc-sections", "-lm", "-o", executable])
                run([executable, output])
                suffix = "-include" if kind == "extended-include" else ""
                family = "extended" if suffix else kind
                target = tests / "fixtures" / f"interpolation-{family}-{precision}{suffix}.bin"
                retain(target, output.read_bytes(), args.check)
                print(f"{precision} {kind}: C reference verified", flush=True)


if __name__ == "__main__":
    main()
