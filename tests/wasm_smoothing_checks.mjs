export async function verifySmoothingBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected smoothing error'); }
    let medianCases = 0, legacyCases = 0, stableCases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label}/${index}`); ++comparisons;
        }
    }
    assert(api.SmoothingPolicy.Stable === 0 && api.SmoothingPolicy.Legacy === 1, 'complete smoothing policy');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double';
        const median = api[`median_filter_${suffix}`], average = api[`moving_average_${suffix}`], averageWith = api[`moving_average_with_${suffix}`];
        const rms = api[`moving_rms_${suffix}`], rmsWith = api[`moving_rms_with_${suffix}`];
        function reader(bytes, magic, cases) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === cases, 'smoothing corpus header');
            return { integer, scalar, plane: count => ArrayType.from({ length: count }, scalar), done: () => assert(offset === bytes.byteLength, 'smoothing corpus trailing bytes') };
        }
        const m = reader(await loadFixture(`median-filter-${precision}.bin`), 'MED1', 96);
        for (let record = 0; record < 96; ++record) {
            const count = m.integer(), radius = m.integer(), signal = m.plane(count), expected = m.plane(count), saved = signal.slice();
            equal(median(signal, radius), expected, 'original C median'); equal(signal, saved, 'median input unchanged'); ++medianCases;
        }
        m.done();
        const l = reader(await loadFixture(`smoothing-${precision}.bin`), 'SMO1', 60);
        for (let record = 0; record < 60; ++record) {
            const count = l.integer(), radius = l.scalar(), signal = l.plane(count), mean = l.plane(count), root = l.plane(count), saved = signal.slice();
            equal(averageWith(signal, radius, 1), mean, 'original C moving average');
            equal(rmsWith(signal, Math.trunc(radius) + 1, 1), root, 'original C RMS');
            equal(signal, saved, 'Legacy source unchanged'); ++legacyCases;
        }
        l.done();
        const s = reader(await loadFixture(`stable_smoothing-${precision}.bin`), 'SSW1', 330);
        for (let record = 0; record < 330; ++record) {
            const count = s.integer(), rootWidth = s.integer(), radius = s.scalar(), signal = s.plane(count), mean = s.plane(count), root = s.plane(count), saved = signal.slice();
            const actual = [average(signal, radius), rms(signal, rootWidth)], expected = [mean, root];
            equal(actual[0], averageWith(signal, radius, 0), 'Stable average default');
            equal(actual[1], rmsWith(signal, rootWidth, 0), 'Stable RMS default');
            for (let field = 0; field < 2; ++field) {
                assert(actual[field] instanceof ArrayType && actual[field].length === count, 'Stable typed output');
                for (let index = 0; index < count; ++index) {
                    const a = actual[field][index], e = expected[field][index];
                    const error = e === 0 ? Math.abs(a) : Math.abs(a / e - 1);
                    const quantum = width === 4 ? 2 ** -149 : Number.MIN_VALUE, minimumNormal = width === 4 ? 2 ** -126 : 2 ** -1022;
                    const subnormal = field === 1 && Math.abs(e) < minimumNormal && Math.abs(a - e) <= quantum;
                    assert((error <= (width === 4 ? 2e-7 : 4e-15) && (e !== 0 || a === 0)) || subnormal, `independent integral moment ${suffix}/${record}/${field}/${index}: ${error}`); ++comparisons;
                }
            }
            equal(signal, saved, 'Stable source unchanged'); ++stableCases;
        }
        s.done();
        equal(median(new ArrayType([4, 1, 9, 2]), 2), new ArrayType([4, 2.5, 5, 2]), 'even progressive endpoints');
        equal(median(new ArrayType([4, 1, 9, 2, 7]), 3), new ArrayType([4, 4, 2, 7, 7]), 'odd progressive endpoints');
        equal(average(new ArrayType([1, 2, 8]), 1), new ArrayType([1.5, 5, 8]), 'asymmetric sample-cell rule');
        equal(average(new ArrayType([1, 1, 1]), 1.5), new ArrayType([1, 1, 1]), 'fractional DC preservation');
        equal(average(new ArrayType([1, 2, 3]), width === 4 ? 2 ** -149 : 1e-300), new ArrayType([1.5, 2.5, 3]), 'positive tiny-width limit');
        equal(average(new ArrayType([-0, 4]), 0), new ArrayType([-0, 4]), 'zero average width preserves sign');
        equal(rms(new ArrayType([-3, 4]), 0), new ArrayType([3, 4]), 'zero RMS width absolute values');
        const max = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        equal(median(new ArrayType([max, max, max, max]), 2), new ArrayType([max, max, max, max]), 'finite median extreme fallback');
        equal(rms(new ArrayType([max]), 1), new ArrayType([max]), 'scaled RMS extreme');
        throws(() => rmsWith(new ArrayType([max]), 1, 1));
        assert(median(new ArrayType(0), 3).length === 0 && average(new ArrayType(0), 1).length === 0 && rms(new ArrayType(0), 1).length === 0, 'empty series');
        for (const invalid of [0, 2, 0xffffffff]) throws(() => median(new ArrayType([1]), invalid));
        for (const invalid of [NaN, Infinity, -Infinity]) {
            throws(() => median(new ArrayType([invalid]), 1)); throws(() => average(new ArrayType([invalid]), 1)); throws(() => rms(new ArrayType([invalid]), 1));
            throws(() => average(new ArrayType([1]), invalid));
        }
        throws(() => average(new ArrayType([1]), -1)); throws(() => rms(new ArrayType([1]), 0xffffffff));
        throws(() => averageWith(new ArrayType([1]), 1, 9)); throws(() => rmsWith(new ArrayType([1]), 1, 9));
        const first = median(new ArrayType([1, 2, 3]), 1), second = median(new ArrayType([1, 2, 3]), 1);
        first[0] = 73; equal(second, new ArrayType([1, 2, 3]), 'independent output and error recovery');
    }
    return { medianCases, legacyCases, stableCases, comparisons, functions: 10, enums: 1 };
}
