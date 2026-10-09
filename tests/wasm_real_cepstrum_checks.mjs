export async function verifyRealCepstrumBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected real cepstrum error'); };
    let cases = 0, comparisons = 0; const maxima = [Array(7).fill(0), Array(7).fill(0)];
    const exact = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    const same = (actual, expected, label) => { assert(actual.length === expected.length, label + ' shape'); actual.forEach((value, index) => exact(value, expected[index], label)); };
    assert(api.CepstralLifter.Causal === 0 && api.CepstralLifter.Legacy === 1 && api.SpectrumNyquist.Preserve === 0 && api.SpectrumNyquist.Legacy === 1, 'complete lifter and Nyquist constants');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, Plan = api[`RealCepstrumPlanF${width * 8}`], suffix = `f${width * 8}`;
        const from = api[`real_cepstrum_from_log_spectrum_${suffix}`], to = api[`real_cepstrum_to_log_spectrum_${suffix}`], minimum = api[`real_cepstrum_minimum_phase_${suffix}`];
        const bounds = width === 4 ? [3e-8, 1.2e-7, 2e-7, 1.6e-7, 7e-9, 0, 2.5e-6] : [5e-17, 3e-16, 3e-16, 3e-16, 3e-17, 0, 0];
        const bytes = await loadFixture(`real-cepstrum-${width === 4 ? 'float' : 'double'}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'RCP1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 72, 'complete real cepstrum corpus');
        let offset = 12;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const values = length => new ArrayType(Array.from({length}, scalar));
        const compare = (actual, expected, field) => {
            assert(actual.length === expected.length, 'C output shape');
            actual.forEach((value, index) => { const error = Math.abs(value - expected[index]) / (1 + Math.abs(expected[index])); maxima[width === 4 ? 0 : 1][field] = Math.max(maxima[width === 4 ? 0 : 1][field], error); assert(Number.isFinite(value) && error <= bounds[field], `unchanged real cepstrum bound width${width} field${field} index${index} error${error}`); ++comparisons; });
        };
        for (let record = 0; record < 72; ++record) {
            const length = integer(), count = integer(), bins = length / 2 + 1, logs = values(bins), coefficients = values(length), inverse = values(length), phase = values(length), magnitudes = values(3 * bins), matrixCoefficients = values(3 * count), matrixSpectra = values(3 * bins), plan = new Plan(length);
            exact(plan.length, length, 'plan length'); exact(plan.bins, bins, 'plan bins'); assert(!plan.is_empty(), 'positive plan');
            const saved = [logs.slice(), coefficients.slice(), magnitudes.slice(), matrixCoefficients.slice()];
            const actual = plan.analyze(logs, 1); compare(actual, coefficients, 0); compare(plan.reconstruct(coefficients.subarray(0, count), 1), inverse, 1); compare(plan.reconstruct(actual.subarray(0, count), 1), inverse, 2); compare(plan.minimum_phase_with(logs, 1, 1), phase, 3);
            const actualMatrix = plan.spectrogram_to_cepstra(magnitudes, count, 1); compare(actualMatrix, matrixCoefficients, 4); compare(plan.cepstra_to_spectrogram(matrixCoefficients, 3, count, 1), matrixSpectra, 5); compare(plan.cepstra_to_spectrogram(actualMatrix, 3, count, 1), matrixSpectra, 6);
            same(from(logs), plan.analyze(logs, 0), 'Preserve one-shot'); same(to(coefficients, length), plan.reconstruct(coefficients, 0), 'reconstruct one-shot'); same(minimum(logs), plan.minimum_phase_with(logs, 0, 0), 'causal one-shot');
            [logs, coefficients, magnitudes, matrixCoefficients].forEach((array, index) => same(array, saved[index], 'input unchanged')); plan.free(); assert(actual.length === length && actualMatrix.length === 3 * count, 'arrays outlive plan'); ++cases;
        }
        assert(offset === bytes.byteLength, 'full RCP1 corpus consumed');
        for (const length of [0, 3, 0xffffffff]) throws(() => new Plan(length)); throws(() => from(new ArrayType())); throws(() => minimum(new ArrayType([1])));
        const plan = new Plan(4), zeros = new ArrayType(3);
        same(plan.analyze(new ArrayType([0, 0, 4]), 0), [1, -1, 1, -1], 'preserved Nyquist independent');
        same(plan.analyze(new ArrayType([0, 0, NaN]), 1), [0, 0, 0, 0], 'Legacy ignored Nyquist');
        same(plan.reconstruct(new ArrayType([0, 0, 0, NaN]), 0), [0, 0, 0, 0], 'ignored coefficient suffix'); same(plan.reconstruct(new ArrayType(), 0), [0, 0, 0, 0], 'empty coefficients');
        same(plan.minimum_phase_with(new ArrayType([1, 1, 1]), 0, 0), [0, 0, 0, 0], 'constant causal phase');
        for (const code of [2, 9, 0xffffffff]) for (const action of [() => plan.analyze(zeros, code), () => plan.reconstruct(zeros, code), () => plan.minimum_phase_with(zeros, code, 0), () => plan.minimum_phase_with(zeros, 0, code), () => plan.spectrogram_to_cepstra(zeros, 2, code), () => plan.cepstra_to_spectrogram(new ArrayType(), 0, 0, code)]) { throws(action); exact(plan.length, 4, 'invalid code owner recovery'); }
        for (const action of [() => plan.analyze(new ArrayType(2), 0), () => plan.analyze(new ArrayType([0, 0, NaN]), 0), () => plan.reconstruct(new ArrayType(5), 0), () => plan.spectrogram_to_cepstra(new ArrayType(2), 1, 0), () => plan.spectrogram_to_cepstra(zeros, 5, 0), () => plan.spectrogram_to_cepstra(new ArrayType([-1, -1, -1]), 1, 0), () => plan.cepstra_to_spectrogram(new ArrayType([1]), 2, 1, 0), () => plan.cepstra_to_spectrogram(new ArrayType(), 0xffffffff, 0, 0)]) { throws(action); exact(plan.bins, 3, 'shape/domain owner recovery'); }
        const ignored = new ArrayType([1, 1, NaN]); same(plan.spectrogram_to_cepstra(ignored, 4, 1), plan.spectrogram_to_cepstra(new ArrayType([1, 1, 1]), 4, 1), 'Legacy matrix ignored Nyquist'); throws(() => plan.spectrogram_to_cepstra(ignored, 4, 0));
        for (const policy of [0, 1]) for (const lifter of [0, 1]) same(plan.minimum_phase_with(zeros, policy, lifter), [0, 0, 0, 0], 'all Nyquist/lifter combinations');
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE; throws(() => plan.reconstruct(new ArrayType([largest, largest, largest]), 0));
        for (const invalid of [NaN, Infinity, -Infinity]) {
            const source = new ArrayType([1, 1, 1, 1, invalid, 1]), saved = new Uint8Array(source.buffer).slice();
            throws(() => plan.spectrogram_to_cepstra(source, 2, 0)); assert(new Uint8Array(source.buffer).every((byte, index) => byte === saved[index]), 'late frame input unchanged');
            same(plan.reconstruct(new ArrayType(), 0), [0, 0, 0, 0], 'workspace recovery after late error');
        }
        assert(plan.spectrogram_to_cepstra(new ArrayType(), 2, 0).length === 0 && plan.cepstra_to_spectrogram(new ArrayType(), 0, 2, 0).length === 0, 'empty frame matrices');
        const unit = api.approximation_exponential(0); plan.cepstra_to_spectrogram(new ArrayType(), 2, 0, 0).forEach(value => exact(value, unit, 'zero coefficient frames')); plan.free();
        const singleton = new Plan(1); same(singleton.analyze(new ArrayType([2]), 1), [2], 'singleton analysis'); same(singleton.minimum_phase_with(new ArrayType([2]), 1, 1), [0], 'singleton legacy phase'); singleton.free();
    }
    const logs = new Float64Array(Array.from({length:9}, (_, index) => Math.cos(2 * Math.PI * index / 16))), coefficients = api.real_cepstrum_from_log_spectrum_f64(logs);
    coefficients.forEach((value, index) => assert(Math.abs(value - (index === 1 || index === 15 ? 0.5 : 0)) < 1e-15, 'independent cosine cepstrum'));
    api.real_cepstrum_to_log_spectrum_f64(coefficients, 16).subarray(0, 9).forEach((value, index) => assert(Math.abs(value - logs[index]) < 1e-15, 'analytic roundtrip'));
    api.real_cepstrum_minimum_phase_f64(logs).forEach((value, index) => assert(Math.abs(value + Math.sin(2 * Math.PI * index / 16)) < 1e-15, 'independent causal phase'));
    const plan = new api.RealCepstrumPlanF64(8), logs8 = new Float64Array([0, 1, 0, 0, 0]), cepstrum = plan.analyze(logs8, 0), causal = plan.minimum_phase_with(logs8, 0, 0), legacy = plan.minimum_phase_with(logs8, 0, 1);
    legacy.forEach((value, index) => assert(Math.abs(value - causal[index] - cepstrum[5] * Math.sin(-2 * Math.PI * index * 5 / 8)) < 1e-15, 'Legacy lifter retained negative quefrency')); plan.free();
    return {cases, comparisons, maximumNormalizedCError:maxima, functions:6, owners:2, lifters:2, nyquistPolicies:2};
}
