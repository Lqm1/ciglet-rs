export async function verifyTrigonometricBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected trigonometric error'); }
    let scalarCases = 0, laneCases = 0, comparisons = 0;
    const buffer = new ArrayBuffer(4), view = new DataView(buffer);
    const fromBits = field => { view.setUint32(0, parseInt(field, 16), true); return view.getFloat32(0, true); };
    const names = { sin: 'sine', cos: 'cosine', tan: 'tangent', sinfull: 'sine_full', cosfull: 'cosine_full', tanfull: 'tangent_full' };
    function equal(actual, expected, label) { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; }
    function arrayEqual(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) equal(actual[index], expected[index], label + '/' + index);
    }
    const text = async filename => new TextDecoder().decode(await loadFixture(filename)).trim().split(/\r?\n/);
    const scalarLines = await text('c-trigonometric.txt'); assert(scalarLines.length === 456, 'full original scalar fixture');
    const covered = new Set();
    for (const line of scalarLines) {
        const fields = line.split(/\s+/), name = fields[0], fast = name.startsWith('faster'), base = name.slice(fast ? 6 : 4);
        if (!['sin', 'cos', 'tan', 'tanfull'].includes(base)) continue;
        const expected = fromBits(fields[2]), value = fromBits(fields[1]), operation = api[`trigonometric_${names[base]}`];
        covered.add(name);
        if (Number.isFinite(expected)) equal(operation(value, fast ? 0 : 1), expected, 'original C ' + name);
        else throws(() => operation(value, fast ? 0 : 1));
        ++scalarCases;
    }
    assert(covered.size === 8, 'all eight native scalar source variants');
    const laneLines = await text('c-lanes-trigonometric.txt'); assert(laneLines.length === 144, 'full original SSE2 fixture');
    const laneCovered = new Set();
    for (const line of laneLines) {
        const fields = line.split(/\s+/), name = fields[0], fast = name.startsWith('vfaster'), base = name.slice(fast ? 7 : 5);
        const operation = api[`trigonometric_${names[base]}_lanes`]; assert(operation, 'full lane dispatch'); laneCovered.add(name);
        const input = Float32Array.from(fields.slice(1, 5), fromBits), expected = Float32Array.from(fields.slice(5), fromBits), saved = new Uint32Array(input.buffer).slice();
        if (expected.every(Number.isFinite)) {
            const actual = operation(input, fast ? 0 : 1); assert(actual instanceof Float32Array, 'complete owned lane output');
            arrayEqual(actual, expected, 'original SSE2 ' + name);
        } else throws(() => operation(input, fast ? 0 : 1));
        assert(new Uint32Array(input.buffer).every((value, index) => value === saved[index]), 'source bits unchanged'); ++laneCases;
    }
    assert(laneCovered.size === 12, 'all twelve original lane variants');
    const halfPi = Math.fround(Math.PI / 2), equalityInput = new Float32Array([halfPi, halfPi, halfPi, halfPi]);
    arrayEqual(api.trigonometric_cosine_lanes(equalityInput, 1), [0, 0, 0, 0], 'vector-specific equality branch');
    assert(api.trigonometric_cosine(halfPi, 1) !== 0, 'scalar strict-greater branch retained');
    for (const name of ['sine', 'cosine', 'tangent', 'sine_full', 'cosine_full', 'tangent_full']) {
        const lane = api[`trigonometric_${name}_lanes`], scalar = api[`trigonometric_${name}`];
        for (const mode of [0, 1, 2]) {
            for (const length of [0, 1, 3, 5]) throws(() => lane(new Float32Array(length), mode));
            for (let slot = 0; slot < 4; ++slot) {
                for (const invalid of [NaN, Infinity, -Infinity]) {
                    const input = new Float32Array([0.25, -0.5, 0.75, -1]); input[slot] = invalid;
                    const saved = new Uint32Array(input.buffer).slice(); throws(() => lane(input, mode));
                    assert(new Uint32Array(input.buffer).every((value, index) => value === saved[index]), 'late lane error preserves bits');
                    if (scalar) throws(() => scalar(invalid, mode));
                }
                if (mode !== 2 && name.endsWith('_full')) {
                    for (const invalid of [Math.fround(3.4028234663852886e38), -Math.fround(3.4028234663852886e38), 2e10, -2e10]) {
                        const input = new Float32Array(4); input[slot] = invalid; throws(() => lane(input, mode));
                        if (scalar) throws(() => scalar(invalid, mode));
                    }
                }
            }
        }
        const input = new Float32Array([-2, -0, 0.5, 2]), expected = Float32Array.from(input, value => Math[name.startsWith('sine') ? 'sin' : name.startsWith('cosine') ? 'cos' : 'tan'](value));
        arrayEqual(lane(input, 2), expected, 'independent Exact native math ' + name);
        if (scalar) for (let index = 0; index < 4; ++index) equal(scalar(input[index], 2), expected[index], 'scalar Exact');
        throws(() => lane(input, 9)); if (scalar) throws(() => scalar(0, 9));
        const first = lane(input, 2), second = lane(input, 2); first[0] = 73;
        arrayEqual(second, expected, 'independent output/post-error reuse');
    }
    for (const mode of [0, 1]) {
        const outside = api.trigonometric_sine(4, mode), full = api[`approximation_sine${mode === 0 ? '_fast' : ''}`](4);
        assert(Number.isFinite(outside) && outside !== full, 'polynomial beyond calibration has no periodic reduction');
    }
    return { scalarCases, laneCases, comparisons, functions: 10, precisionModes: 3 };
}
