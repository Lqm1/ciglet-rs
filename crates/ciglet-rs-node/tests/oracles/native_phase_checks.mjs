export async function verifyPhaseBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected native input error'); }
    let comparisons = 0, dctCases = 0, phaseCases = 0, argumentF64Differences = 0, argumentF64MaxUlps = 0;
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array;
        const suffix = `f${width * 8}`;
        function reader(bytes, tag) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
            let offset = 4;
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === tag, 'fixture tag');
            return {
                integer() { const value = view.getUint32(offset, true); offset += 4; return value; },
                scalar(size = width) { const value = size === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += size; return value; },
                finished() { return offset === bytes.byteLength; },
            };
        }
        function equal(actual, expected, label) {
            assert(actual instanceof ArrayType && actual.length === expected.length, label + ' shape');
            for (let index = 0; index < actual.length; ++index) {
                if (width === 8 && label === 'argument' && !Object.is(actual[index], expected[index])) {
                    // The pinned WASM atan2 backend differs from native C by
                    // at most one ULP in the measured full original corpus.
                    // Keep signs, zeros, every other operation and f32 exact.
                    const view = new DataView(new ArrayBuffer(16));
                    view.setFloat64(0, actual[index]); view.setFloat64(8, expected[index]);
                    const a = view.getBigUint64(0), b = view.getBigUint64(8), distance = a > b ? a - b : b - a;
                    assert(Number.isFinite(actual[index]) && (a >> 63n) === (b >> 63n) && distance <= 1n, 'measured binary64 atan2 bound');
                    ++argumentF64Differences;
                    argumentF64MaxUlps = Math.max(argumentF64MaxUlps, Number(distance));
                } else {
                    assert(Object.is(actual[index], expected[index]), `${label} exact scalar mismatch at ${index}: ${actual[index]} versus ${expected[index]}`);
                }
                ++comparisons;
            }
        }
        const dct = api[`dct_${suffix}`];
        const dctReader = reader(await loadFixture(`c-dct-${suffix}.bin`), 'DCT1');
        assert(dctReader.integer() === width && dctReader.integer() === 32, 'DCT fixture header');
        for (let record = 0; record < 32; ++record) {
            const length = dctReader.integer(), scenario = dctReader.integer();
            const input = new ArrayType(length);
            for (let index = 0; index < length; ++index) {
                input[index] = scenario === 0 ? (index === Math.floor(length / 3) ? 0.75 : 0)
                    : scenario === 1 ? (index % 17 - 8) * 0.125
                    : scenario === 2 ? 2048 : index % 2 ? -0.5 : 0.5;
            }
            const saved = input.slice(), output = dct(input);
            assert(output instanceof ArrayType && output.length === length, 'DCT output shape');
            let scale = 1; for (const value of input) scale = Math.max(scale, Math.abs(value)); scale *= Math.sqrt(length);
            for (const actual of output) {
                const expected = dctReader.scalar();
                assert(Math.abs(actual - expected) / scale <= (width === 4 ? 2e-7 : 1e-15), 'unchanged normalized DCT bound');
                ++comparisons;
            }
            assert(input.every((value, index) => Object.is(value, saved[index])), 'DCT input retained');
            ++dctCases;
        }
        // This corpus also stores unrelated approximation records; consume the
        // exact format without claiming coverage of those separate functions.
        assert(dctReader.integer() === 12, 'unrelated approximation record count');
        for (let index = 0; index < 36; ++index) dctReader.scalar(4);
        assert(dctReader.finished(), 'DCT corpus trailing bytes');
        const unwrap = api[`phase_unwrap_${suffix}`], wrap = api[`phase_wrap_${suffix}`], difference = api[`phase_difference_${suffix}`];
        const magnitude = api[`magnitude_${suffix}`], argument = api[`argument_${suffix}`];
        const polarReal = api[`polar_real_${suffix}`], polarImaginary = api[`polar_imaginary_${suffix}`], complete = api[`complete_spectrum_${suffix}`];
        const phaseReader = reader(await loadFixture(`vector-phase-${width === 4 ? 'float' : 'double'}.bin`), 'VPH1');
        assert(phaseReader.integer() === width && phaseReader.integer() === 24, 'phase fixture header');
        for (let record = 0; record < 24; ++record) {
            const length = phaseReader.integer();
            const planes = Array.from({ length: 3 }, () => ArrayType.from({ length }, () => phaseReader.scalar()));
            const expected = Array.from({ length: 12 }, () => ArrayType.from({ length }, () => phaseReader.scalar()));
            const [real, imaginary, angles] = planes, saved = planes.map(plane => plane.slice());
            equal(unwrap(angles), expected[3], 'phase unwrap');
            equal(ArrayType.from(angles, angle => wrap(angle)), expected[4], 'phase wrap');
            equal(ArrayType.from(angles, (angle, index) => difference(angle, angles[(index + 3) % length])), expected[5], 'phase difference');
            equal(magnitude(real, imaginary), expected[6], 'magnitude');
            equal(argument(real, imaginary), expected[7], 'argument');
            equal(polarReal(expected[6], angles), expected[8], 'polar real');
            equal(polarImaginary(expected[6], angles), expected[9], 'polar imaginary');
            equal(complete(real, 0, 1), expected[10], 'legacy symmetric completion');
            equal(complete(real, 1, 1), expected[11], 'legacy antisymmetric completion');
            assert(planes.every((plane, index) => plane.every((value, slot) => Object.is(value, saved[index][slot]))), 'phase input planes retained');
            ++phaseCases;
        }
        // The remaining grids/interpolation/integer sequences belong to other
        // public families and are parsed without claiming their WASM coverage.
        assert(phaseReader.integer() === 18, 'unrelated grid count');
        for (let record = 0; record < 18; ++record) {
            const length = phaseReader.integer();
            for (let index = 0; index < length + 4; ++index) phaseReader.scalar();
        }
        assert(phaseReader.integer() === 9, 'unrelated integer-sequence count');
        for (let record = 0; record < 9; ++record) {
            const length = phaseReader.integer();
            for (let index = 0; index < length + 2; ++index) phaseReader.integer();
        }
        assert(phaseReader.finished(), 'phase corpus trailing bytes');
        throws(() => dct(new ArrayType(0)));
        for (const invalid of [NaN, Infinity, -Infinity]) {
            throws(() => wrap(invalid)); throws(() => difference(invalid, 0)); throws(() => difference(0, invalid));
            throws(() => unwrap(new ArrayType([invalid])));
            throws(() => magnitude(new ArrayType([invalid]), new ArrayType([0])));
            throws(() => argument(new ArrayType([0]), new ArrayType([invalid])));
            throws(() => polarReal(new ArrayType([1]), new ArrayType([invalid])));
            throws(() => polarImaginary(new ArrayType([invalid]), new ArrayType([0])));
        }
        for (const operation of [magnitude, argument, polarReal, polarImaginary]) {
            throws(() => operation(new ArrayType(1), new ArrayType(2)));
            assert(operation(new ArrayType(0), new ArrayType(0)).length === 0, 'empty paired operation');
        }
        assert(unwrap(new ArrayType(0)).length === 0, 'empty phase unwrap');
        throws(() => difference(0, 1e20));
        const input = new ArrayType([1, 2, 3, NaN]);
        equal(complete(input, 0), new ArrayType([1, 2, 3, 2]), 'default preserved symmetric Nyquist');
        equal(complete(input, 1, null), new ArrayType([1, 2, 3, -2]), 'default preserved antisymmetric Nyquist');
        equal(complete(input, 1, 1), new ArrayType([1, 2, 2, -2]), 'legacy positive antisymmetric Nyquist');
        throws(() => complete(new ArrayType([1, NaN, 3, 4]), 0, 0));
        equal(complete(new ArrayType([1, 2, NaN, 4]), 0, 1), new ArrayType([1, 2, 2, 2]), 'legacy ignored Nyquist is overwritten');
        for (const symmetry of [0, 1]) for (const policy of [0, 1]) {
            assert(complete(new ArrayType(0), symmetry, policy).length === 0, 'empty spectrum');
            equal(complete(new ArrayType([3]), symmetry, policy), new ArrayType([3]), 'singleton spectrum');
        }
        const first = magnitude(new ArrayType([3]), new ArrayType([4])); first[0] = 73;
        equal(magnitude(new ArrayType([3]), new ArrayType([4])), new ArrayType([5]), 'independent output/recovery');
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        equal(magnitude(new ArrayType([largest]), new ArrayType([0])), new ArrayType([largest]), 'native scaled magnitude fallback');
        throws(() => magnitude(new ArrayType([largest]), new ArrayType([largest])));
        const tiny = width === 4 ? Math.fround(1e-30) : 1e-200;
        equal(magnitude(new ArrayType([tiny]), new ArrayType([0])), new ArrayType([tiny]), 'native magnitude underflow fallback');
        const single = new ArrayType([-0]);
        equal(unwrap(single), single, 'singleton signed-zero phase retention');
        assert(Object.is(argument(new ArrayType([1]), new ArrayType([-0]))[0], -0), 'negative-zero complex argument');
        equal(complete(new ArrayType([1, 2, 3, NaN, NaN]), 1, 0), new ArrayType([1, 2, 3, -3, -2]), 'preserved odd spectrum completion');
    }
    assert(api.SpectrumSymmetry.Antisymmetric === 1 && api.SpectrumNyquist.Legacy === 1, 'spectrum enum codes');
    return { dctCases, phaseCases, comparisons, functions: 18, argumentF64Differences, argumentF64MaxUlps };
}
