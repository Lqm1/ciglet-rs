export async function verifySincBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected sinc error'); }
    let cases = 0, comparisons = 0;
    function close(actual, expected, bound, label) {
        const error = Math.abs(actual - expected) / (1 + Math.abs(expected));
        assert(Number.isFinite(actual) && error <= bound, `${label}: ${actual} != ${expected}; error ${error}`);
        if (bound === 0 || actual === 0 || expected === 0) assert(Object.is(actual, expected), label + ' exact zero/bits');
        ++comparisons;
    }
    assert(api.SincPolicy.Stable === 0 && api.SincPolicy.Legacy === 1, 'complete policy');
    for (const width of [4, 8]) {
        const suffix = `f${width * 8}`, precision = width === 4 ? 'float' : 'double', round = width === 4 ? Math.fround : value => value;
        const value = api[`aliased_sinc_${suffix}`], selected = api[`aliased_sinc_with_${suffix}`];
        const derivative = api[`aliased_sinc_derivative_${suffix}`], selectedDerivative = api[`aliased_sinc_derivative_with_${suffix}`];
        const bytes = await loadFixture(`sinc-${precision}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'SNC1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 192, 'complete sinc corpus header');
        let offset = 12;
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < 192; ++record) {
            const length = scalar(), angle = scalar(), expected = scalar(), expectedDerivative = scalar();
            close(selected(length, angle, 1), expected, width === 4 ? 0 : 2e-16, 'original C sinc');
            close(selectedDerivative(length, angle, 1), expectedDerivative, width === 4 ? 0 : 5e-16, 'original C derivative'); ++cases;
        }
        assert(offset === bytes.byteLength, 'full sinc corpus consumed');
        for (const angle of [-1, -1e-4, -1e-8, 0, 1e-12, 1e-8, 1e-4, 1]) {
            const a = round(angle), bound = width === 4 ? 1e-7 : 3e-15;
            close(value(2, a), round(2 * Math.cos(a * 0.5)), bound, 'independent length2 formula');
            close(derivative(2, a), round(-Math.sin(a * 0.5)), bound, 'independent length2 derivative');
            close(value(3, a), round(1 + 2 * Math.cos(a)), bound, 'independent length3 formula');
            if (a !== 0) close(derivative(3, a), round(-2 * Math.sin(a)), bound, 'independent length3 derivative');
            close(value(2, a), selected(2, a, 0), 0, 'default stable'); close(derivative(2, a), selectedDerivative(2, a, 0), 0, 'default stable derivative');
            close(value(1, a), 1, 0, 'unit length'); close(derivative(-1, a), 0, 0, 'negative unit derivative');
        }
        for (const policy of [0, 1]) {
            for (const [length, angle] of [[NaN, 0], [1, Infinity], [Infinity, 0], [1, NaN]]) {
                throws(() => selected(length, angle, policy)); throws(() => selectedDerivative(length, angle, policy));
            }
            const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
            close(selected(largest, 0, policy), round(largest), 0, 'maximum length zero limit');
            close(selectedDerivative(largest, 0, policy), policy === 0 ? -0 : 0, 0, 'maximum length derivative zero limit');
            if (width === 8 || policy === 1) throws(() => selected(largest, 4, policy));
            else assert(Number.isFinite(selected(largest, 4, policy)), 'binary32 stable double intermediate remains finite');
        }
        throws(() => selected(1, 0, 9)); throws(() => selectedDerivative(1, 0, 9));
        close(value(0.5, 0), 0.5, 0, 'fractional length zero limit'); close(value(0, 1), 0, 0, 'zero length');
        close(selectedDerivative(3, round(1e-8), 1), 0, 0, 'legacy derivative plateau');
    }
    for (const angle of [-2 * Math.PI, 2 * Math.PI, 4 * Math.PI]) {
        close(api.aliased_sinc_f64(2, angle), 2 * Math.cos(angle * 0.5), 2e-15, 'stable removable integer pole');
        close(api.aliased_sinc_f64(3, angle), 3, 0, 'odd length pole');
    }
    close(api.aliased_sinc_with_f64(2, 2 * Math.PI, 1), 2, 0, 'legacy pole remains plateau');
    close(api.aliased_sinc_f64(-2, 2 * Math.PI), 2, 0, 'negative even pole');
    assert(Math.abs(api.aliased_sinc_derivative_f64(3, 1e-8) + 2e-8) < 1e-23, 'small-angle derivative avoids legacy plateau');
    assert(Math.abs(api.aliased_sinc_derivative_f64(0.5, 1e-8) - 3.125e-10) < 1e-25, 'fractional small-angle derivative');
    assert(api.aliased_sinc_f64(0.5, 2 * Math.PI) > 1e15, 'fractional nonremovable pole not clamped');
    return { cases, comparisons, functions: 8, policies: 2 };
}
