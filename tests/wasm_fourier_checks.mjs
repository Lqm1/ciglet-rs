// Shared actual browser/Node checks; original C fixtures and bounds are unchanged.
export async function verifyFourierBindings(api, loadFixture) {
    function assert(condition, message) { if (!condition) throw new Error(message); }
    function throws(action) { let caught = false; try { action(); } catch { caught = true; } assert(caught, 'expected Fourier validation error'); }
    let recordsChecked = 0, comparisons = 0;
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array;
        const Plan = api[width === 4 ? 'FftPlanF32' : 'FftPlanF64'];
        const Result = api[width === 4 ? 'ComplexValuesF32' : 'ComplexValuesF64'];
        const fft = api[`fft_f${width * 8}`], idft = api[`inverse_dft_f${width * 8}`], shift = api[`fft_shift_f${width * 8}`];
        const tolerance = width === 4 ? 2e-7 : 1e-15;
        const plans = new Map();
        for (const direct of [false, true]) {
            const bytes = await loadFixture(`c-${direct ? 'idft' : 'fft'}-f${width * 8}.bin`);
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
            let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === (direct ? 'IDF1' : 'FFT1'), 'Fourier fixture tag');
            assert(integer() === width, 'Fourier fixture precision');
            const records = integer(); assert(records === (direct ? 120 : 240), 'Fourier fixture count');
            for (let record = 0; record < records; ++record) {
                const length = integer(), scenario = integer(), mask = integer(), direction = integer();
                const real = new ArrayType(length), imaginary = new ArrayType(length);
                for (let index = 0; index < length; ++index) {
                    if (scenario === 0) {
                        real[index] = index === Math.floor(length / 3) ? 0.75 : 0;
                        imaginary[index] = index === Math.floor(length / 4) ? -0.25 : 0;
                    } else if (scenario === 1) {
                        real[index] = (index % 17 - 8) * 0.125;
                        imaginary[index] = (index % 11 - 5) * 0.0625;
                    } else {
                        assert(scenario === 2, 'Fourier fixture scenario');
                        real[index] = 2048 + (index % 7) * 0.125;
                        imaginary[index] = -1024 + (index % 5) * 0.0625;
                    }
                }
                const realInput = mask & 1 ? real : null, imaginaryInput = mask & 2 ? imaginary : undefined;
                const realBefore = real.slice(), imaginaryBefore = imaginary.slice();
                const results = [direct ? idft(length, realInput, imaginaryInput) : fft(length, realInput, imaginaryInput, direction)];
                if (!direct) {
                    if (!plans.has(length)) plans.set(length, new Plan(length));
                    const plan = plans.get(length);
                    assert(plan.length === length && !plan.is_empty(), 'reusable plan fields');
                    results.push(plan.transform(realInput, imaginaryInput, direction));
                    const interleaved = new ArrayType(length * 2);
                    for (let index = 0; index < length; ++index) {
                        interleaved[index * 2] = realInput ? real[index] : 0;
                        interleaved[index * 2 + 1] = imaginaryInput ? imaginary[index] : 0;
                    }
                    const saved = interleaved.slice(); results.push(plan.process(interleaved, direction));
                    assert(interleaved.every((value, index) => Object.is(value, saved[index])), 'process input retained');
                }
                assert(real.every((value, index) => Object.is(value, realBefore[index])) && imaginary.every((value, index) => Object.is(value, imaginaryBefore[index])), 'plane inputs retained');
                let inputScale = 1;
                for (const plane of [realInput, imaginaryInput]) if (plane) for (const value of plane) inputScale = Math.max(inputScale, Math.abs(value));
                const scale = inputScale * (direction === 0 ? length : 1);
                const outputs = results.map(result => {
                    assert(result instanceof Result && result.length === length, 'complex result fields');
                    const real = result.real, imaginary = result.imaginary, interleaved = result.interleaved;
                    assert(real instanceof ArrayType && imaginary instanceof ArrayType && interleaved instanceof ArrayType, 'complex typed arrays');
                    assert(real.length === length && imaginary.length === length && interleaved.length === length * 2, 'complete complex dimensions');
                    result.free();
                    return { real, imaginary, interleaved };
                });
                for (let index = 0; index < length; ++index) {
                    for (const [component, slot] of [['real', 0], ['imaginary', 1]]) {
                        const expected = scalar();
                        for (const output of outputs) {
                            const actual = output[component][index];
                            assert(Number.isFinite(actual) && Math.abs(actual - expected) / scale <= tolerance, 'original normalized C Fourier bound');
                            assert(Object.is(actual, output.interleaved[index * 2 + slot]), 'interleaved field retains bits');
                            ++comparisons;
                        }
                    }
                }
                ++recordsChecked;
            }
            assert(offset === bytes.byteLength, 'Fourier fixture trailing bytes');
        }
        for (const plan of plans.values()) plan.free();
        for (const invalid of [0, 3, 0xffffffff]) throws(() => new Plan(invalid));
        const plan = new Plan(4), real = new ArrayType([1, 2, 3, 4]);
        for (const direction of [2, 9, 0xffffffff]) {
            throws(() => plan.transform(real, null, direction));
            assert(plan.length === 4 && !plan.is_empty(), 'transform direction error releases owner borrow');
            throws(() => plan.process(new ArrayType(8), direction));
            assert(plan.length === 4 && !plan.is_empty(), 'process direction error releases owner borrow');
            const recovered = plan.transform(real, null, api.FftDirection.Forward);
            assert(recovered.real[0] === 10, 'same plan reusable after invalid direction'); recovered.free();
            throws(() => fft(4, real, null, direction));
        }
        throws(() => plan.transform(new ArrayType(3), null, 0));
        throws(() => plan.transform(null, new ArrayType(5), 1));
        throws(() => plan.process(new ArrayType(7), 0));
        throws(() => plan.process(new ArrayType(9), 1));
        const forward = plan.transform(real, null, 0);
        const inverse = plan.transform(forward.real, forward.imaginary, 1);
        plan.free(); forward.free();
        const retained = inverse.real; assert(retained.every((value, index) => value === real[index]), 'normalized roundtrip after plan release');
        inverse.free(); assert(retained[3] === 4, 'numeric snapshots outlive result');
        for (const length of [0, 0xffffffff]) throws(() => idft(length, null, null));
        throws(() => idft(3, new ArrayType(2), null));
        throws(() => idft(3, null, new ArrayType(4)));
        const zero = idft(3, undefined, null); assert([...zero.real, ...zero.imaginary].every(value => value === 0), 'missing planes represent zeros'); zero.free();
        for (const length of [0, 1, 2, 3, 5, 8]) {
            const input = ArrayType.from({ length }, (_, index) => index + 1), original = input.slice();
            const expected = [...input.slice(length - Math.floor(length / 2)), ...input.slice(0, length - Math.floor(length / 2))];
            const output = shift(input); assert(output.length === length && output.every((value, index) => value === expected[index]), 'original odd/even FFT shift');
            assert(input.every((value, index) => value === original[index]), 'shift input retained');
        }
        const constructed = new Result(new ArrayType([-0, Infinity, NaN]), new ArrayType([1, -Infinity, 0]));
        const first = constructed.real, second = constructed.real, imaginary = constructed.imaginary;
        assert(constructed.length === 3 && Object.is(first[0], -0) && first[1] === Infinity && Number.isNaN(first[2]) && imaginary[1] === -Infinity, 'arbitrary complex fields retained');
        first[0] = 73; assert(Object.is(second[0], -0), 'independent numeric getters');
        constructed.free(); assert(second[1] === Infinity, 'constructor snapshot lifetime');
        throws(() => new Result(new ArrayType(1), new ArrayType(2)));
        const empty = new Result(new ArrayType(0), new ArrayType(0)); assert(empty.length === 0 && empty.interleaved.length === 0, 'empty arbitrary result'); empty.free();
        const BitArray = width === 4 ? Uint32Array : BigUint64Array;
        const patterns = width === 4
            ? [0x80000000, 0x7fc12345, 0x7f812345, 0x7f800000, 0xff800000, 1, 0x7f7fffff]
            : [0x8000000000000000n, 0x7ff8123456789abcn, 0x7ff0123456789abcn, 0x7ff0000000000000n, 0xfff0000000000000n, 1n, 0x7fefffffffffffffn];
        const rawReal = new BitArray(patterns), rawImaginary = new BitArray([...patterns].reverse());
        const exact = new Result(new ArrayType(rawReal.buffer), new ArrayType(rawImaginary.buffer));
        const exactReal = new BitArray(exact.real.buffer), exactImaginary = new BitArray(exact.imaginary.buffer);
        const exactInterleaved = new BitArray(exact.interleaved.buffer);
        exact.free();
        assert(exactReal.every((value, index) => value === rawReal[index]) && exactImaginary.every((value, index) => value === rawImaginary[index]), 'arbitrary complex scalar bits');
        assert(exactInterleaved.every((value, index) => value === (index % 2 ? rawImaginary[Math.floor(index / 2)] : rawReal[Math.floor(index / 2)])), 'arbitrary interleaved scalar bits');
        const shiftedBits = new BitArray(shift(new ArrayType(rawReal.buffer)).buffer);
        const rotation = Math.floor(patterns.length / 2);
        assert(shiftedBits.every((value, index) => value === rawReal[(index + patterns.length - rotation) % patterns.length]), 'FFT shift retains nonfinite scalar bits');
    }
    assert(api.FftDirection.Forward === 0 && api.FftDirection.Inverse === 1, 'direction codes');
    return { records: recordsChecked, comparisons, functions: 6, classes: 4 };
}
