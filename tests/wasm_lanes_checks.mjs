export async function verifyLaneBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected lane error'); }
    let cases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) { assert(Object.is(actual[index], expected[index]), label + '/' + index); ++comparisons; }
    }
    const names = { vfastpow2: 'exponential2', vfasterpow2: 'exponential2_fast', vfastexp: 'exponential', vfasterexp: 'exponential_fast',
        vfastlog2: 'logarithm2', vfasterlog2: 'logarithm2_fast', vfastlog: 'logarithm', vfasterlog: 'logarithm_fast',
        vfastpow: 'power', vfasterpow: 'power_fast', to_integers: 'to_integers', from_integers: 'from_integers', absolute: 'absolute' };
    const lines = new TextDecoder().decode(await loadFixture('c-lanes.txt')).trim().split(/\r?\n/);
    assert(lines.length === 80, 'complete original SSE2 fixture'); const covered = new Set();
    for (const line of lines) {
        const fields = line.split(/\s+/), name = fields[0];
        if (!name.startsWith('v') && !['to_integers', 'from_integers', 'absolute'].includes(name)) continue;
        const operation = api[`lanes_${names[name]}`]; assert(operation, 'complete lane dispatch'); covered.add(name);
        const words = Uint32Array.from(fields.slice(1, 5), field => parseInt(field, 16)), expected = Uint32Array.from(fields.slice(-4), field => parseInt(field, 16));
        const input = name === 'from_integers' ? new Int32Array(words.buffer) : new Float32Array(words.buffer), saved = words.slice();
        const binary = fields.length === 13, exponentWords = binary ? Uint32Array.from(fields.slice(5, 9), field => parseInt(field, 16)) : null;
        const exponents = binary ? new Float32Array(exponentWords.buffer) : undefined;
        const savedExponents = exponentWords?.slice();
        if (name.startsWith('v') && new Float32Array(expected.buffer).some(value => !Number.isFinite(value))) throws(() => operation(input, exponents));
        else {
            const actual = operation(input, exponents);
            assert(actual instanceof (name === 'to_integers' ? Int32Array : Float32Array), 'result typed storage');
            equal(new Uint32Array(actual.buffer), expected, 'original SSE2 output bits');
        }
        equal(words, saved, 'input words unchanged');
        if (binary) equal(new Uint32Array(exponents.buffer), savedExponents, 'per-lane exponents retained');
        ++cases;
    }
    assert(covered.size === 13, 'all ten kernels and three original conversions');
    for (const name of Object.values(names).slice(0, 10)) {
        const operation = api[`lanes_${name}`], binary = name.startsWith('power');
        for (const length of [0, 1, 3, 5]) {
            throws(() => operation(new Float32Array(length), new Float32Array(4)));
            if (binary) throws(() => operation(new Float32Array(4), new Float32Array(length)));
        }
        for (let slot = 0; slot < 4; ++slot) {
            const invalids = [NaN, Infinity, -Infinity];
            if (name.startsWith('logarithm') || binary) invalids.push(0, -1);
            if (name.startsWith('exponential')) invalids.push(128);
            for (const invalid of invalids) {
                const input = new Float32Array([1, 2, 3, 4]); input[slot] = invalid; const saved = new Uint32Array(input.buffer).slice();
                throws(() => operation(input, new Float32Array([1, 2, 3, 4]))); equal(new Uint32Array(input.buffer), saved, 'late invalid lane immutable');
            }
            if (binary) {
                const exponents = new Float32Array([1, 2, 3, 4]); exponents[slot] = 256; throws(() => operation(new Float32Array([2, 2, 2, 2]), exponents));
            }
        }
        const input = new Float32Array([0.25, 0.5, 1, 2]), exponents = new Float32Array([1, 2, 3, 4]);
        const expected = Float32Array.from(input, (value, index) => api[`approximation_${name}`](value, exponents[index]));
        const first = operation(input, exponents), second = operation(input, exponents); equal(first, expected, 'root scalar agreement'); first[0] = 73;
        equal(second, expected, 'independent arrays/post-error reuse');
    }
    const patterns = new Uint32Array([0x80000000, 0x7fc12345, 0x7f812345, 0xff812345]);
    const arbitrary = api.lanes_from_bits(patterns); equal(api.lanes_to_bits(arbitrary), patterns, 'complete quiet/signaling NaN payload roundtrip');
    equal(new Uint32Array(api.lanes_absolute(arbitrary).buffer), new Uint32Array([0, 0x7fc12345, 0x7f812345, 0x7f812345]), 'absolute clears only sign bit');
    const raw = api.lanes_from_bits(patterns); patterns[0] = 73; assert(new Uint32Array(raw.buffer)[0] === 0x80000000, 'from bits independent storage');
    const signed = new Int32Array([-2147483648, 2147483647, 16777217, -16777217]);
    equal(api.lanes_from_integers(signed), Float32Array.from(signed), 'nearest-even integer rounding');
    equal(api.lanes_to_integers(new Float32Array([NaN, Infinity, -Infinity, 2147483648])), [-2147483648, -2147483648, -2147483648, -2147483648], 'SSE2 indefinite conversion');
    equal(api.lanes_to_integers(new Float32Array([-1.75, 1.75, -0, 2147483520])), [-1, 1, 0, 2147483520], 'truncation and boundary');
    for (const [first, second] of [[0x0123456789abcdefn, -0x0123456789abcdf0n], [-9223372036854775808n, 9223372036854775807n], [-1n, 0n]]) {
        const expected = [first, second].flatMap(value => [Number(BigInt.asIntN(32, value)), Number(BigInt.asIntN(32, value >> 32n))]);
        const actual = api.lanes_from_integer64(first, second); assert(actual instanceof Int32Array, 'BigInt construction returns integer lanes');
        equal(actual, expected, 'low/high word order without Number precision loss');
    }
    for (const [name, Type] of [['to_bits', Float32Array], ['from_bits', Uint32Array], ['to_integers', Float32Array], ['from_integers', Int32Array], ['absolute', Float32Array]]) {
        for (const length of [0, 1, 3, 5]) throws(() => api[`lanes_${name}`](new Type(length)));
    }
    return { cases, comparisons, functions: 16, kernels: 10, conversions: 6 };
}
