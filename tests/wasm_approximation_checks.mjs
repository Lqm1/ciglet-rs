export async function verifyApproximationBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected approximation error'); }
    let trigCases = 0, logExpCases = 0, frequencyTailCases = 0, bundledScalarCases = 0, comparisons = 0, hostDifferences = 0, maximumHostUlps = 0n;
    const differencesByOperation = {};
    const bitsBuffer = new ArrayBuffer(8), bitsView = new DataView(bitsBuffer);
    function bits(value) { bitsView.setFloat64(0, value, true); return bitsView.getBigUint64(0, true); }
    function equal(actual, expected, label, host = false) {
        if (host && actual !== 0 && expected !== 0) {
            const a = bits(actual), b = bits(expected), ulps = a > b ? a - b : b - a;
            const error = Math.abs(actual - expected) / (1 + Math.abs(expected));
            assert(Number.isFinite(actual) && error <= Number.EPSILON && ulps <= 1n, `${label}: ${actual} != ${expected}; ULP ${ulps}`);
            if (ulps) { ++hostDifferences; differencesByOperation[label] = (differencesByOperation[label] ?? 0) + 1; }
            if (ulps > maximumHostUlps) maximumHostUlps = ulps;
        } else assert(Object.is(actual, expected) || Number.isNaN(actual) && Number.isNaN(expected), `${label}: ${actual} != ${expected}`);
        ++comparisons;
    }
    function reader(bytes, width, magic, count) {
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const single = () => { const value = view.getFloat32(offset, true); offset += 4; return value; };
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === count, 'approximation corpus header');
        return { integer, scalar, single, skip: length => { offset += length; }, done: () => assert(offset === bytes.byteLength, 'complete approximation corpus consumption') };
    }
    assert(api.ApproximationPrecision.Fast === 0 && api.ApproximationPrecision.Approximate === 1 && api.ApproximationPrecision.Exact === 2, 'complete precision enum');
    equal(api.legacy_epsilon(), 1e-15, 'legacy epsilon'); equal(api.legacy_infinity_sentinel(), 1e15, 'finite legacy infinity');
    for (const width of [4, 8]) {
        const suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double', round = width === 4 ? Math.fround : value => value;
        const sine = api[`approximation_sine_${suffix}`], cosine = api[`approximation_cosine_${suffix}`];
        const atan = api[`approximation_atan2_${suffix}`], fastAtan = api[`approximation_atan2_fast_${suffix}`];
        const log = api[`approximation_logarithm_${suffix}`], exp = api[`approximation_exponential_${suffix}`];
        const minimum = api[`scalar_minimum_${suffix}`], maximum = api[`scalar_maximum_${suffix}`];
        const r = reader(await loadFixture(`scalar-math-${precision}.bin`), width, 'SCM1', 64);
        for (let record = 0; record < 64; ++record) {
            const angle = r.scalar(), y = r.scalar(), x = r.scalar();
            for (const mode of [0, 1, 2]) for (const [name, operation] of [['sine', sine], ['cosine', cosine]]) {
                const expected = r.scalar(); equal(operation(angle, mode), expected, name, mode === 2);
                if (width === 4 && mode < 2) equal(api[`approximation_${name}${mode === 0 ? '_fast' : ''}`](angle), expected, 'binary32 convenience');
            }
            for (const mode of [0, 1, 2]) {
                const expected = r.scalar(); equal(atan(y, x, mode), expected, 'atan2', mode !== 0);
                if (mode === 0) equal(fastAtan(y, x), expected, 'generic fast atan2');
            }
            ++trigCases;
        }
        assert(r.integer() === 16, 'comparison tail count');
        for (let record = 0; record < 16; ++record) {
            const a = r.scalar(), b = r.scalar();
            equal(minimum(a, b), r.scalar(), 'original scalar minimum'); equal(maximum(a, b), r.scalar(), 'original scalar maximum');
        }
        r.done();
        const c = reader(await loadFixture(`complex-${precision}.bin`), width, 'CPX1', 64);
        c.skip(64 * 22 * width); assert(c.integer() === 15, 'scalar log/exp tail');
        for (let record = 0; record < 15; ++record) {
            const value = c.scalar(), exponent = c.scalar();
            for (const mode of [0, 1, 2]) {
                const expected = c.scalar(); equal(log(value, mode), expected, 'original logarithm', mode === 2 && width === 8);
                if (width === 4 && mode < 2) equal(api[`approximation_logarithm${mode === 0 ? '_fast' : ''}`](value), expected, 'log convenience');
            }
            for (const mode of [0, 1, 2]) {
                const expected = c.scalar(); equal(exp(exponent, mode), expected, 'original exponential', mode === 2 && width === 8);
                if (width === 4 && mode < 2) equal(api[`approximation_exponential${mode === 0 ? '_fast' : ''}`](exponent), expected, 'exp convenience');
            }
            equal(api.approximation_logarithm2_fast(value), c.scalar(), 'fast base2 logarithm');
            equal(api.approximation_exponential2_fast(exponent), c.scalar(), 'fast base2 exponential'); ++logExpCases;
        }
        c.done();
        const f = reader(await loadFixture(`c-frequency-${suffix}.bin`), width, 'FRQ1', 12);
        f.skip(12 * 6 * width); assert(f.integer() === 5, 'frequency grids');
        for (let grid = 0; grid < 5; ++grid) f.skip((f.integer() + 1) * width);
        assert(f.integer() === 12, 'approximate log tail');
        for (let record = 0; record < 12; ++record) {
            const value = f.single(); equal(api.approximation_logarithm2(value), f.single(), 'approximate base2 log'); equal(api.approximation_logarithm(value), f.single(), 'approximate natural log'); ++frequencyTailCases;
        }
        assert(f.integer() === 12, 'approximate exponential tail');
        for (let record = 0; record < 12; ++record) {
            const value = f.single(); equal(api.approximation_exponential2(value), f.single(), 'approximate base2 exp'); equal(api.approximation_exponential(value), f.single(), 'approximate natural exp');
            equal(api.approximation_power(10, Math.fround(value / 40)), f.single(), 'original approximate power'); ++frequencyTailCases;
        }
        f.done();
        equal(sine(-0, 2), -0, 'exact signed-zero sine'); equal(cosine(0, 2), 1, 'exact cosine identity'); equal(log(1, 2), 0, 'log identity'); equal(exp(0, 2), 1, 'exp identity');
        equal(atan(-0, -1, 2), round(-Math.PI), 'negative signed branch'); equal(fastAtan(-0, -1), round(Math.PI), 'legacy fast branch');
        equal(atan(1, -1, 1), atan(1, -1, 2), 'shared accurate atan modes');
        for (const op of [minimum, maximum]) {
            equal(op(-0, 0), 0, 'second-operand zero tie'); equal(op(0, -0), -0, 'reversed zero tie');
            equal(op(NaN, 3), 3, 'unordered first operand'); equal(op(3, NaN), NaN, 'unordered second operand');
        }
        const largest = width === 4 ? Math.fround(3.4028234663852886e38) : Number.MAX_VALUE;
        for (const op of [sine, cosine]) {
            assert(Number.isFinite(op(largest, 2)), 'exact full range');
            for (const mode of [0, 1]) throws(() => op(largest, mode));
        }
        for (const mode of [0, 1]) equal(exp(-200, mode), exp(-1000, mode), 'source underflow clamp');
        for (const op of [sine, cosine, log, exp]) {
            throws(() => op(1, 9)); for (const mode of [0, 1, 2]) for (const value of [NaN, Infinity, -Infinity]) throws(() => op(value, mode));
        }
        for (const mode of [0, 1, 2]) {
            throws(() => log(0, mode)); throws(() => log(-1, mode)); throws(() => atan(NaN, 1, mode)); throws(() => atan(1, Infinity, mode));
        }
        throws(() => atan(1, 1, 9)); equal(sine(0, 2), 0, 'post-error reuse');
    }
    for (const operation of [api.approximation_logarithm2, api.approximation_logarithm2_fast]) for (const value of [0, -1, NaN, Infinity]) throws(() => operation(value));
    for (const operation of [api.approximation_exponential2, api.approximation_exponential2_fast]) {
        for (const value of [128, NaN, Infinity]) throws(() => operation(value));
        equal(operation(-126), operation(-1000), 'base2 clamp');
    }
    for (const operation of [api.approximation_power, api.approximation_power_fast]) { throws(() => operation(-1, 2)); throws(() => operation(1, NaN)); }
    equal(api.approximation_power_fast(2, 3), api.approximation_exponential2_fast(Math.fround(3 * api.approximation_logarithm2_fast(2))), 'fast power complete native kernels');
    const fromHex = value => { bitsView.setUint32(0, parseInt(value, 16), true); return bitsView.getFloat32(0, true); };
    const kernels = {
        fastpow2: api.approximation_exponential2, fasterpow2: api.approximation_exponential2_fast,
        fastexp: api.approximation_exponential, fasterexp: api.approximation_exponential_fast,
        fastlog2: api.approximation_logarithm2, fasterlog2: api.approximation_logarithm2_fast,
        fastlog: api.approximation_logarithm, fasterlog: api.approximation_logarithm_fast,
        fastpow: api.approximation_power, fasterpow: api.approximation_power_fast,
        fastsinfull: api.approximation_sine, fastersinfull: api.approximation_sine_fast,
        fastcosfull: api.approximation_cosine, fastercosfull: api.approximation_cosine_fast,
    }, covered = new Set();
    for (const filename of ['c-lanes.txt', 'c-trigonometric.txt']) {
        const lines = new TextDecoder().decode(await loadFixture(filename)).trim().split(/\r?\n/);
        assert(lines.length === (filename === 'c-lanes.txt' ? 80 : 456), 'bundled original corpus line count');
        for (const line of lines) {
            const fields = line.trim().split(/\s+/), vector = fields[0].startsWith('v'), name = vector ? fields[0].slice(1) : fields[0];
            if (!(name in kernels)) continue;
            const operation = kernels[name]; covered.add(name);
            for (let lane = 0; lane < (vector ? 4 : 1); ++lane) {
                const value = fromHex(fields[1 + lane]);
                const exponent = vector && fields.length === 13 ? fromHex(fields[5 + lane]) : !vector && fields.length === 4 ? fromHex(fields[2]) : 0;
                const expected = fromHex(fields[fields.length - (vector ? 4 : 1) + lane]);
                if (Number.isFinite(expected)) equal(operation(value, exponent), expected, 'original bundled scalar ' + name);
                else throws(() => operation(value, exponent));
                ++bundledScalarCases;
            }
        }
    }
    assert(covered.size === 14, 'all fourteen bundled root scalar kernels covered by original C');
    return { trigCases, logExpCases, frequencyTailCases, bundledScalarCases, comparisons, hostDifferences, differencesByOperation, maximumHostUlps: String(maximumHostUlps), functions: 32, precisionModes: 3 };
}
