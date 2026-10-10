export async function verifySpecialBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected special-function error'); }
    let scalarCases = 0, laneCases = 0, comparisons = 0, correctedValues = 0;
    const names = { fasterfc: 'complementary_error', fastererfc: 'complementary_error_fast', fasterf: 'error', fastererf: 'error_fast',
        fastinverseerf: 'inverse_error', fasterinverseerf: 'inverse_error_fast', fastlgamma: 'log_gamma', fasterlgamma: 'log_gamma_fast',
        fastdigamma: 'digamma', fasterdigamma: 'digamma_fast', fastlambertw: 'lambert_w', fasterlambertw: 'lambert_w_fast',
        fastlambertwexpx: 'lambert_w_exp', fasterlambertwexpx: 'lambert_w_exp_fast' };
    const buffer = new ArrayBuffer(4), view = new DataView(buffer);
    const fromBits = field => { view.setUint32(0, parseInt(field, 16), true); return view.getFloat32(0, true); };
    const bits = value => { view.setFloat32(0, value, true); return view.getUint32(0, true); };
    function equal(actual, expected, label) { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; }
    function arrayEqual(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) equal(actual[index], expected[index], label + '/' + index);
    }
    const text = async filename => new TextDecoder().decode(await loadFixture(filename)).trim().split(/\r?\n/);
    const roots = new Map((await text('lambert-exp-corrected.txt')).map(line => line.split(/\s+/).map(field => parseInt(field, 16))));
    function corrected(actual, inputBits, label) {
        const root = roots.get(inputBits); assert(root !== undefined, 'independent corrected Lambert reference');
        assert(Number.isFinite(actual) && actual >= 0 && Math.abs(bits(actual) - root) <= 1, label + ' existing corrected one-ULP bound');
        if (root === 0) equal(actual, 0, 'positive zero underflow');
        ++correctedValues;
    }
    const scalarLines = await text('c-special.txt'); assert(scalarLines.length === 232, 'complete original scalar fixture');
    const scalarCovered = new Set();
    for (const line of scalarLines) {
        const [name, input, output] = line.split(/\s+/), operation = api[`special_${names[name]}`]; assert(operation, 'full scalar dispatch'); scalarCovered.add(name);
        const expected = fromBits(output), actual = operation(fromBits(input));
        if (name.endsWith('lambertwexpx') && expected <= 0) {
            assert(actual > 0 && Number.isFinite(actual), 'retained scalar Lambert correction'); ++correctedValues;
        } else equal(actual, expected, 'original C ' + name);
        ++scalarCases;
    }
    assert(scalarCovered.size === 14, 'all fourteen scalar kernels');
    const laneLines = await text('c-lanes-special.txt'); assert(laneLines.length === 67, 'complete original SSE2 fixture');
    const laneCovered = new Set();
    for (const line of laneLines) {
        const fields = line.split(/\s+/), name = fields[0].slice(1), operation = api[`special_${names[name]}_lanes`]; assert(operation, 'full lane dispatch'); laneCovered.add(name);
        const input = Float32Array.from(fields.slice(1, 5), fromBits), saved = new Uint32Array(input.buffer).slice();
        const actual = operation(input); assert(actual instanceof Float32Array && actual.length === 4, 'complete owned lane output');
        for (let slot = 0; slot < 4; ++slot) {
            const expected = fromBits(fields[slot + 5]);
            if (name.endsWith('lambertwexpx') && (!Number.isFinite(expected) || expected <= 0)) corrected(actual[slot], saved[slot], 'corrected SSE2 Lambert');
            else equal(actual[slot], expected, 'original SSE2 ' + name);
        }
        assert(new Uint32Array(input.buffer).every((value, index) => value === saved[index]), 'source bits unchanged'); ++laneCases;
    }
    assert(laneCovered.size === 14, 'all fourteen lane kernels');
    for (const name of Object.values(names)) {
        const scalar = api[`special_${name}`], lane = api[`special_${name}_lanes`];
        for (const invalid of [NaN, Infinity, -Infinity]) throws(() => scalar(invalid));
        for (const length of [0, 1, 3, 5]) throws(() => lane(new Float32Array(length)));
        for (let slot = 0; slot < 4; ++slot) {
            const invalids = [NaN, Infinity, -Infinity];
            if (name.startsWith('inverse_error')) invalids.push(-1, 1, -2, 2);
            if (name.startsWith('log_gamma') || name.startsWith('digamma')) invalids.push(0, -0);
            if (name === 'lambert_w' || name === 'lambert_w_fast') invalids.push(-0.5);
            if (name.startsWith('error') || name.startsWith('complementary_error')) invalids.push(40);
            for (const invalid of invalids) {
                const input = new Float32Array([0.25, 0.5, 0.75, 0.125]); input[slot] = invalid;
                const saved = new Uint32Array(input.buffer).slice(); throws(() => lane(input)); throws(() => scalar(invalid));
                assert(new Uint32Array(input.buffer).every((value, index) => value === saved[index]), 'late invalid lane immutable');
            }
        }
        const input = new Float32Array([0.125, 0.25, 0.5, 0.75]), expected = Float32Array.from(input, scalar);
        const first = lane(input), second = lane(input); arrayEqual(first, expected, 'native scalar/lane agreement'); first[0] = 73;
        arrayEqual(second, expected, 'independent output/error recovery');
    }
    for (const [name, values] of [['log_gamma', [-1.75, -1.5, -1.25, 1]], ['digamma', [-1.75, -1.5, -0.75, -0.25]], ['digamma_fast', [-0.75, -0.5, -0.25, 1]]]) {
        const input = new Float32Array(values), scalar = api[`special_${name}`];
        arrayEqual(api[`special_${name}_lanes`](input), Float32Array.from(input, scalar), 'source-defined negative intervals');
    }
    for (const name of ['lambert_w_exp', 'lambert_w_exp_fast']) {
        const input = new Float32Array([-1000, -200, -40, 3.4028234663852886e38]), actual = api[`special_${name}_lanes`](input);
        assert(actual.every(value => Number.isFinite(value) && value >= 0), 'extreme corrected values finite/nonnegative');
        equal(actual[0], 0, 'underflow positive zero'); equal(actual[1], 0, 'underflow positive zero');
        equal(actual[3], input[3], 'avoid forming extreme exponential');
    }
    return { scalarCases, laneCases, comparisons, correctedValues, functions: 28 };
}
