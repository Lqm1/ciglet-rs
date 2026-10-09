export async function verifyFrameExtremaBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected frame/extrema error'); }
    let frameCases = 0, extremaCases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label}/${index}`); ++comparisons;
        }
    }
    assert(api.ExtremumDirection.Maximum === 0 && api.ExtremumDirection.Minimum === 1, 'complete direction enum');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double';
        const frame = api[`fetch_frame_${suffix}`], extremum = api[`extremum_${suffix}`], peak = api[`peak_${suffix}`];
        function reader(bytes, magic, cases) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === cases, 'frame/extrema corpus header');
            return { integer, plane: count => ArrayType.from({ length: count }, scalar), done: () => assert(offset === bytes.byteLength, 'frame/extrema corpus trailing bytes') };
        }
        const f = reader(await loadFixture(`frame-${precision}.bin`), 'FRM1', 25);
        const input = new ArrayType([-2, 0.5, 4, 1.25, -0.75, 3]), saved = input.slice();
        for (let record = 0; record < 25; ++record) {
            const center = f.integer() | 0, length = f.integer(), expected = f.plane(length), actual = frame(input, center, length);
            assert(actual instanceof ArrayType, 'frame typed array'); equal(actual, expected, 'original C frame'); ++frameCases;
        }
        f.done(); equal(input, saved, 'frame source unchanged');
        const e = reader(await loadFixture(`extrema-${precision}.bin`), 'EXT2', 180);
        for (let record = 0; record < 180; ++record) {
            const count = e.integer(), start = e.integer(), end = e.integer(), direction = e.integer(), signal = e.plane(count);
            const expectedPeak = e.integer(), expectedExtremum = e.integer(), saved = signal.slice();
            equal([peak(signal, start, end, direction), extremum(signal, start, end, direction)], [expectedPeak, expectedExtremum], 'original C extrema');
            equal(signal, saved, 'search source unchanged'); ++extremaCases;
        }
        e.done();
        const signal = new ArrayType([8, 1, 5, 0, 5, 1, 9]);
        equal([peak(signal, 0, 6, 0), peak(signal, 1, 5, 0), extremum(signal, 0, 6, 0)], [0, 2, 6], 'native fallback/first tie/global distinction');
        equal([peak(new ArrayType([1]), 0, 0, 1), extremum(new ArrayType([1]), 0, 0, 1)], [0, 0], 'singleton searches');
        equal([extremum(new ArrayType([-0, 0]), 0, 1, 0), extremum(new ArrayType([-0, 0]), 0, 1, 1)], [0, 0], 'first signed-zero ties');
        const inspected = new ArrayType([NaN, 1, 3, 2, NaN]);
        assert(extremum(inspected, 1, 3, 0) === 2 && peak(inspected, 2, 2, 0) === 2, 'nonfinite outside inspected range');
        throws(() => peak(inspected, 1, 3, 0));
        for (const operation of [extremum, peak]) {
            throws(() => operation(new ArrayType(0), 0, 0, 0));
            throws(() => operation(signal, 5, 3, 0)); throws(() => operation(signal, 0, 7, 0));
            throws(() => operation(signal, 0, 6, 9));
        }
        equal(frame(new ArrayType(0), 0, 3), new ArrayType([0, 0, 0]), 'empty input zero padding');
        assert(frame(input, 0, 0).length === 0, 'empty frame');
        equal(frame(input, -100, 3), new ArrayType([0, 0, 0]), 'distant signed center');
        throws(() => frame(input, -2147483648, 4)); throws(() => frame(input, 2147483647, 4));
        throws(() => frame(input, 0, 0xffffffff));
        const BitArray = width === 4 ? Uint32Array : BigUint64Array;
        const patterns = width === 4 ? [0x80000000, 0x7fc12345, 0x7f812345, 0x7f800000, 0xff800000, 1]
            : [0x8000000000000000n, 0x7ff8123456789abcn, 0x7ff0123456789abcn, 0x7ff0000000000000n, 0xfff0000000000000n, 1n];
        const sourceBits = new BitArray(patterns), special = new ArrayType(sourceBits.buffer), actual = frame(special, 3, 8), outputBits = new BitArray(actual.buffer);
        equal(Array.from(outputBits), [width === 4 ? 0 : 0n, ...patterns, width === 4 ? 0 : 0n], 'complete frame payload bits and zero padding');
        equal(Array.from(sourceBits), patterns, 'source payload bits unchanged');
        const first = frame(input, 2, 3), second = frame(input, 2, 3); first[0] = 73;
        equal(second, new ArrayType([0.5, 4, 1.25]), 'independent frames and post-error reuse');
    }
    return { frameCases, extremaCases, comparisons, functions: 6, enums: 1 };
}
