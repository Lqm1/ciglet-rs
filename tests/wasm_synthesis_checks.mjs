export async function verifySynthesisBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected synthesis error'); };
    let cases = 0, comparisons = 0;
    const maximum = [0, 0];
    const exact = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, Owner = api[`SinusoidF${width * 8}`], single = api[`sinusoid_f${width * 8}`], multiple = api[`sinusoids_f${width * 8}`], round = width === 4 ? Math.fround : value => value;
        const bytes = await loadFixture(`c-synthesis-f${width * 8}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'SYN1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 48, 'complete synthesis corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < 48; ++record) {
            const isSingle = integer(), length = integer(), rate = integer(), count = integer(), frequencies = new ArrayType(count), amplitudes = new ArrayType(count), phases = new ArrayType(count);
            for (let index = 0; index < count; ++index) { frequencies[index] = scalar(); amplitudes[index] = scalar(); phases[index] = scalar(); }
            const saved = [frequencies.slice(), amplitudes.slice(), phases.slice()];
            let actual;
            if (isSingle === 1) { assert(count === 1, 'single corpus shape'); const tone = new Owner(frequencies[0], amplitudes[0], phases[0]); actual = single(tone, rate, length); tone.free(); }
            else actual = multiple(frequencies, amplitudes, phases, rate, length);
            const scale = Math.max(1, [...amplitudes].reduce((sum, value) => sum + Math.abs(value), 0));
            assert(actual.length === length, 'complete output samples');
            actual.forEach(value => {
                const expected = scalar(), error = Math.abs(value - expected) / scale;
                maximum[width === 4 ? 0 : 1] = Math.max(maximum[width === 4 ? 0 : 1], error);
                assert(Number.isFinite(value) && error <= (width === 4 ? 2e-7 : 2e-14), `unchanged C synthesis bound: ${error}`); ++comparisons;
            });
            [frequencies, amplitudes, phases].forEach((array, field) => array.forEach((value, index) => exact(value, saved[field][index], 'source unchanged'))); ++cases;
        }
        assert(offset === bytes.byteLength, 'full synthesis corpus consumed');
        for (const length of [0, 1, 2, 5, 16, 127]) for (const amplitude of [0, 0.25, -0.5, 2]) {
            const tone = new Owner(375, amplitude, 0.25), samples = single(tone, 16000, length);
            samples.forEach((value, index) => {
                // Long binary32 recurrence is checked against the C corpus above;
                // direct analytic equality applies to the two initial samples.
                if (width === 4 && index >= 2) return;
                const step = round(2 * Math.PI / 16000 * 375);
                const angle = width === 4 ? round(round(step * round(index - Math.floor(length / 2))) + round(0.25)) : step * (index - Math.floor(length / 2)) + 0.25;
                const expected = round(amplitude * Math.cos(angle));
                assert(Math.abs(value - expected) < (width === 4 ? 2e-7 : 1e-12), 'independent centered amplitude formula'); ++comparisons;
            }); tone.free();
        }
        const first = new Owner(220, 0.5, 0), second = new Owner(-440, -0.25, 0.125), a = single(first, 16000, 31), b = single(second, 16000, 31);
        const combined = multiple(new ArrayType([220, -440]), new ArrayType([0.5, -0.25]), new ArrayType([0, 0.125]), 16000, 31);
        combined.forEach((value, index) => exact(value, round(a[index] + b[index]), 'input order addition')); second.free();
        for (const [rate, length] of [[0, 3], [0xffffffff, 3], [16000, 0xffffffff]]) { throws(() => single(first, rate, length)); exact(first.amplitude, 0.5, 'owner borrow released on errors'); }
        const empty = new ArrayType(); multiple(empty, empty, empty, 16000, 5).forEach(value => exact(value, 0, 'empty tone silence'));
        for (const arrays of [[empty, new ArrayType([1]), empty], [new ArrayType([1]), empty, empty], [empty, empty, new ArrayType([1])]]) throws(() => multiple(...arrays, 16000, 3));
        for (const invalid of [NaN, Infinity, -Infinity]) for (let field = 0; field < 3; ++field) {
            const values = new ArrayType([220, 0.5, 0]); values[field] = invalid; first.set_values(values);
            throws(() => single(first, 16000, 3)); throws(() => single(first, 16000, 0));
            const inputs = [new ArrayType([220, 220]), new ArrayType([0.5, 0.5]), new ArrayType([0, 0])]; inputs[field][1] = invalid;
            throws(() => multiple(...inputs, 16000, 3));
        }
        first.set_values(new ArrayType([375, 0.25, 0.125])); const recovered = single(first, 16000, 1); exact(recovered[0], round(Math.cos(0.125) * 0.25), 'short amplitude correction');
        first.frequency_hz = 0; first.amplitude = -2; first.phase_radians = 0;
        single(first, 16000, 5).forEach(value => exact(value, -2, 'all field setters'));
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE; first.frequency_hz = largest; throws(() => single(first, 1, 3));
        assert(single(first, 1, 0).length === 0, 'empty output skips step evaluation after finite parameter validation');
        const overflowSum = multiple(new ArrayType([0, 0]), new ArrayType([largest, largest]), new ArrayType([0, 0]), 16000, 1);
        exact(overflowSum[0], Infinity, 'native IEEE sum overflow retained');
        const payload = new ArrayType(3), data = new DataView(payload.buffer), patterns = width === 4 ? [0x7f812345n, 0x80000000n, 1n] : [0x7ff0123456789abcn, 0x8000000000000000n, 1n];
        patterns.forEach((bits, index) => width === 4 ? data.setUint32(index * width, Number(bits), true) : data.setBigUint64(index * width, bits, true));
        const sameBits = values => assert(new Uint8Array(values.buffer, values.byteOffset, values.byteLength).every((byte, index) => byte === new Uint8Array(payload.buffer)[index]), 'full tone field bits');
        const arbitrary = Owner.from_values(payload); sameBits(arbitrary.values()); first.set_values(payload); sameBits(first.values());
        for (const length of [0, 1, 2, 4]) { throws(() => Owner.from_values(new ArrayType(length))); throws(() => first.set_values(new ArrayType(length))); sameBits(first.values()); }
        const cloned = first.cloned(), retained = first.values(); first.free(); arbitrary.free(); sameBits(cloned.values()); sameBits(retained); cloned.free();
    }
    return {cases, comparisons, maximumNormalizedCError:maximum, functions:4, owners:2};
}
