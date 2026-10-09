export async function verifyFilteringBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected filtering error'); }
    let firCases = 0, convolutionCases = 0, recursiveCases = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label} bits at ${index}: ${actual[index]} vs ${expected[index]}`); ++comparisons;
        }
    }
    assert(api.FilterPassband.Lowpass === 0 && api.FilterPassband.Highpass === 1 && api.FilterPassband.Bandpass === 2 && api.FilterPassband.Bandstop === 3, 'complete passband enum');
    assert(api.FilterStartup.ZeroState === 0 && api.FilterStartup.LegacyZeroPrefix === 1, 'complete startup enum');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const windowed = api[`windowed_fir_${suffix}`], fir = api[`fir_${suffix}`], band = api[`bandpass_fir_${suffix}`];
        const convolve = api[`convolve_${suffix}`], filter = api[`filter_${suffix}`], withStartup = api[`filter_with_${suffix}`];
        const backward = api[`forward_backward_filter_${suffix}`], backwardWith = api[`forward_backward_filter_with_${suffix}`];
        function reader(bytes, magic) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width, 'filter corpus header');
            return { integer, scalar, plane: count => ArrayType.from({ length: count }, scalar), done: () => assert(offset === bytes.byteLength, 'filter corpus trailing bytes') };
        }
        function bounded(actual, expected, label) {
            assert(actual instanceof ArrayType && actual.length === expected.length, label + ' typed shape');
            for (let index = 0; index < actual.length; ++index) {
                const error = Math.abs(actual[index] - expected[index]) / Math.max(1, Math.abs(expected[index]));
                assert(error < (width === 4 ? 2e-7 : 2e-15), `${label}/${index}: ${error}`); ++comparisons;
            }
        }
        const r = reader(await loadFixture(`c-filter-resample-${suffix}.bin`), 'FRS1');
        assert(r.integer() === 300 && r.integer() === 60 && r.integer() === 105, 'FRS1 counts');
        for (let record = 0; record < 300; ++record) {
            const count = r.integer(), window = r.integer(), kind = r.integer(), lower = r.scalar(), upper = r.scalar(), expected = r.plane(count);
            const actual = windowed(count, kind, lower, upper, window);
            bounded(actual, expected, 'original C FIR');
            if (kind < 2) equal(actual, fir(count, lower * 2, kind === 1, window), 'Nyquist-normalized FIR');
            if (kind === 2) equal(actual, band(count, lower * 2, upper * 2, window), 'Nyquist-normalized bandpass');
            ++firCases;
        }
        for (let record = 0; record < 60; ++record) {
            const signalCount = r.integer(), kernelCount = r.integer(), signal = r.plane(signalCount), impulse = r.plane(kernelCount), expected = r.plane(signalCount + kernelCount - 1);
            const savedSignal = signal.slice(), savedImpulse = impulse.slice();
            bounded(convolve(signal, impulse), expected, 'original C convolution');
            equal(signal, savedSignal, 'convolution signal unchanged'); equal(impulse, savedImpulse, 'convolution impulse unchanged'); ++convolutionCases;
        }
        // Resampling has its own complete checks; consume remaining records only.
        for (let record = 0; record < 105; ++record) {
            const count = r.integer(); r.scalar(); r.plane(count); r.plane(r.integer());
        }
        r.done();
        const iir = reader(await loadFixture(`recursive-filter-${width === 4 ? 'float' : 'double'}.bin`), 'IIR1');
        assert(iir.integer() === 210, 'IIR1 count');
        for (let record = 0; record < 210; ++record) {
            const nb = iir.integer(), na = iir.integer(), nx = iir.integer();
            const numerator = iir.plane(nb), denominator = iir.plane(na), signal = iir.plane(nx);
            const saved = [signal.slice(), numerator.slice(), denominator.slice()];
            for (const startup of [api.FilterStartup.LegacyZeroPrefix, api.FilterStartup.ZeroState]) {
                const causal = iir.plane(nx), twoPass = iir.plane(nx);
                equal(withStartup(signal, numerator, denominator, startup), causal, 'original/disclosed-corrected C causal filter');
                equal(backwardWith(signal, numerator, denominator, startup), twoPass, 'original/disclosed-corrected C two-pass filter');
                if (startup === api.FilterStartup.ZeroState) {
                    equal(filter(signal, numerator, denominator), causal, 'default corrected startup');
                    equal(backward(signal, numerator, denominator), twoPass, 'default two-pass startup');
                }
            }
            [signal, numerator, denominator].forEach((plane, index) => equal(plane, saved[index], 'recursive input unchanged'));
            ++recursiveCases;
        }
        iir.done();
        for (const count of [7, 8]) for (const kind of [0, 1, 2, 3]) {
            const boxcar = windowed(count, kind, 0.125, 0.375, api.WindowKind.Boxcar);
            for (let window = 0; window < 7; ++window) {
                const weights = api[`window_${suffix}`](window, count, api.WindowPrecision.Accurate);
                const expected = ArrayType.from(boxcar, (value, index) => value * weights[index]);
                equal(windowed(count, kind, 0.125, 0.375, window), expected, 'all seven windows and four passbands');
            }
        }
        equal(convolve(new ArrayType([1, 2, 3]), new ArrayType([2, -1])), new ArrayType([2, 3, 4, -3]), 'independent convolution');
        assert(convolve(new ArrayType(0), new ArrayType([1])).length === 0, 'empty convolution');
        equal(filter(new ArrayType([1, 0, 0]), new ArrayType([0.5, 0]), new ArrayType([1, -0.5])), new ArrayType([0.5, 0.25, 0.125]), 'independent impulse recurrence');
        equal(withStartup(new ArrayType([1, 0, 0]), new ArrayType([0.5, 0]), new ArrayType([1, -0.5]), 1), new ArrayType([0, 0, 0]), 'legacy skipped prefix');
        equal(filter(new ArrayType([1]), new ArrayType([0.5]), new ArrayType([2])), new ArrayType([0.5]), 'monic convention ignores denominator zero');
        equal(backward(new ArrayType([1, 2, 3]), new ArrayType([1, 2]), new ArrayType(0)), new ArrayType([9, 18, 7]), 'independent two-pass arithmetic');
        equal(filter(new ArrayType([1, 2]), new ArrayType(0), new ArrayType(0)), new ArrayType([0, 0]), 'empty numerator');
        assert(filter(new ArrayType(0), new ArrayType(0), new ArrayType(0)).length === 0, 'empty recursive input');
        equal(windowed(8, 0, 0.125, NaN, 0), windowed(8, 0, 0.125, 0, 0), 'unused high cutoff retained');
        for (const count of [0, 1, 0xffffffff]) throws(() => windowed(count, 0, 0.125, 0.375, 0));
        for (const [lower, upper] of [[NaN, 0.3], [-0.1, 0.3], [0.4, 0.3], [0.1, Infinity], [0.1, 0.6]]) throws(() => windowed(8, 2, lower, upper, 0));
        throws(() => windowed(8, 9, 0.1, 0.3, 0)); throws(() => windowed(8, 0, 0.1, 0.3, 9));
        throws(() => withStartup(new ArrayType([1]), new ArrayType([1]), new ArrayType([1]), 9));
        const max = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        for (const operation of [filter, backward]) {
            for (const invalid of [NaN, Infinity, -Infinity]) {
                throws(() => operation(new ArrayType([invalid]), new ArrayType([1]), new ArrayType([1])));
                throws(() => operation(new ArrayType([1]), new ArrayType([invalid]), new ArrayType([1])));
                throws(() => operation(new ArrayType([1]), new ArrayType([1]), new ArrayType([invalid])));
            }
            throws(() => operation(new ArrayType([max]), new ArrayType([2]), new ArrayType(0)));
        }
        throws(() => convolve(new ArrayType([NaN]), new ArrayType([1])));
        const first = filter(new ArrayType([1]), new ArrayType([0.5]), new ArrayType([1]));
        const second = filter(new ArrayType([1]), new ArrayType([0.5]), new ArrayType([1]));
        first[0] = 73; equal(second, new ArrayType([0.5]), 'independent output and post-error reuse');
    }
    return { firCases, convolutionCases, recursiveCases, comparisons, functions: 16, enums: 2 };
}
