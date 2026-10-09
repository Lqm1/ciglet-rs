export async function verifyVectorBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected native vector error'); }
    let comparisons = 0, vectorCases = 0, gridCases = 0, integerCases = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label} bits at ${index}`);
            ++comparisons;
        }
    }
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const bytes = await loadFixture(`vector-phase-${width === 4 ? 'float' : 'double'}.bin`);
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        let offset = 4;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'VPH1' && integer() === width && integer() === 24, 'vector corpus header');
        const difference = api[`difference_${suffix}`], sum = api[`cumulative_sum_${suffix}`], reverse = api[`reverse_${suffix}`];
        const grid = api[`linear_grid_${suffix}`], between = api[`interpolate_between_${suffix}`];
        for (let record = 0; record < 24; ++record) {
            const length = integer();
            const planes = Array.from({ length: 3 }, () => ArrayType.from({ length }, scalar));
            const expected = Array.from({ length: 12 }, () => ArrayType.from({ length }, scalar));
            const input = planes[0], saved = input.slice();
            for (const [operation, field] of [[difference, 0], [sum, 1], [reverse, 2]]) {
                const output = operation(input);
                assert(output instanceof ArrayType, 'correct vector typed array');
                equal(output, expected[field], 'original C vector');
            }
            equal(input, saved, 'input unchanged');
            ++vectorCases;
        }
        assert(integer() === 18, 'grid record count');
        for (let record = 0; record < 18; ++record) {
            const length = integer(), start = scalar(), end = scalar();
            const expected = ArrayType.from({ length }, scalar), output = grid(start, end, length);
            assert(output instanceof ArrayType, 'grid typed array'); equal(output, expected, 'original C grid');
            const ratio = scalar(), interpolated = scalar();
            assert(Object.is(between(start, end, ratio), interpolated), 'original C scalar interpolation bits'); ++comparisons;
            ++gridCases;
        }
        assert(integer() === 9, 'integer sequence record count');
        for (let record = 0; record < 9; ++record) {
            const length = integer(), start = integer() | 0, step = integer() | 0;
            const expected = Int32Array.from({ length }, () => integer() | 0), output = api.integer_sequence(start, step, length);
            assert(output instanceof Int32Array, 'signed integer typed array'); equal(output, expected, 'original C integer sequence');
            ++integerCases;
        }
        assert(offset === bytes.byteLength, 'vector corpus trailing bytes');
        for (const operation of [difference, sum, reverse]) {
            assert(operation(new ArrayType(0)).length === 0, 'empty vector');
            equal(operation(new ArrayType([-0])), new ArrayType([-0]), 'singleton negative zero');
        }
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        for (const operation of [difference, sum]) {
            for (const invalid of [NaN, Infinity, -Infinity]) throws(() => operation(new ArrayType([1, invalid])));
        }
        throws(() => difference(new ArrayType([-largest, largest])));
        throws(() => sum(new ArrayType([largest, largest])));
        for (const invalid of [NaN, Infinity, -Infinity]) {
            throws(() => grid(invalid, 1, 0)); throws(() => grid(0, invalid, 1));
            throws(() => between(invalid, 1, 0.5)); throws(() => between(0, invalid, 0.5)); throws(() => between(0, 1, invalid));
        }
        assert(grid(1, 2, 0).length === 0, 'zero-point grid');
        equal(grid(-0, 2, 1), new ArrayType([-0]), 'singleton grid sign');
        equal(grid(-largest, largest, 3), new ArrayType([-largest, 0, largest]), 'native grid range fallback');
        assert(Object.is(between(-largest, largest, 0.5), 0), 'native scalar interpolation range fallback');
        assert(between(-largest, largest, 0) === -largest && between(-largest, largest, 1) === largest, 'fallback exact endpoints');
        throws(() => between(largest, -largest, 2)); throws(() => grid(0, 1, 0xffffffff));
        const first = difference(new ArrayType([1, 3])), second = difference(new ArrayType([1, 3]));
        first[0] = 73; equal(second, new ArrayType([1, 2]), 'independent vector outputs');
        const BitArray = width === 4 ? Uint32Array : BigUint64Array;
        const patterns = width === 4 ? [0x80000000, 0x7fc12345, 0x7f812345, 0x7f800000, 0xff800000, 1]
            : [0x8000000000000000n, 0x7ff8123456789abcn, 0x7ff0123456789abcn, 0x7ff0000000000000n, 0xfff0000000000000n, 1n];
        const inputBits = new BitArray(patterns), outputBits = new BitArray(reverse(new ArrayType(inputBits.buffer)).buffer);
        assert(outputBits.every((value, index) => value === patterns[patterns.length - 1 - index]), 'reverse retains arbitrary scalar bits');
        assert(inputBits.every((value, index) => value === patterns[index]), 'reverse retains original bit storage');
    }
    equal(api.integer_sequence(-2000000000, 2000000000, 3), new Int32Array([-2000000000, 0, 2000000000]), 'wide intermediate integer arithmetic');
    equal(api.integer_sequence(-2147483648, 0, 2), new Int32Array([-2147483648, -2147483648]), 'minimum signed integer');
    equal(api.integer_sequence(2147483647, 0, 2), new Int32Array([2147483647, 2147483647]), 'maximum signed integer');
    assert(api.integer_sequence(1, 2, 0).length === 0, 'empty signed sequence');
    throws(() => api.integer_sequence(2147483647, 1, 2)); throws(() => api.integer_sequence(-2147483648, -1, 2));
    throws(() => api.integer_sequence(0, 1, 0xffffffff));
    equal(api.integer_sequence(1, 2, 3), new Int32Array([1, 3, 5]), 'instance usable after integer errors');
    return { vectorCases, gridCases, integerCases, comparisons, functions: 11 };
}
