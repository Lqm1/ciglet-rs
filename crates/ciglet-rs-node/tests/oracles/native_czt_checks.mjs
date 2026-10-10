export async function verifyCztBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected CZT error'); };
    let cases = 0, comparisons = 0; const maxima = [0, 0];
    const exact = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, Plan = api[`CztPlanF${width * 8}`], once = api[`czt_f${width * 8}`], bound = width === 4 ? 5e-7 : 2e-15;
        const bytes = await loadFixture(`c-czt-f${width * 8}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'CZT1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 1200, 'complete CZT corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < 1200; ++record) {
            const length = integer(), scenario = integer(), mask = integer(), direction = integer(), step = scalar(), real = new ArrayType(length), imaginary = new ArrayType(length);
            for (let index = 0; index < length; ++index) {
                if (scenario === 0) { real[index] = index === Math.floor(length / 3) ? 0.75 : 0; imaginary[index] = index === Math.floor(length / 4) ? -0.25 : 0; }
                else if (scenario === 1) { real[index] = (index % 17 - 8) * 0.125; imaginary[index] = (index % 11 - 5) * 0.0625; }
                else { assert(scenario === 2, 'scenario'); real[index] = 2048 + index % 7 * 0.125; imaginary[index] = -1024 + index % 5 * 0.0625; }
            }
            const inputReal = mask & 1 ? real : undefined, inputImaginary = mask & 2 ? imaginary : undefined;
            const savedReal = real.slice(), savedImaginary = imaginary.slice();
            const nativeScale = Math.max(1, ...inputReal ? [...inputReal].map(Math.abs) : [], ...inputImaginary ? [...inputImaginary].map(Math.abs) : []) * (direction === 0 ? length : 1);
            assert(Number.isFinite(nativeScale) && nativeScale >= 1, 'complete normalization scale');
            const expected = new ArrayType(Array.from({length:2 * length}, scalar)), plan = new Plan(length, step, direction), actual = plan.transform(inputReal, inputImaginary);
            exact(plan.length, length, 'plan length'); assert(!plan.is_empty(), 'plan not empty'); exact(actual.length, length, 'result length');
            actual.interleaved.forEach((value, index) => {
                const error = Math.abs(value - expected[index]) / nativeScale; maxima[width === 4 ? 0 : 1] = Math.max(maxima[width === 4 ? 0 : 1], error);
                assert(Number.isFinite(value) && error <= bound, `unchanged C CZT bound width${width} record${record} index${index}: ${error}`); ++comparisons;
            });
            const output = new ArrayType(2 * length).fill(17); const [status, updated] = plan.transform_into(inputReal, inputImaginary, output); assert(status === undefined, "native into status"); assert(output.every(value => value === 17), "native output input unchanged"); output.set(updated);
            const retained = actual.interleaved; output.forEach((value, index) => exact(value, retained[index], 'native into output'));
            const repeated = plan.transform(inputReal, inputImaginary); repeated.interleaved.forEach((value, index) => exact(value, retained[index], 'reused plan')); repeated.free();
            if (record % 20 === 0) { const temporary = once(length, step, direction, inputReal, inputImaginary); temporary.interleaved.forEach((value, index) => exact(value, retained[index], 'one-shot')); temporary.free(); }
            actual.free(); plan.free(); assert(retained.length === 2 * length, 'snapshot outlives owners'); ++cases;
            real.forEach((value, index) => exact(value, savedReal[index], 'real input unchanged'));
            imaginary.forEach((value, index) => exact(value, savedImaginary[index], 'imaginary input unchanged'));
        }
        assert(offset === bytes.byteLength, 'full CZT1 corpus consumed');
        for (const length of [0, 0xffffffff, 0x7fffffff]) throws(() => new Plan(length, 0.125, 0));
        for (const step of [NaN, Infinity, -Infinity, width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE]) throws(() => new Plan(5, step, 0));
        for (const direction of [2, 9, 0xffffffff]) { throws(() => new Plan(5, 0.125, direction)); throws(() => once(5, 0.125, direction, undefined, undefined)); }
        const real = new ArrayType([0.25, -0.5, 1, 0.125, -0.25]), imaginary = new ArrayType([-0.125, 0.25, 0.5, -0.25, 0.125]);
        for (const step of [0, 0.125, -0.25]) for (const direction of [0, 1]) {
            const plan = new Plan(5, step, direction), result = plan.transform(real, imaginary), actual = result.interleaved;
            for (let k = 0; k < 5; ++k) {
                let re = 0, im = 0;
                for (let i = 0; i < 5; ++i) { const angle = (direction === 0 ? -1 : 1) * step * i * k, cosine = Math.cos(angle), sine = Math.sin(angle); re += real[i] * cosine - imaginary[i] * sine; im += real[i] * sine + imaginary[i] * cosine; }
                if (direction) { re /= 5; im /= 5; }
                const tolerance = width === 4 ? bound * (direction ? 1 : 5) : 1e-13;
                assert(Math.hypot(actual[2 * k] - re, actual[2 * k + 1] - im) < tolerance, 'independent arbitrary grid sum'); ++comparisons;
            }
            const output = new ArrayType(10).fill(7), saved = new Uint8Array(output.buffer).slice();
            for (const [re, im] of [[real.subarray(0, 4), undefined], [undefined, imaginary.subarray(0, 4)], [new ArrayType(), undefined]]) {
                throws(() => plan.transform(re, im)); throws(() => plan.transform_into(re, im, output));
                assert(new Uint8Array(output.buffer).every((byte, index) => byte === saved[index]), 'atomic output shape error'); exact(plan.length, 5, 'error releases plan borrow');
            }
            for (const length of [0, 9, 11]) throws(() => plan.transform_into(real, imaginary, new ArrayType(length)));
            output.set(plan.transform_into(real, imaginary, output)[1]); output.forEach((value, index) => exact(value, actual[index], 'recovery'));
            const aliased = new ArrayType([...real, ...imaginary]); aliased.set(plan.transform_into(aliased.subarray(0, 5), aliased.subarray(5), aliased)[1]);
            aliased.forEach((value, index) => exact(value, actual[index], 'output aliases both input planes safely'));
            const nonfinite = plan.transform(new ArrayType([NaN, 0, 0, 0, 0]), undefined); assert([...nonfinite.interleaved].some(Number.isNaN), 'native IEEE nonfinite propagation'); nonfinite.free();
            const silence = plan.transform(undefined, undefined); silence.interleaved.forEach(value => assert(value === 0, 'absent planes zero')); silence.free(); result.free(); plan.free();
        }
        // The native exact-DFT-grid test is binary64. Binary32 chirp phase
        // rounding is covered by every original C case and the direct sums above.
        if (width !== 8) continue;
        for (const length of [1, 2, 8, 32]) {
            const real = new ArrayType(Array.from({length}, (_, i) => (i - 3) * 0.125)), step = 2 * Math.PI / length;
            const forward = new Plan(length, step, 0), transformed = forward.transform(real, undefined), inverse = new Plan(length, step, 1), recovered = inverse.transform(transformed.real, transformed.imaginary);
            const expected = api[`fft_f${width * 8}`](length, real, undefined, 0);
            const expectedValues = expected.interleaved;
            transformed.interleaved.forEach((value, index) => assert(Math.abs(value - expectedValues[index]) < 1e-11, 'native binary64 orthogonal FFT grid'));
            recovered.real.forEach((value, index) => assert(Math.abs(value - real[index]) < 1e-12, 'native binary64 orthogonal round trip'));
            recovered.imaginary.forEach(value => assert(Math.abs(value) < 1e-12, 'native binary64 round trip imaginary'));
            expected.free(); transformed.free(); recovered.free(); forward.free(); inverse.free();
        }
    }
    return {cases, comparisons, maximumNormalizedCError:maxima, functions:2, owners:2};
}
