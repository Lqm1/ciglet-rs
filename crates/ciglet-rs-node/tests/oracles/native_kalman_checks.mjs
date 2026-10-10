export async function verifyKalmanBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected Kalman error'); };
    let cases = 0, comparisons = 0;
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    const arrayEqual = (actual, expected, label) => { assert(actual.length === expected.length, label + ' shape'); actual.forEach((value, index) => equal(value, expected[index], label)); };
    for (const width of [4, 8]) {
        const suffix = `f${width * 8}`, ArrayType = width === 4 ? Float32Array : Float64Array, Owner = api[`KalmanEstimateF${width * 8}`];
        const filter = api[`kalman_filter_${suffix}`], initialFilter = api[`kalman_filter_with_initial_${suffix}`], smooth = api[`kalman_smooth_${suffix}`];
        const bytes = await loadFixture(`kalman-${width === 4 ? 'float' : 'double'}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'KLM1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 108, 'complete Kalman corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const values = count => new ArrayType(Array.from({length:count}, scalar));
        for (let record = 0; record < 108; ++record) {
            const count = integer(), initial = scalar(), measurements = values(count), process = values(count), noise = values(count);
            const means = values(count), variances = values(count), smoothed = values(count), likelihood = scalar();
            const saved = [measurements.slice(), process.slice(), noise.slice()];
            const actual = initialFilter(measurements, process, noise, initial);
            arrayEqual(actual.means, means, 'C means'); arrayEqual(actual.variances, variances, 'C variances'); equal(actual.log_likelihood, likelihood, 'C likelihood');
            arrayEqual(smooth(actual.means, actual.variances, process), smoothed, 'C smoothing');
            if (Object.is(initial, measurements[0])) {
                const automatic = filter(measurements, process, noise);
                arrayEqual(automatic.means, means, 'default initial means'); arrayEqual(automatic.variances, variances, 'default initial variances'); equal(automatic.log_likelihood, likelihood, 'default initial likelihood'); automatic.free();
            }
            [measurements, process, noise].forEach((array, index) => arrayEqual(array, saved[index], 'source unchanged'));
            const cloned = actual.cloned(), retained = actual.means; actual.free();
            arrayEqual(cloned.means, means, 'clone after source free'); arrayEqual(retained, means, 'snapshot after source free'); cloned.free(); ++cases;
        }
        assert(offset === bytes.byteLength, 'full Kalman corpus consumed');
        const empty = new ArrayType(), unit = new ArrayType([1]), zero = new ArrayType([0]);
        const emptyResult = filter(empty, empty, empty); equal(emptyResult.log_likelihood, 0, 'empty likelihood'); assert(emptyResult.means.length === 0 && emptyResult.variances.length === 0, 'empty estimate'); emptyResult.free(); assert(smooth(empty, empty, empty).length === 0, 'empty smoothing');
        for (const bad of [new ArrayType([-1]), new ArrayType([NaN]), new ArrayType([Infinity]), empty, new ArrayType([1, 1])]) {
            throws(() => filter(unit, bad, unit)); throws(() => filter(unit, unit, bad)); throws(() => smooth(unit, bad, unit)); throws(() => smooth(unit, unit, bad));
        }
        for (const invalid of [NaN, Infinity, -Infinity]) {
            throws(() => initialFilter(unit, unit, unit, invalid)); throws(() => filter(new ArrayType([invalid]), unit, unit)); throws(() => smooth(new ArrayType([invalid]), unit, unit));
        }
        throws(() => filter(unit, zero, zero));
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        throws(() => filter(unit, new ArrayType([largest]), new ArrayType([largest])));
        throws(() => smooth(new ArrayType([1, 2]), new ArrayType([largest, 1]), new ArrayType([1, largest])));
        arrayEqual(smooth(new ArrayType([1, 2]), new ArrayType([0, 0]), new ArrayType([0, 0])), [1, 2], 'deterministic smoothing');
        const fixed = initialFilter(new ArrayType([1, 2]), new ArrayType([0, 0]), new ArrayType([1, 1]), 0.25);
        arrayEqual(fixed.means, [0.25, 0.25], 'fixed initial'); arrayEqual(fixed.variances, [0, 0], 'zero process variance'); fixed.free();
        const independent = initialFilter(new ArrayType([1, 0]), new ArrayType([1, 1]), new ArrayType([1, 1]), 0);
        equal(independent.means[0], 0.5, 'independent posterior'); equal(independent.variances[0], 0.5, 'independent variance');
        assert(Math.abs(independent.means[1] - 0.2) < (width === 4 ? 1e-7 : 1e-15), 'independent second posterior');
        assert(Math.abs(smooth(independent.means, independent.variances, new ArrayType([1, 1]))[0] - 0.4) < (width === 4 ? 1e-7 : 1e-15), 'independent backward posterior'); independent.free();
        const payload = new ArrayType(3), data = new DataView(payload.buffer);
        const patterns = width === 4 ? [0x7f812345n, 0x80000000n, 1n] : [0x7ff0123456789abcn, 0x8000000000000000n, 1n];
        patterns.forEach((bits, index) => width === 4 ? data.setUint32(index * width, Number(bits), true) : data.setBigUint64(index * width, bits, true));
        const bitArray = values => [...new Uint8Array(values.buffer, values.byteOffset, values.byteLength)];
        const sameBits = (actual, expected) => assert(JSON.stringify(bitArray(actual)) === JSON.stringify(bitArray(expected)), 'full native field bits');
        const owned = new Owner(payload, empty, Infinity); sameBits(owned.means, payload); assert(owned.variances.length === 0, 'arbitrary mismatched fields permitted');
        owned.variances = payload; owned.means = empty; owned.log_likelihood = -Infinity; equal(owned.log_likelihood, -Infinity, 'likelihood setter');
        owned.set_likelihood_value(payload.subarray(0, 1)); sameBits(owned.likelihood_value(), payload.subarray(0, 1));
        for (const length of [0, 2, 3]) { throws(() => owned.set_likelihood_value(new ArrayType(length))); sameBits(owned.likelihood_value(), payload.subarray(0, 1)); }
        const cloned = owned.cloned(), retained = owned.variances; owned.variances = empty; owned.free();
        sameBits(cloned.variances, payload); sameBits(retained, payload); sameBits(cloned.likelihood_value(), payload.subarray(0, 1)); cloned.free();
    }
    for (const variance of [1e-100, 1e100]) {
        const result = api.kalman_filter_with_initial_f64(new Float64Array([0]), new Float64Array([variance]), new Float64Array([variance]), 0);
        assert(Math.abs(result.log_likelihood + 0.5 * Math.log(2 * Math.PI * 2 * variance)) < 1e-12, 'wide logarithm corrected native range'); result.free();
    }
    return {cases, comparisons, functions:6, owners:2};
}
