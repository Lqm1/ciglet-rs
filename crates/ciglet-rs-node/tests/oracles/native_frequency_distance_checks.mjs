export async function verifyFrequencyDistanceBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected conversion/distance error'); }
    let frequencyCases = 0, gridCases = 0, distanceCases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label}/${index}`); ++comparisons;
        }
    }
    assert(api.DistancePolicy.Stable === 0 && api.DistancePolicy.Legacy === 1, 'complete distance policy');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double';
        const toMel = api[`frequency_to_mel_${suffix}`], fromMel = api[`mel_to_frequency_${suffix}`];
        const toBark = api[`frequency_to_bark_${suffix}`], fromBark = api[`bark_to_frequency_${suffix}`];
        const loudness = api[`equal_loudness_${suffix}`], grid = api[`mel_space_${suffix}`];
        const distance = api[`itakura_saito_${suffix}`], log = api[`log_itakura_saito_${suffix}`], withPolicy = api[`log_itakura_saito_with_${suffix}`];
        function reader(bytes, magic, cases) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === cases, 'frequency/distance corpus header');
            return { integer, scalar, plane: count => ArrayType.from({ length: count }, scalar), skip: count => { offset += count; }, done: () => assert(offset === bytes.byteLength, 'frequency/distance corpus trailing bytes') };
        }
        const f = reader(await loadFixture(`c-frequency-${suffix}.bin`), 'FRQ1', 12);
        for (let record = 0; record < 12; ++record) {
            const input = f.scalar(), mel = toMel(input), bark = toBark(input);
            for (const actual of [mel, fromMel(mel), bark, fromBark(bark), loudness(input)]) {
                const expected = f.scalar(), error = Math.abs(actual - expected) / Math.max(1, Math.abs(expected));
                assert(error < (width === 4 ? 1e-7 : 1e-15), 'original C frequency bound'); ++comparisons;
            }
            ++frequencyCases;
        }
        assert(f.integer() === 5, 'Mel grid count');
        for (let record = 0; record < 5; ++record) {
            const count = f.integer(), expected = f.plane(count + 1), actual = grid(50, 8000, count);
            assert(actual instanceof ArrayType, 'Mel typed array'); equal(actual, expected, 'original C Mel grid'); ++gridCases;
        }
        // Approximation scalar functions remain separately pending; parse only.
        assert(f.integer() === 12, 'approximation log count'); f.skip(12 * 3 * 4);
        assert(f.integer() === 12, 'approximation exponential count'); f.skip(12 * 4 * 4); f.done();
        const d = reader(await loadFixture(`distance-${precision}.bin`), 'ISD1', 53);
        for (let record = 0; record < 53; ++record) {
            const count = d.integer(), source = d.plane(count), reference = d.plane(count), expected = d.scalar();
            const saved = [source.slice(), reference.slice()];
            equal([withPolicy(source, reference, 1)], [expected], 'original C Legacy log distance');
            equal(source, saved[0], 'distance source unchanged'); equal(reference, saved[1], 'distance reference unchanged'); ++distanceCases;
        }
        d.done();
        for (const ratio of [0.125, 0.5, 2, 8]) {
            const source = new ArrayType([ratio, ratio, ratio]), reference = new ArrayType([1, 1, 1]);
            const expected = ratio - Math.log(ratio) - 1, actual = distance(source, reference), logarithm = log(source, reference);
            assert(Math.abs(actual - expected) <= (width === 4 ? 2e-7 : 2e-15) * expected, 'independent analytic divergence');
            assert(Math.abs(logarithm - Math.log(expected)) < (width === 4 ? 2e-7 : 1e-15), 'independent analytic log divergence');
            equal([actual], [distance(new ArrayType([ratio * 0.25, ratio * 0.25, ratio * 0.25]), new ArrayType([0.25, 0.25, 0.25]))], 'scale invariance');
            equal([logarithm], [withPolicy(source, reference, 0)], 'default stable policy');
        }
        equal([distance(new ArrayType([1]), new ArrayType([1])), log(new ArrayType([1]), new ArrayType([1]))], [0, -Infinity], 'identical spectra');
        for (const [source, reference] of [[0, 1], [1, 0]]) equal([distance(new ArrayType([source]), new ArrayType([reference])), log(new ArrayType([source]), new ArrayType([reference]))], [Infinity, Infinity], 'single zero limit');
        for (const operation of [distance, log]) {
            throws(() => operation(new ArrayType([0]), new ArrayType([0])));
            throws(() => operation(new ArrayType(0), new ArrayType(0))); throws(() => operation(new ArrayType([1]), new ArrayType(0)));
            for (const invalid of [-1, NaN, Infinity]) throws(() => operation(new ArrayType([invalid]), new ArrayType([1])));
        }
        const max = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE, tiny = width === 4 ? 2 ** -149 : Number.MIN_VALUE;
        assert(Number.isFinite(log(new ArrayType([max]), new ArrayType([tiny]))) && distance(new ArrayType([max]), new ArrayType([tiny])) === Infinity, 'finite extreme log vs divergent mean');
        const adjacent = width === 4 ? Math.fround(1 + 2 ** -23) : 1 + Number.EPSILON;
        assert(distance(new ArrayType([adjacent]), new ArrayType([1])) > 0, 'adjacent positive divergence');
        throws(() => withPolicy(new ArrayType([1]), new ArrayType([1]), 9));
        equal([loudness(0)], [0], 'zero equal loudness');
        for (const input of [-8000, -440, 0, 440, 8000]) assert(Math.abs(fromBark(toBark(input)) - input) < (width === 4 ? 0.003 : 1e-10), 'signed Bark roundtrip');
        for (const invalid of [NaN, Infinity, -Infinity]) { throws(() => toBark(invalid)); throws(() => fromMel(invalid)); throws(() => loudness(invalid)); }
        throws(() => toMel(-700)); throws(() => loudness(max));
        for (const count of [0, 2 ** 53, Infinity]) throws(() => grid(0, 8000, count));
        throws(() => grid(1000, 50, 4));
        const first = grid(50, 8000, 4), second = grid(50, 8000, 4); first[0] = 73;
        assert(second[0] !== 73 && second.length === 5, 'independent grid and post-error reuse');
    }
    return { frequencyCases, gridCases, distanceCases, comparisons, functions: 18, enums: 1 };
}
