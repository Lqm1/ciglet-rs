export async function verifyResamplingBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected resampling error'); }
    let records = 0, comparisons = 0;
    function equal(actual, expected, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            assert(Object.is(actual[index], expected[index]), `${label} bits at ${index}`); ++comparisons;
        }
    }
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array, suffix = `f${width * 8}`;
        const resample = api[`resample_${suffix}`], withBoundary = api[`resample_with_${suffix}`];
        const withKernel = api[`resample_with_kernel_${suffix}`], outputLength = api[`resample_output_length_${suffix}`];
        const bytes = await loadFixture(`c-filter-resample-${suffix}.bin`);
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        let offset = 4;
        const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const plane = count => ArrayType.from({ length: count }, scalar);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'FRS1' && integer() === width, 'resampling corpus header');
        assert(integer() === 300 && integer() === 60 && integer() === 105, 'resampling corpus counts');
        // FIR and convolution have separate coverage; only parse their records here.
        for (let record = 0; record < 300; ++record) {
            const count = integer(); integer(); integer(); offset += (count + 2) * width;
        }
        for (let record = 0; record < 60; ++record) {
            const signalCount = integer(), kernelCount = integer();
            offset += (signalCount + kernelCount + signalCount + kernelCount - 1) * width;
        }
        for (let record = 0; record < 105; ++record) {
            const count = integer(), ratio = scalar(), signal = plane(count), length = integer(), expected = plane(length);
            const saved = signal.slice(); assert(outputLength(count, ratio) === length, 'source output sizing');
            const output = withKernel(signal, ratio, api.BoundaryPolicy.LegacySkipFirst, api.KernelPolicy.Legacy);
            assert(output instanceof ArrayType && output.length === length, 'resampling typed output shape');
            equal(output, withBoundary(signal, ratio, api.BoundaryPolicy.LegacySkipFirst), 'legacy convenience');
            for (let index = 0; index < length; ++index) {
                const error = Math.abs(output[index] - expected[index]) / Math.max(1, Math.abs(expected[index]));
                assert(error < (width === 4 ? 2e-7 : 2e-15), `original C resampling ${suffix}/${record}/${index}: ${error}`); ++comparisons;
            }
            equal(signal, saved, 'source unchanged'); ++records;
        }
        assert(offset === bytes.byteLength, 'resampling corpus trailing bytes');
        for (const count of [2, 5, 32, 65]) {
            const signal = ArrayType.from({ length: count }, (_, index) => (index % 7) * 0.125 - 0.375);
            for (const rawRatio of [0.9, 1.1, 1.5, 2]) {
                const ratio = width === 4 ? Math.fround(rawRatio) : rawRatio;
                for (const boundary of [api.BoundaryPolicy.IncludeFirst, api.BoundaryPolicy.LegacySkipFirst]) {
                    for (const kernel of [api.KernelPolicy.Stable, api.KernelPolicy.Legacy]) {
                        const actual = withKernel(signal, ratio, boundary, kernel);
                        assert(actual.length === outputLength(count, ratio), 'all policy output sizes');
                        if (kernel === api.KernelPolicy.Stable) assert(actual.every(value => Number.isFinite(value) && Math.abs(value) < 1), 'corrected fractional rates bounded');
                        if (boundary === 0 && kernel === 0) equal(actual, resample(signal, ratio), 'corrected default');
                        if (kernel === 1) equal(actual, withBoundary(signal, ratio, boundary), 'boundary convenience');
                    }
                }
            }
        }
        const signal = new ArrayType([1, -0, 0.5]);
        for (const boundary of [0, 1]) for (const kernel of [0, 1]) equal(withKernel(signal, 1, boundary, kernel), signal, 'ratio-one exact identity');
        equal(resample(signal, 1), signal, 'default identity');
        equal(withBoundary(new ArrayType([0.75]), 2, 1), new ArrayType([0, 0]), 'source sample-zero omission');
        const corrected = resample(new ArrayType([0.75]), 2);
        assert(corrected.length === 2 && corrected[0] === 0.75 && corrected[1] > 0.4, 'corrected sample-zero inclusion');
        // A nearly integer upsample location exercises the corrected kernel limit.
        const nearRatio = width === 4 ? Math.fround(1 + 2 ** -23) : 1 + 2 ** -50;
        const near = resample(new ArrayType([0, 0.75, 0]), nearRatio);
        assert(Math.abs(near[1] - 0.75) <= (width === 4 ? 1e-7 : 3e-16), 'independent near-integer impulse limit');
        const legacyNear = withKernel(new ArrayType([0, 0.75, 0]), nearRatio, 0, 1);
        assert(Math.abs(legacyNear[1] - near[1]) > (width === 4 ? 0.1 : 1e16), 'legacy near-integer arithmetic retained');
        assert(outputLength(0x7fffffff, 1) === 0x7fffffff, 'ratio-one count does not round through f32');
        assert(outputLength(1, 0.5) === 1 && outputLength(1, 0.25) === 0, 'native half-away positive rounding');
        assert(resample(new ArrayType(0), 0.5).length === 0 && resample(new ArrayType([1]), 0.25).length === 0, 'empty output');
        for (const invalid of [0, -1, NaN, Infinity, -Infinity]) {
            throws(() => resample(signal, invalid)); throws(() => outputLength(0, invalid));
        }
        for (const invalid of [NaN, Infinity, -Infinity]) throws(() => resample(new ArrayType([invalid]), 1));
        throws(() => outputLength(0x80000000, 1)); throws(() => outputLength(0x7fffffff, 2));
        throws(() => resample(signal, width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE));
        throws(() => withBoundary(signal, 1, 9)); throws(() => withKernel(signal, 1, 0, 9));
        const first = resample(signal, 1), second = resample(signal, 1);
        first[0] = 73; equal(second, signal, 'independent results and post-error use'); equal(signal, new ArrayType([1, -0, 0.5]), 'input still unchanged');
    }
    return { records, comparisons, functions: 8 };
}
