export async function verifyCorrelogramBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected correlogram error'); };
    let cases = 0, comparisons = 0; const maxima = [0, 0];
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    const arrays = (actual, expected, label) => { assert(actual.length === expected.length, label + ' shape'); actual.forEach((value, index) => equal(value, expected[index], label)); };
    assert(api.CorrelogramMethod.Autocorrelation === 0 && api.CorrelogramMethod.AbsoluteDifference === 1 && api.CorrelogramMethod.SquaredDifference === 2 && api.CorrelogramMethod.Yin === 3, 'all method constants');
    assert(api.CorrelogramPolicy.Stable === 0 && api.CorrelogramPolicy.Legacy === 1, 'all policies');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`, Owner = api[`Array2F${width * 8}`];
        const compute = api[`correlogram_compute_${suffix}`], withPolicy = api[`correlogram_compute_with_${suffix}`], map = api[`correlogram_to_frequency_${suffix}`];
        const bytes = await loadFixture(`correlogram-${width === 4 ? 'float' : 'double'}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'CRG1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 32, 'complete correlogram corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const values = length => new ArrayType(Array.from({length}, scalar));
        const compareC = (actual, expected) => {
            assert(actual.length === expected.length, 'C shape');
            actual.forEach((value, index) => {
                const error = Math.abs(value - expected[index]) / (1 + Math.abs(expected[index])); maxima[width === 4 ? 0 : 1] = Math.max(maxima[width === 4 ? 0 : 1], error);
                if (width === 4) equal(value, expected[index], 'C binary32 exact');
                else { assert(Number.isFinite(value) && error <= 3e-15, 'unchanged binary64 C bound'); ++comparisons; }
            });
        };
        for (let record = 0; record < 32; ++record) {
            const sampleCount = integer(), frameCount = integer(), periodCount = integer(), method = integer(), samples = values(sampleCount);
            const centers = new Int32Array(Array.from({length:frameCount}, integer)), lengths = new Uint32Array(Array.from({length:frameCount}, integer)), expected = values(frameCount * periodCount);
            const saved = [samples.slice(), centers.slice(), lengths.slice()];
            const actual = withPolicy(samples, centers, lengths, periodCount, method, 1);
            equal(actual.rows, frameCount, 'result rows'); equal(actual.columns, periodCount, 'result columns'); compareC(actual.values(), expected);
            const frequencyCount = integer(), frequencies = values(frequencyCount), mappedExpected = values(frameCount * frequencyCount), mapped = map(actual, 16000, frequencies);
            equal(mapped.rows, frameCount, 'mapped rows'); equal(mapped.columns, frequencyCount, 'mapped columns'); compareC(mapped.values(), mappedExpected);
            actual.free(); mapped.free(); [samples, centers, lengths].forEach((array, index) => arrays(array, saved[index], 'source unchanged')); ++cases;
        }
        assert(offset === bytes.byteLength, 'full CRG1 corpus consumed');
        const ramp = new ArrayType([0, 1, 2, 3, 4, 5, 6, 7]), centers = new Int32Array([2]), lengths = new Uint32Array([4]);
        for (const policy of [0, 1]) for (const [method, expected] of [[1, [0, 4, 8, 12, 0]], [2, [0, 4, 16, 36, 0]], [3, [1, 1, 1.6, 108 / 56, 1]]]) {
            const result = withPolicy(ramp, centers, lengths, 5, method, policy);
            result.values().forEach((value, index) => { assert(Math.abs(value - expected[index]) < (width === 4 ? 2e-7 : 3e-16), 'independent difference/YIN'); ++comparisons; }); result.free();
        }
        for (const method of [0, 1, 2, 3]) {
            const automatic = compute(ramp, centers, lengths, 5, method), explicit = withPolicy(ramp, centers, lengths, 5, method, 0); arrays(automatic.values(), explicit.values(), 'default Stable'); automatic.free(); explicit.free();
        }
        const stable = compute(ramp, centers, lengths, 5, 0); stable.values().subarray(0, 4).forEach(value => { assert(Math.abs(value - 1) < (width === 4 ? 1e-7 : 3e-16), 'independent stable ramp ACF'); }); equal(stable.get(0, 4), 0, 'strict trailing boundary'); stable.free();
        const negative = compute(ramp, new Int32Array([-0x80000000]), lengths, 3, 1), clamped = compute(ramp, new Int32Array([0]), lengths, 3, 1); arrays(negative.values(), clamped.values(), 'negative center clamp'); negative.free(); clamped.free();
        const silence = compute(new ArrayType(8), centers, lengths, 3, 3); arrays(silence.values(), [1, 1, 1], 'YIN silence floor'); silence.free();
        const noFrames = compute(new ArrayType(), new Int32Array(), new Uint32Array(), 3, 3); equal(noFrames.rows, 0, 'empty frame rows'); equal(noFrames.columns, 3, 'empty frame periods'); noFrames.free();
        for (const [samples, c, l, periods] of [[ramp, centers, lengths, 0], [ramp, centers, lengths, 0x80000000], [ramp, centers, new Uint32Array(), 3], [ramp, centers, new Uint32Array([0]), 3], [ramp, centers, new Uint32Array([0x80000000]), 3], [new ArrayType(), centers, lengths, 3], [new ArrayType([NaN]), new Int32Array(), new Uint32Array(), 3]]) throws(() => compute(samples, c, l, periods, 3));
        for (const method of [4, 9, 0xffffffff]) throws(() => compute(ramp, centers, lengths, 3, method));
        throws(() => withPolicy(ramp, centers, lengths, 3, 0, 9));
        const frame = new api.CorrelogramFrame(-0x80000000, 4), cloned = frame.cloned(); frame.center = 2; frame.length = 0;
        throws(() => frame[`compute_${suffix}`](ramp, 3, 0, 0)); equal(frame.length, 0, 'frame error recovery'); frame.length = 4;
        for (const [method, policy] of [[9, 0], [0, 9], [0xffffffff, 0]]) { throws(() => frame[`compute_${suffix}`](ramp, 3, method, policy)); equal(frame.center, 2, 'invalid code releases frame borrow'); }
        const frameResult = frame[`compute_${suffix}`](ramp, 3, 1, 0); arrays(frameResult.values(), [0, 4, 8], 'single frame owner'); frameResult.free(); equal(cloned.center, -0x80000000, 'frame clone independent'); cloned.free();
        frame.center = 0x7fffffff; frame.length = 0x7fffffff;
        const wideFrame = frame[`compute_${suffix}`](ramp, 3, 3, 0); arrays(wideFrame.values(), [1, 1, 1], 'maximal native frame unavailable lags'); wideFrame.free();
        frame.length = 0xffffffff; throws(() => frame[`compute_${suffix}`](ramp, 3, 3, 0)); equal(frame.length, 0xffffffff, 'complete unsigned frame storage'); frame.free();
        const source = new Owner(2, 4, new ArrayType([0, 10, 20, 30, 100, 110, 120, 130])), frequencies = new ArrayType([100, 50, 40, 0, -100, -0]);
        const mapped = map(source, 100, frequencies); arrays(mapped.values(), [10, 20, 25, 30, 0, 0, 110, 120, 125, 130, 100, 100], 'independent frequency mapping and signed zero');
        for (const rate of [0, -1]) { throws(() => map(source, rate, frequencies)); equal(source.rows, 2, 'mapping error releases array borrow'); }
        for (const invalid of [NaN, Infinity, -Infinity]) { throws(() => map(source, 100, new ArrayType([invalid]))); equal(source.columns, 4, 'frequency validation recovery'); }
        const emptyFrequencies = map(source, 100, new ArrayType()); equal(emptyFrequencies.rows, 2, 'empty frequency rows'); equal(emptyFrequencies.columns, 0, 'empty frequency columns'); emptyFrequencies.free();
        source.free(); arrays(mapped.values(), [10, 20, 25, 30, 0, 0, 110, 120, 125, 130, 100, 100], 'mapped owner independent'); mapped.free();
        const singleton = new Owner(1, 1, new ArrayType([3])), singleMapped = map(singleton, 100, new ArrayType([0, 1])); arrays(singleMapped.values(), [3, 3], 'singleton lag'); singleton.free(); singleMapped.free();
        const bad = new Owner(1, 1, new ArrayType([NaN])); throws(() => map(bad, 100, new ArrayType([1]))); bad.free();
        const zeroColumns = Owner.zeros(1, 0); throws(() => map(zeroColumns, 100, new ArrayType([1]))); zeroColumns.free();
    }
    const offset = new Float64Array([0, 1, 2, 3, 4, 5, 6, 7].map(value => value + 1e12)), centers = new Int32Array([2]), lengths = new Uint32Array([4]);
    const legacy = api.correlogram_compute_with_f64(offset, centers, lengths, 3, 0, 1), stable = api.correlogram_compute_f64(offset, centers, lengths, 3, 0);
    equal(legacy.get(0, 0), 0, 'Legacy cancellation retained'); stable.values().forEach(value => assert(Math.abs(value - 1) < 1e-7, 'Stable DC conditioning')); legacy.free(); stable.free();
    const huge = api.correlogram_compute_f64(new Float64Array([1, -1, 1, -1, 1, -1, 1, -1].map(value => value * Number.MAX_VALUE)), centers, lengths, 3, 0);
    equal(huge.get(0, 0), 1, 'Stable scaling avoids overflow'); equal(huge.get(0, 1), 0, 'negative correlation clipped'); huge.free();
    return {cases, comparisons, maximumNormalizedCError:maxima, functions:6, methods:4, policies:2, frameOwners:1};
}
