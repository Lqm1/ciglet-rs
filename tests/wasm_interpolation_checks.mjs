export async function verifyInterpolationBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected interpolation error'); }
    let comparisons = 0, linearCases = 0, extendedCases = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label} bits at ${index}: ${actual[index]} vs ${expected[index]}`);
            ++comparisons;
        }
    }
    assert(api.BoundaryPolicy.IncludeFirst === 0 && api.BoundaryPolicy.LegacySkipFirst === 1, 'complete boundary enum');
    assert(api.KernelPolicy.Stable === 0 && api.KernelPolicy.Legacy === 1, 'complete kernel enum');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const linear = api[`interpolate_linear_${suffix}`], uniform = api[`interpolate_uniform_${suffix}`];
        const sinc = api[`interpolate_sinc_${suffix}`], withBoundary = api[`interpolate_sinc_with_${suffix}`];
        const withKernel = api[`interpolate_sinc_with_kernel_${suffix}`], fill = api[`interpolate_fill_missing_${suffix}`];
        function reader(bytes, magic, cases) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
            let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            const plane = count => ArrayType.from({ length: count }, scalar);
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === cases, 'interpolation corpus header');
            return { integer, scalar, plane, done: () => assert(offset === bytes.byteLength, 'interpolation corpus trailing bytes') };
        }
        const precision = width === 4 ? 'float' : 'double';
        const r = reader(await loadFixture(`interpolation-linear-${precision}.bin`), 'INT1', 24);
        for (let record = 0; record < 24; ++record) {
            const kind = r.integer(), count = r.integer(), queries = r.integer(), start = r.scalar(), end = r.scalar();
            const coordinates = r.plane(count), values = r.plane(count), positions = r.plane(queries), expected = r.plane(queries);
            const saved = [coordinates.slice(), values.slice(), positions.slice()];
            const output = kind === 0 ? linear(coordinates, values, positions) : uniform(start, end, values, positions);
            assert(output instanceof ArrayType, 'linear typed array'); equal(output, expected, 'original C linear');
            [coordinates, values, positions].forEach((plane, index) => equal(plane, saved[index], 'linear input unchanged'));
            ++linearCases;
        }
        r.done();
        for (const [tail, boundary] of [['', api.BoundaryPolicy.LegacySkipFirst], ['-include', api.BoundaryPolicy.IncludeFirst]]) {
            const e = reader(await loadFixture(`interpolation-extended-${precision}${tail}.bin`), 'EXT1', 28);
            for (let record = 0; record < 28; ++record) {
                const kind = e.integer(), count = e.integer(), queries = e.integer(), start = e.scalar(), end = e.scalar();
                const values = e.plane(count), positions = kind === 0 ? e.plane(queries) : new ArrayType(0), expected = e.plane(queries);
                const saved = values.slice(), savedPositions = positions.slice();
                if (kind === 0) {
                    equal(withBoundary(start, end, values, positions, boundary), expected, 'disclosed-corrected C sinc convenience');
                    equal(withKernel(start, end, values, positions, boundary, api.KernelPolicy.Legacy), expected, 'disclosed-corrected C sinc policies');
                    if (boundary === api.BoundaryPolicy.IncludeFirst) {
                        equal(sinc(start, end, values, positions), expected, 'corrected default C sinc');
                        equal(withKernel(start, end, values, positions, boundary, api.KernelPolicy.Stable), expected, 'explicit stable C sinc');
                    }
                } else {
                    equal(fill(values, start), expected, 'disclosed-corrected C missing gaps');
                }
                equal(values, saved, 'extended source unchanged'); equal(positions, savedPositions, 'extended queries unchanged');
                ++extendedCases;
            }
            e.done();
        }
        const values = new ArrayType([7, 0]), positions = new ArrayType([0, -100, 100]);
        for (const boundary of [api.BoundaryPolicy.IncludeFirst, api.BoundaryPolicy.LegacySkipFirst]) {
            for (const kernel of [api.KernelPolicy.Stable, api.KernelPolicy.Legacy]) {
                equal(withKernel(0, 2, values, positions, boundary, kernel), new ArrayType([boundary === 0 ? 7 : 0, 0, 0]), 'all policy boundary combinations');
            }
        }
        equal(sinc(0, 2, values, positions), withKernel(0, 2, values, positions, 0, 0), 'default corrected policies');
        const tiny = width === 4 ? Math.fround(1e-8) : 1e-8;
        const expected = new ArrayType([7 * Math.sin(Math.PI * tiny) / (Math.PI * tiny) * Math.sin(Math.PI * tiny / 4) / (Math.PI * tiny / 4)]);
        const stable = sinc(0, 2, values, new ArrayType([tiny]));
        assert(Math.abs(stable[0] - expected[0]) <= (width === 4 ? 1e-6 : 2e-15), 'independent analytic near-zero kernel');
        equal(stable, withKernel(0, 2, values, new ArrayType([tiny]), 0, 0), 'near-zero default delegation');
        equal(linear(new ArrayType([0, 2, 5]), new ArrayType([1, 5, -1]), new ArrayType([4, 1, -9, 10])), new ArrayType([1, 3, 1, -1]), 'unordered clamped queries');
        equal(uniform(0, 3, new ArrayType([1, 3, 5]), new ArrayType([1.5, -1, 8, 0])), new ArrayType([4, 1, 5, 1]), 'uniform source convention');
        equal(uniform(0, 2, new ArrayType([4]), new ArrayType([-2, 0, 1, 9])), new ArrayType([4, 4, 4, 4]), 'singleton source');
        for (const blank of [-99, NaN, Infinity]) {
            equal(fill(new ArrayType([blank, 2, blank, 6, blank]), blank), new ArrayType([2, 2, 4, 6, 6]), 'sentinel interior and exterior gaps');
            equal(fill(new ArrayType([blank, blank]), blank), new ArrayType([0, 0]), 'all missing fallback');
        }
        equal(fill(new ArrayType([-0]), NaN), new ArrayType([-0]), 'known signed zero');
        assert(fill(new ArrayType(0), NaN).length === 0, 'empty gaps');
        assert(linear(new ArrayType([0]), new ArrayType([1]), new ArrayType(0)).length === 0, 'empty linear queries');
        for (const operation of [uniform, sinc]) {
            assert(operation(0, 2, values, new ArrayType(0)).length === 0, 'empty uniform queries');
            throws(() => operation(0, 2, new ArrayType(0), positions));
            for (const [start, end] of [[0, 0], [2, 1], [NaN, 2], [0, Infinity]]) throws(() => operation(start, end, values, positions));
            throws(() => operation(0, 2, values, new ArrayType([NaN])));
            throws(() => operation(0, 2, new ArrayType([Infinity]), positions));
        }
        for (const coordinates of [[], [0], [0, 0], [1, 0], [0, NaN]]) throws(() => linear(new ArrayType(coordinates), values, positions));
        throws(() => fill(new ArrayType([NaN]), 0)); throws(() => fill(new ArrayType([Infinity]), NaN));
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        throws(() => linear(new ArrayType([0, 1]), new ArrayType([-largest, largest]), new ArrayType([0.5])));
        throws(() => uniform(-largest, largest, values, positions));
        throws(() => withBoundary(0, 2, values, positions, 9));
        throws(() => withKernel(0, 2, values, positions, 0, 9));
        const first = fill(new ArrayType([1, -99, 3]), -99), second = fill(new ArrayType([1, -99, 3]), -99);
        first[0] = 73; equal(second, new ArrayType([1, 2, 3]), 'independent arrays and instance reuse');
    }
    return { linearCases, extendedCases, comparisons, functions: 12, enums: 2 };
}
