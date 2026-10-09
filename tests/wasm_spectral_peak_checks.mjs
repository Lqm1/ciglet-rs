export async function verifySpectralPeakBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected peak error'); };
    let cases = 0, comparisons = 0;
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, Owner = api[`SpectralPeakF${width * 8}`], interpolate = api[`spectral_peak_interpolate_f${width * 8}`];
        const bytes = await loadFixture(`spectral-peak-${width === 4 ? 'float' : 'double'}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'QIF1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 64, 'full peak corpus header');
        let offset = 12;
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < 64; ++record) {
            const source = new ArrayType([scalar(), scalar(), scalar()]), saved = source.slice();
            const expected = [scalar(), scalar()], actual = interpolate(source, 1);
            equal(actual.bin, expected[0], 'C peak bin'); equal(actual.magnitude, expected[1], 'C peak magnitude');
            source.forEach((value, index) => equal(value, saved[index], 'source unchanged'));
            const copied = actual.cloned(), retained = actual.values(); actual.free();
            equal(copied.bin, expected[0], 'clone lifetime'); equal(copied.magnitude, expected[1], 'clone magnitude'); copied.free();
            retained.forEach((value, index) => equal(value, expected[index], 'snapshot lifetime')); ++cases;
        }
        assert(offset === bytes.byteLength, 'full peak corpus consumed');
        const analytic = interpolate(new ArrayType([NaN, 0, 1, 0.5, Infinity]), 2);
        assert(Math.abs(analytic.bin - (1 + 7 / 6)) < (width === 4 ? 2e-7 : 1e-15), 'independent parabola bin');
        assert(Math.abs(analytic.magnitude - 49 / 48) < (width === 4 ? 2e-7 : 1e-15), 'independent parabola magnitude'); analytic.free();
        for (const [source, expected] of [[[0, 10, 10], [1.5, width === 4 ? Math.fround(10.2) : 10.2]], [[1, 1, 1], [1, 1]], [[0, 1, 2], [1, 1]], [[0, 1, 3], [1, 1]]]) {
            const result = interpolate(new ArrayType(source), 1); equal(result.bin, expected[0], 'cap/flat/displaced bin'); equal(result.magnitude, expected[1], 'cap/flat/displaced magnitude'); result.free();
        }
        for (const [source, bin] of [[[], 0], [[1], 0], [[1, 1], 1], [[1, 1, 1], 0], [[1, 1, 1], 2], [[1, 1, 1], 0xffffffff]]) throws(() => interpolate(new ArrayType(source), bin));
        for (const invalid of [NaN, Infinity, -Infinity]) for (let field = 0; field < 3; ++field) {
            const source = new ArrayType([0, 1, 0]); source[field] = invalid; const before = new Uint8Array(source.buffer).slice();
            throws(() => interpolate(source, 1)); assert(new Uint8Array(source.buffer).every((byte, index) => byte === before[index]), 'failed input unchanged');
        }
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        throws(() => interpolate(new ArrayType([largest, largest, largest]), 1));
        const source = new ArrayType(2), data = new DataView(source.buffer);
        const patterns = width === 4 ? [0x7f812345n, 0x80000000n] : [0x7ff0123456789abcn, 0x8000000000000000n];
        patterns.forEach((bits, index) => width === 4 ? data.setUint32(index * width, Number(bits), true) : data.setBigUint64(index * width, bits, true));
        const sameBits = values => assert(new Uint8Array(values.buffer, values.byteOffset, values.byteLength).every((byte, index) => byte === new Uint8Array(source.buffer)[index]), 'full native peak field bits');
        const arbitrary = Owner.from_values(source); sameBits(arbitrary.values());
        for (const length of [0, 1, 3]) { throws(() => Owner.from_values(new ArrayType(length))); throws(() => arbitrary.set_values(new ArrayType(length))); sameBits(arbitrary.values()); }
        arbitrary.bin = -Infinity; arbitrary.magnitude = Infinity; equal(arbitrary.bin, -Infinity, 'bin setter'); equal(arbitrary.magnitude, Infinity, 'magnitude setter');
        arbitrary.set_values(source); const cloned = arbitrary.cloned(), snapshot = arbitrary.values(); arbitrary.free(); sameBits(cloned.values()); sameBits(snapshot); cloned.free();
        const recovered = interpolate(new ArrayType([-2, -1, -2]), 1); equal(recovered.bin, 1, 'signed log magnitude'); equal(recovered.magnitude, -1, 'signed magnitude'); recovered.free();
    }
    return {cases, comparisons, functions:2, owners:2};
}
