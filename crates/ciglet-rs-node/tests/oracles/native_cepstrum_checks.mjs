export async function verifyBandCepstrumBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected band cepstrum error'); };
    let cases = 0, comparisons = 0;
    const maxima = [0, 0];
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    const Options = api.BandCepstralOptions;
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const selected = api[`band_to_cepstrum_${suffix}`], legacy = api[`band_to_cepstrum_legacy_${suffix}`], matrix = api[`band_spectrogram_to_cepstra_${suffix}`];
        const bytes = await loadFixture(`c-cepstrum-${suffix}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'CEP1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 48, 'complete band cepstrum corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < 48; ++record) {
            const bands = integer(), order = integer(), includeDc = integer() !== 0, options = new Options(order, includeDc), count = options.coefficient_count(bands);
            equal(count, Math.min(order, bands - 1) + Number(includeDc), 'coefficient count');
            const features = new ArrayType(Array.from({length:3 * bands}, scalar)), saved = features.slice(), transformed = matrix(features, bands, options), expectedMatrix = [];
            assert(transformed.length === 3 * count, 'matrix full output shape');
            for (let frame = 0; frame < 3; ++frame) {
                const input = features.subarray(frame * bands, (frame + 1) * bands), full = legacy(input, options), prefix = selected(input, options);
                assert(full.length === bands && prefix.length === count, 'full legacy buffer and prefix');
                for (let index = 0; index < bands; ++index) {
                    const expected = scalar(), error = Math.abs(full[index] - expected) / Math.max(1, Math.abs(expected)); expectedMatrix.push(expected);
                    maxima[width === 4 ? 0 : 1] = Math.max(maxima[width === 4 ? 0 : 1], error);
                    assert(Number.isFinite(full[index]) && error < (width === 4 ? 1e-7 : 1e-15), 'unchanged original C band cepstrum bound'); ++comparisons;
                    if (index < count) { equal(prefix[index], full[index], 'selected prefix'); equal(transformed[frame * count + index], prefix[index], 'frame-major output'); }
                }
            }
            expectedMatrix.forEach(expected => equal(scalar(), expected, 'original C full matrix suffix'));
            features.forEach((value, index) => equal(value, saved[index], 'source unchanged')); options.free(); ++cases;
        }
        assert(offset === bytes.byteLength, 'full CEP1 corpus consumed');
        const options = new Options(0xffffffff, true), cloned = options.cloned(); options.order = 0; options.include_dc = false;
        equal(cloned.order, 0xffffffff, 'full uint32 order clone'); assert(cloned.include_dc && !options.include_dc, 'independent boolean setter');
        equal(options.coefficient_count(20), 0, 'zero order no DC'); assert(selected(new ArrayType([1, -2, 3]), options).length === 0, 'empty selected coefficients');
        options.include_dc = true; equal(options.coefficient_count(20), 1, 'DC only');
        equal(cloned.coefficient_count(20), 20, 'maximum order clamp'); cloned.free();
        for (const bands of [0, 0x80000000, 0xffffffff]) { throws(() => options.coefficient_count(bands)); equal(options.order, 0, 'error releases options borrow'); }
        const empty = new ArrayType(); assert(matrix(empty, 2, options).length === 0, 'empty frames'); throws(() => matrix(empty, 0, options));
        for (const operation of [selected, legacy]) { throws(() => operation(empty, options)); throws(() => operation(new ArrayType([NaN]), options)); }
        throws(() => matrix(new ArrayType([1, 2, 3]), 2, options));
        for (const invalid of [NaN, Infinity, -Infinity]) {
            const source = new ArrayType([1, 2, 3, invalid]), before = new Uint8Array(source.buffer).slice();
            throws(() => matrix(source, 2, options)); assert(new Uint8Array(source.buffer).every((byte, index) => byte === before[index]), 'late frame error retains source');
        }
        options.order = 12; const singleton = selected(new ArrayType([2]), options);
        const cosineZero = api.approximation_cosine(0); equal(singleton[0], width === 4 ? Math.fround(2 * cosineZero) : 2 * cosineZero, 'native singleton DC');
        const bank = api[`FilterBankF${width * 8}`].mel_default(257, 8000, 20, 50, 8000);
        const bands = bank.apply_spectrogram(new ArrayType(3 * 257), 0); bank.free();
        const floored = new ArrayType([...bands].map(value => Math.max(value, -15))), cepstra = matrix(floored, 20, options);
        assert(cepstra.length === 39, 'filterbank to cepstrum pipeline complete');
        const expectedDc = -15 * Math.sqrt(20) * cosineZero;
        for (let frame = 0; frame < 3; ++frame) {
            const difference = Math.abs(cepstra[frame * 13 + 12] - expectedDc);
            assert(width === 4 ? difference / Math.max(1, Math.abs(expectedDc)) < 1e-7 : difference < 1e-12, 'pipeline DC');
        }
        options.free(); assert(cepstra.length === 39, 'independent output survives options free');
    }
    return {cases, comparisons, maximumNormalizedCError:maxima, functions:6, owners:1};
}
