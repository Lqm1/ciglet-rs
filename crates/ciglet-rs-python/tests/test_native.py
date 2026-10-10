"""Exercise the installed extension against unchanged original C FFT fixtures."""
from pathlib import Path
import struct

import pytest
import ciglet_rs as api

FIXTURES = Path(__file__).resolve().parents[3] / "tests" / "fixtures"


@pytest.mark.parametrize("width", [4, 8])
@pytest.mark.parametrize("inverse", [False, True])
def test_original_c_fourier_records(width, inverse):
    suffix = f"f{width * 8}"
    data = (FIXTURES / f"c-{'idft' if inverse else 'fft'}-{suffix}.bin").read_bytes()
    assert data[:4] == (b"IDF1" if inverse else b"FFT1")
    offset = 4

    def integer():
        nonlocal offset
        value = struct.unpack_from("<I", data, offset)[0]
        offset += 4
        return value

    def number():
        nonlocal offset
        value = struct.unpack_from("<f" if width == 4 else "<d", data, offset)[0]
        offset += width
        return value

    assert integer() == width
    count = integer()
    assert count == (120 if inverse else 240)
    plans = {}
    for _ in range(count):
        length, scenario, mask, direction = (integer() for _ in range(4))
        if scenario == 0:
            real = [0.75 if index == length // 3 else 0 for index in range(length)]
            imaginary = [-0.25 if index == length // 4 else 0 for index in range(length)]
        elif scenario == 1:
            real = [(index % 17 - 8) * 0.125 for index in range(length)]
            imaginary = [(index % 11 - 5) * 0.0625 for index in range(length)]
        else:
            assert scenario == 2
            real = [2048 + (index % 7) * 0.125 for index in range(length)]
            imaginary = [-1024 + (index % 5) * 0.0625 for index in range(length)]
        expected = [number() for _ in range(length * 2)]
        real_input = real if mask & 1 else None
        imaginary_input = imaginary if mask & 2 else None
        if inverse:
            results = [getattr(api, f"inverse_dft_{suffix}")(length, real_input, imaginary_input)]
        else:
            plan = plans.setdefault(length, getattr(api, f"FftPlanF{width * 8}")(length))
            results = [
                getattr(api, f"fft_{suffix}")(length, real_input, imaginary_input, direction),
                plan.transform(real_input, imaginary_input, direction),
            ]
        tolerance = 2e-7 if width == 4 else 1e-15
        input_scale = max([1] + [abs(value) for plane in (real_input, imaginary_input) if plane for value in plane])
        scale = input_scale * (length if direction == 0 else 1)
        for result in results:
            for actual, reference in zip(result.interleaved, expected, strict=True):
                assert abs(actual - reference) / scale <= tolerance
            copied = result.real
            copied[0] = 12345
            assert result.real[0] != 12345
    assert offset == len(data)


def test_callbacks_keep_context_and_exception_identity():
    context = object()
    def fold(accumulator, value, actual_context):
        assert actual_context is context
        return accumulator + value
    assert api.scalar_fold_f64([1, 2, 3], 4, fold, context) == 10
    error = RuntimeError("callback failure")
    def fail(*_):
        raise error
    with pytest.raises(RuntimeError) as caught:
        api.scalar_fold_f64([1], 0, fail, context)
    assert caught.value is error


def test_mutating_algorithms_return_updated_owned_arrays():
    source = [3.0, 1.0, 2.0]
    result, updated = api.select_nth_f64(source, 1)
    assert result == 2.0
    assert updated[1] == 2.0
    assert source == [3.0, 1.0, 2.0]


def test_invalid_inputs_raise_checked_exceptions():
    with pytest.raises(ValueError):
        api.FftPlanF32(3)
    with pytest.raises(ValueError):
        api.FftPlanF32(4).transform([1], None, 0)



def test_installed_type_stub_matches_public_runtime_api():
    import ast
    stub = Path(api.__file__).with_name("__init__.pyi")
    assert stub.is_file()
    assert stub.with_name("py.typed").is_file()
    for definition in ast.parse(stub.read_text()).body:
        if isinstance(definition, (ast.ClassDef, ast.FunctionDef)):
            exported = getattr(api, definition.name)
            if isinstance(definition, ast.ClassDef):
                for member in definition.body:
                    if isinstance(member, ast.FunctionDef):
                        assert hasattr(exported, member.name), f"{definition.name}.{member.name}"



def test_native_process_output_and_thread_safe_snapshot():
    import sys
    from concurrent.futures import ThreadPoolExecutor
    child = api.Figure.spawn(sys.executable, ["-c", "import sys; sys.stdout.buffer.write(sys.stdin.buffer.read()); sys.stderr.write('captured')"])
    child.append(b"native process input")
    child.flush()
    output = child.close_with_output()
    assert child.is_closed
    assert output.success and output.code == 0
    assert output.stdout() == b"native process input"
    assert output.stderr() == b"captured"
    with ThreadPoolExecutor(max_workers=1) as executor:
        assert executor.submit(output.stdout).result() == b"native process input"
    with pytest.raises(RuntimeError):
        child.flush()
