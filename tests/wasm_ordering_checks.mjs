export async function verifyOrderingBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected ordering error'); }
    let records = 0, selections = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label}/${index}`); ++comparisons;
        }
    }
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const BitArray = width === 4 ? Uint32Array : BigUint64Array;
        const minimum = api[`minimum_${suffix}`], maximum = api[`maximum_${suffix}`], median = api[`median_${suffix}`];
        const select = api[`select_nth_${suffix}`], sort = api[`sort_${suffix}`], Owner = api[`SortedValuesF${width * 8}`];
        const bytes = await loadFixture(`statistics-${width === 4 ? 'float' : 'double'}.bin`);
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const plane = count => ArrayType.from({ length: count }, scalar);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'STA1' && integer() === width && integer() === 32, 'ordering corpus header');
        for (let record = 0; record < 32; ++record) {
            const count = integer(), lags = integer(), input = plane(count); plane(count);
            const expected = plane(9);
            equal([minimum(input), maximum(input), median(input)], [expected[4], expected[5], expected[6]], 'original C order statistics');
            integer(); scalar();
            const ranked = plane(count), sorted = plane(count), indices = Uint32Array.from({ length: count }, integer);
            plane(lags);
            const saved = input.slice();
            for (let index = 0; index < count; ++index) {
                const copy = input.slice(); equal([select(copy, index), copy[index]], [ranked[index], ranked[index]], 'original C selected value/copyback');
                assert(copy.slice(0, index).every(value => value <= copy[index]) && copy.slice(index + 1).every(value => value >= copy[index]), 'partition invariant');
                equal(ArrayType.from(copy).sort(), sorted, 'partition retains every value'); ++selections;
            }
            const result = sort(input); assert(result instanceof Owner && result.length === count, 'sort result class/length');
            const values = result.values, originalIndices = result.indices;
            assert(values instanceof ArrayType && originalIndices instanceof Uint32Array, 'sort typed fields');
            equal(values, sorted, 'original C sorted values'); equal(originalIndices, indices, 'original C stable original indices');
            result.free(); equal(values, sorted, 'values outlive owner'); equal(originalIndices, indices, 'indices outlive owner');
            equal(input, saved, 'shared inputs unchanged'); ++records;
        }
        assert(offset === bytes.byteLength, 'ordering corpus trailing bytes');
        assert(minimum(new ArrayType(0)) === undefined && maximum(new ArrayType(0)) === undefined, 'empty optional scalar absence');
        assert(Number.isNaN(minimum(new ArrayType([NaN]))) && Number.isNaN(maximum(new ArrayType([NaN]))), 'present NaN differs from absence');
        assert(minimum(new ArrayType([1, NaN, 2])) === 2 && maximum(new ArrayType([1, NaN, 2])) === 2, 'native NaN comparison fold');
        equal([minimum(new ArrayType([0, -0])), maximum(new ArrayType([0, -0]))], [-0, -0], 'second-operand zero ties');
        const max = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        equal([median(new ArrayType([max, max])), median(new ArrayType([-max, max]))], [max, 0], 'finite median overflow fallback');
        throws(() => median(new ArrayType(0)));
        const rejected = new ArrayType([3, 1, 2]); throws(() => select(rejected, 3)); equal(rejected, new ArrayType([3, 1, 2]), 'failed selection no mutation');
        throws(() => select(new ArrayType(0), 0));
        const patterns = width === 4
            ? [0x7fc12345, 0x80000000, 0, 0xffc12345, 0x7f812345, 0xff812345, 0x7f800000, 0xff800000, 1, 0x80000001, 0x7fc12345]
            : [0x7ff8123456789abcn, 0x8000000000000000n, 0n, 0xfff8123456789abcn, 0x7ff0123456789abcn, 0xfff0123456789abcn, 0x7ff0000000000000n, 0xfff0000000000000n, 1n, 0x8000000000000001n, 0x7ff8123456789abcn];
        const sign = width === 4 ? 0x80000000n : 0x8000000000000000n, mask = sign * 2n - 1n;
        const key = value => { const bits = BigInt(value); return bits & sign ? mask ^ bits : bits ^ sign; };
        const expectedIndices = patterns.map((_, index) => index).sort((left, right) => key(patterns[left]) < key(patterns[right]) ? -1 : key(patterns[left]) > key(patterns[right]) ? 1 : left - right);
        const sourceBits = new BitArray(patterns), input = new ArrayType(sourceBits.buffer), result = sort(input);
        const output = result.values, outputBits = new BitArray(output.buffer), outputIndices = result.indices;
        equal(Array.from(outputBits), expectedIndices.map(index => patterns[index]), 'complete IEEE total-order payloads');
        equal(outputIndices, expectedIndices, 'stable equal-payload indices');
        const changed = result.values; changed[0] = 73; equal(Array.from(new BitArray(result.values.buffer)), Array.from(outputBits), 'getters independent');
        result.free(); equal(Array.from(outputBits), expectedIndices.map(index => patterns[index]), 'full payloads outlive owner');
        for (let index = 0; index < patterns.length; ++index) {
            const copyBits = new BitArray(patterns), copy = new ArrayType(copyBits.buffer); select(copy, index);
            assert(copyBits[index] === patterns[expectedIndices[index]], 'mutable selection payload copyback');
            assert(Array.from(copyBits.slice(0, index)).every(bits => key(bits) <= key(copyBits[index])) && Array.from(copyBits.slice(index + 1)).every(bits => key(bits) >= key(copyBits[index])), 'IEEE partition invariant');
            equal(Array.from(copyBits).sort((a, b) => key(a) < key(b) ? -1 : key(a) > key(b) ? 1 : 0), expectedIndices.map(index => patterns[index]), 'partition payload multiset');
        }
        equal(Array.from(sourceBits), patterns, 'sort input payloads unchanged');
        const constructedValues = new ArrayType(new BitArray(patterns).buffer), constructedIndices = new Uint32Array([0xffffffff, 0, 17]);
        const constructed = new Owner(constructedValues, constructedIndices);
        constructedValues[0] = 73; constructedIndices[0] = 73;
        const snapshots = [constructed.values, constructed.indices];
        assert(constructed.length === patterns.length, 'constructor full value length');
        constructed.free(); equal(Array.from(new BitArray(snapshots[0].buffer)), patterns, 'arbitrary constructor field bits');
        equal(snapshots[1], [0xffffffff, 0, 17], 'complete independent index field');
        const empty = sort(new ArrayType(0)); assert(empty.length === 0 && empty.values.length === 0 && empty.indices.length === 0, 'empty result fields'); empty.free();
    }
    return { records, selections, comparisons, functions: 10, classes: 2 };
}
