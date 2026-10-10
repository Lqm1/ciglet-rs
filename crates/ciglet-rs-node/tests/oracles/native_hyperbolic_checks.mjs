export async function verifyHyperbolicBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected hyperbolic error'); }
    let scalarCases = 0, laneCases = 0, comparisons = 0;
    const names = { sinh: 'sine', cosh: 'cosine', tanh: 'tangent', sigmoid: 'sigmoid' };
    function equal(actual, expected, label) { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; }
    function equalArray(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) equal(actual[index], expected[index], label + '/' + index);
    }
    const text = async filename => new TextDecoder().decode(await loadFixture(filename)).trim().split(/\r?\n/);
    const scalarLines = await text('c-hyperbolic.txt'); assert(scalarLines.length === 128, 'complete scalar C corpus');
    const covered = new Set();
    for (const line of scalarLines) {
        const [name, input, expected] = line.split(/\s+/), fast = name.startsWith('faster');
        const base = name.slice(fast ? 6 : 4), operation = api[`hyperbolic_${names[base]}`];
        assert(operation, 'complete scalar dispatch'); covered.add(name);
        equal(operation(Math.fround(Number(input)), fast ? 0 : 1), Math.fround(Number(expected)), 'original C ' + name); ++scalarCases;
    }
    assert(covered.size === 8, 'all eight source scalar variants');
    const buffer = new ArrayBuffer(4), view = new DataView(buffer);
    const fromBits = field => { view.setUint32(0, parseInt(field, 16), true); return view.getFloat32(0, true); };
    const laneLines = await text('c-lanes-hyperbolic.txt'); assert(laneLines.length === 48, 'complete SSE2 C corpus');
    const laneCovered = new Set();
    for (const line of laneLines) {
        const fields = line.split(/\s+/), name = fields[0], fast = name.startsWith('vfaster');
        const base = name.slice(fast ? 7 : 5), operation = api[`hyperbolic_${names[base]}_lanes`];
        assert(operation, 'complete lane dispatch'); laneCovered.add(name);
        const input = Float32Array.from(fields.slice(1, 5), fromBits), saved = input.slice(), expected = Float32Array.from(fields.slice(5), fromBits);
        const actual = operation(input, fast ? 0 : 1);
        assert(actual instanceof Float32Array, 'owned complete lane result');
        equalArray(actual, expected, 'original SSE2 ' + name); equalArray(input, saved, 'lane source unchanged'); ++laneCases;
    }
    assert(laneCovered.size === 8, 'all eight source lane variants');
    const f = Math.fround;
    for (const name of Object.values(names)) {
        const scalar = api[`hyperbolic_${name}`], lane = api[`hyperbolic_${name}_lanes`];
        for (const precision of [0, 1, 2]) {
            for (const value of [NaN, Infinity, -Infinity]) throws(() => scalar(value, precision));
            for (let slot = 0; slot < 4; ++slot) {
                for (const invalid of [NaN, Infinity, -Infinity, name === 'sine' || name === 'cosine' ? 100 : -100]) {
                    const values = new Float32Array([0, 0.25, -0.5, 1]); values[slot] = invalid;
                    const before = new Uint32Array(values.buffer).slice();
                    throws(() => lane(values, precision));
                    assert(new Uint32Array(values.buffer).every((value, index) => value === before[index]), 'late invalid lane preserves source bits');
                }
            }
            for (const length of [0, 1, 3, 5]) throws(() => lane(new Float32Array(length), precision));
            const input = new Float32Array([-2, -0, 0.5, 2]), saved = input.slice();
            equalArray(lane(input, precision), Float32Array.from(input, value => scalar(value, precision)), 'full scalar/lane precision agreement');
            equalArray(input, saved, 'all precision source retained');
        }
        const exactFormula = value => {
            const positive = f(Math.exp(value)), negative = f(Math.exp(-value));
            if (name === 'sine') return f(0.5 * f(positive - negative));
            if (name === 'cosine') return f(0.5 * f(positive + negative));
            if (name === 'tangent') return f(-1 + f(2 / f(1 + f(Math.exp(f(-2 * value))))));
            return f(1 / f(1 + negative));
        };
        for (const value of [-2, -0, 0, 0.5, 2]) equal(scalar(value, 2), exactFormula(value), 'independent Exact binary32 formula ' + name);
        throws(() => scalar(0, 9)); throws(() => lane(new Float32Array(4), 9));
        const first = lane(new Float32Array(4), 2), second = lane(new Float32Array(4), 2), expected = name === 'cosine' ? 1 : name === 'sigmoid' ? 0.5 : 0;
        first[0] = 73; equalArray(second, [expected, expected, expected, expected], 'independent outputs/post-error reuse');
    }
    return { scalarCases, laneCases, comparisons, functions: 8, precisionModes: 3 };
}
