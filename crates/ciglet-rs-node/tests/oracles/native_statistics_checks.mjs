export async function verifyStatisticsBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected statistics error'); }
    let originalCases = 0, stableCases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label}/${index}: ${actual[index]} vs ${expected[index]}`); ++comparisons;
        }
    }
    assert(api.StatisticsPolicy.Stable === 0 && api.StatisticsPolicy.Legacy === 1, 'complete policy enum');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double';
        const unary = ['sum', 'sum_squares', 'mean', 'variance'].map(name => api[`${name}_${suffix}`]);
        const meanWith = api[`mean_with_${suffix}`], varianceWith = api[`variance_with_${suffix}`];
        const dot = api[`dot_${suffix}`], covariance = api[`covariance_${suffix}`], covarianceWith = api[`covariance_with_${suffix}`];
        const correlation = api[`correlation_${suffix}`], correlationWith = api[`correlation_with_${suffix}`], cross = api[`cross_correlation_${suffix}`];
        function reader(bytes, magic, cases) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === cases, 'statistics corpus header');
            return { integer, scalar, plane: count => ArrayType.from({ length: count }, scalar), done: () => assert(offset === bytes.byteLength, 'statistics corpus trailing bytes') };
        }
        const r = reader(await loadFixture(`statistics-${precision}.bin`), 'STA1', 32);
        for (let record = 0; record < 32; ++record) {
            const count = r.integer(), lags = r.integer(), left = r.plane(count), right = r.plane(count), expected = r.plane(9);
            const savedLeft = left.slice(), savedRight = right.slice();
            const actual = [unary[0](left), unary[1](left), meanWith(left, 1), varianceWith(left, 1), dot(left, right), covarianceWith(left, right, 1)];
            equal(actual, [expected[0], expected[1], expected[2], expected[3], expected[7], expected[8]], 'original C statistics');
            const valid = r.integer(), expectedCorrelation = r.scalar();
            if (valid) equal([correlationWith(left, right, 1)], [expectedCorrelation], 'original C correlation');
            else throws(() => correlationWith(left, right, 1));
            // Ordering functions remain separate pending bindings; parse their fields.
            r.plane(count); r.plane(count); for (let index = 0; index < count; ++index) r.integer();
            equal(cross(left, right, lags), r.plane(lags), 'original C cross correlation');
            equal(left, savedLeft, 'left unchanged'); equal(right, savedRight, 'right unchanged'); ++originalCases;
        }
        r.done();
        const s = reader(await loadFixture(`stable_statistics-${precision}.bin`), 'SMT1', 48);
        for (let record = 0; record < 48; ++record) {
            const count = s.integer(), left = s.plane(count), right = s.plane(count);
            const selected = [() => meanWith(left, 1), () => varianceWith(left, 1), () => covarianceWith(left, right, 1), () => correlationWith(left, right, 1)];
            for (const action of selected) {
                const code = s.integer(), expected = s.scalar();
                if (code === 4) throws(action);
                else {
                    const actual = action();
                    if (code === 0) equal([actual], [expected], 'original C extreme moment');
                    else { assert(code === 1 ? actual === Infinity : code === 2 ? actual === -Infinity : code === 3 && Number.isNaN(actual), 'source nonfinite classification'); ++comparisons; }
                }
            }
            const defaults = [() => unary[2](left), () => unary[3](left), () => covariance(left, right), () => correlation(left, right)];
            const stable = [() => meanWith(left, 0), () => varianceWith(left, 0), () => covarianceWith(left, right, 0), () => correlationWith(left, right, 0)];
            for (let operation = 0; operation < 4; ++operation) {
                const valid = s.integer(), expected = s.scalar();
                if (!valid) { throws(defaults[operation]); throws(stable[operation]); }
                else {
                    const actual = defaults[operation]();
                    equal([actual], [stable[operation]()], 'default stable policy');
                    if (expected === 0) assert(actual === 0, 'zero rational moment');
                    else assert(Math.abs(actual / expected - 1) <= (width === 4 ? 2e-7 : 4e-15), 'independent rational stable moment');
                    ++comparisons;
                }
            }
            ++stableCases;
        }
        s.done();
        equal([unary[0](new ArrayType(0)), unary[0](new ArrayType([-0])), unary[1](new ArrayType(0)), dot(new ArrayType([-0]), new ArrayType([1]))], [0, 0, 0, 0], 'positive-zero reduction identity');
        equal(cross(new ArrayType(0), new ArrayType(0), 3), new ArrayType([0, 0, 0]), 'empty zero-filled lags');
        equal(cross(new ArrayType([1, 2]), new ArrayType([3, 4]), 4), new ArrayType([11, 6, 0, 0]), 'directed excess lags');
        for (const operation of [unary[2], unary[3]]) { throws(() => operation(new ArrayType(0))); throws(() => operation(new ArrayType([NaN]))); }
        for (const operation of [dot, covariance, correlation]) throws(() => operation(new ArrayType([1]), new ArrayType(0)));
        throws(() => correlation(new ArrayType([1, 1]), new ArrayType([1, 2])));
        throws(() => cross(new ArrayType(0), new ArrayType(0), 0xffffffff));
        throws(() => meanWith(new ArrayType([1]), 9));
        const max = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        equal([unary[2](new ArrayType([max, max])), unary[3](new ArrayType([max, max]))], [max, 0], 'constant extreme stable moments');
        assert(meanWith(new ArrayType([max, max]), 1) === Infinity, 'Legacy overflow retained');
        throws(() => unary[3](new ArrayType([-max, max])));
        assert(Math.abs(correlation(new ArrayType([-max, max]), new ArrayType([-max, max])) - 1) <= (width === 4 ? 2e-7 : 4e-16), 'scaled correlation survives variance overflow');
        const first = cross(new ArrayType([1, 2]), new ArrayType([3, 4]), 4), second = cross(new ArrayType([1, 2]), new ArrayType([3, 4]), 4);
        first[0] = 73; equal(second, new ArrayType([11, 6, 0, 0]), 'independent arrays and error recovery');
        assert(Number.isNaN(unary[0](new ArrayType([NaN]))) && dot(new ArrayType([Infinity]), new ArrayType([1])) === Infinity, 'unvalidated IEEE reductions retained');
    }
    return { originalCases, stableCases, comparisons, functions: 24, enums: 1 };
}
