export async function verifyFilterBankBindings(api, loadFixture) {
    function assert(value, label) { if (!value) throw new Error(label); }
    function throws(action) { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected filterbank error'); }
    let generatedCases = 0, customCases = 0, comparisons = 0;
    function close(actual, expected, tolerance, label) {
        assert(actual.length === expected.length, label + ' shape');
        for (let index = 0; index < actual.length; ++index) {
            const a = actual[index], e = expected[index];
            assert(Object.is(a, e) || Number.isNaN(a) && Number.isNaN(e) || Number.isFinite(a) && Number.isFinite(e) && Math.abs(a - e) <= tolerance * (1 + Math.abs(e)), `${label}/${index}: ${a} versus ${e}`);
            ++comparisons;
        }
    }
    assert(api.FilterBankCompression.Logarithmic === 0 && api.FilterBankCompression.Plp === 1, 'complete compression enum');
    for (const width of [4, 8]) {
        const ArrayType = width === 4 ? Float32Array : Float64Array;
        const Bank = api[`FilterBankF${width * 8}`], Options = api[`MelOptionsF${width * 8}`];
        const round = value => width === 4 ? Math.fround(value) : value;
        function reader(bytes, magic, count) {
            const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength); let offset = 4;
            const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
            const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
            assert(String.fromCharCode(...bytes.subarray(0, 4)) === magic && integer() === width && integer() === count, 'filterbank corpus header');
            return { integer, scalar, plane: length => ArrayType.from({ length }, scalar), done: () => assert(offset === bytes.byteLength, 'filterbank corpus fully consumed') };
        }
        for (const custom of [false, true]) {
            const count = custom ? 33 : 72;
            const filename = custom ? `custom_filterbank-${width === 4 ? 'float' : 'double'}.bin` : `c-filterbank-f${width * 8}.bin`;
            const r = reader(await loadFixture(filename), custom ? 'CFB1' : 'FBK1', count);
            for (let record = 0; record < count; ++record) {
                const kind = r.integer(), bins = r.integer(), channels = r.integer(), nyquist = r.scalar();
                let bank;
                if (custom) bank = kind < 5 ? new Bank(bins, nyquist, channels) : Bank.plp(bins, nyquist, channels);
                else if (kind === 0) bank = new Bank(bins, nyquist, channels);
                else if (kind === 1) {
                    const options = new Options(50, nyquist, 0.85, 120);
                    bank = Bank.mel(bins, nyquist, channels, options); options.free();
                } else if (kind === 2) bank = Bank.mel_default(bins, nyquist, channels, 50, nyquist);
                else bank = Bank.plp(bins, nyquist, channels);
                assert(bank.bins === bins && bank.channels === channels, 'complete dimensions');
                close([bank.nyquist_hz], [nyquist], 0, 'metadata');
                for (let channel = 0; channel < channels; ++channel) {
                    const start = r.integer(), end = r.integer(), response = r.plane(bins);
                    if (custom && kind < 5) bank.set_channel(channel, response, start, end);
                    close(bank.support(channel), [start, end], 0, 'half-open support');
                    close(bank.response(channel), response, custom ? 0 : width === 4 ? 2e-7 : 2e-14, 'original C response');
                }
                assert(bank.response(channels) === undefined && bank.support(channels) === undefined, 'missing channels');
                const spectra = custom ? r.plane(3 * bins) : ArrayType.from({ length: 3 * bins }, (_, index) => {
                    const scenario = Math.floor(index / bins), bin = index % bins;
                    return scenario === 0 ? 0 : scenario === 1 ? (bin % 17 + 1) * 0.125 : bin === Math.floor(bins / 3) ? 2 : 0;
                });
                const saved = spectra.slice(), clone = bank.cloned();
                for (const compression of [0, 1]) {
                    const tolerance = custom ? width === 4 ? 2e-7 : 2e-15 : width === 4 ? 2e-6 : 2e-13;
                    const expected = new ArrayType(3 * channels);
                    for (let frame = 0; frame < 3; ++frame) {
                        const values = r.plane(channels); expected.set(values, frame * channels);
                        close(bank.apply(spectra.subarray(frame * bins, (frame + 1) * bins), compression), values, tolerance, 'original C spectrum');
                    }
                    close(r.plane(3 * channels), expected, 0, 'repeated C spectrogram');
                    const output = bank.apply_frames(spectra, 3, compression);
                    assert(output instanceof ArrayType, 'owned output precision');
                    close(output, expected, tolerance, 'original C explicit frames');
                    if (bins) close(bank.apply_spectrogram(spectra, compression), output, 0, 'inferred frames');
                    else throws(() => bank.apply_spectrogram(spectra, compression));
                    clone.set_nyquist_hz(-Infinity);
                    assert(clone.nyquist_hz === -Infinity, 'nonfinite inert metadata');
                    close(clone.apply_frames(spectra, 3, compression), output, 0, 'independent clone calculation');
                }
                close(spectra, saved, 0, 'source unchanged');
                const retained = channels ? bank.response(0) : new ArrayType(0), retainedSupport = channels ? bank.support(0) : new Uint32Array(0);
                bank.free();
                if (channels) {
                    close(clone.response(0), retained, 0, 'response outlives original');
                    close(clone.support(0), retainedSupport, 0, 'support outlives original');
                }
                clone.free(); assert(retained.length === bins || channels === 0, 'getter storage outlives owners');
                if (custom) ++customCases; else ++generatedCases;
            }
            r.done();
        }
        const options = new Options(50, 8000, 1, 400);
        close([options.minimum_hz, options.maximum_hz, options.scale, options.minimum_width_hz], [50, 8000, 1, 400], 0, 'complete options');
        const defaults = Bank.mel_default(16, 8000, 4, 50, 8000), explicit = Bank.mel(16, 8000, 4, options);
        for (let channel = 0; channel < 4; ++channel) {
            close(defaults.response(channel), explicit.response(channel), 0, 'default Mel options');
            close(defaults.support(channel), explicit.support(channel), 0, 'default Mel support');
        }
        options.minimum_hz = 100; options.maximum_hz = 6000; options.scale = 0.85; options.minimum_width_hz = 120;
        close([options.minimum_hz, options.maximum_hz, options.scale, options.minimum_width_hz], [100, 6000, round(0.85), 120], 0, 'all mutable options');
        const changed = Bank.mel(16, 8000, 4, options); options.scale = 0;
        throws(() => Bank.mel(16, 8000, 4, options)); options.scale = 1;
        options.minimum_width_hz = -1; throws(() => Bank.mel(16, 8000, 4, options)); options.minimum_width_hz = 0;
        options.minimum_hz = NaN; throws(() => Bank.mel(16, 8000, 4, options)); options.minimum_hz = 0;
        options.maximum_hz = -1; throws(() => Bank.mel(16, 8000, 4, options));
        options.free(); changed.free(); defaults.free(); explicit.free();

        const bank = new Bank(4, NaN, 1), source = new ArrayType([NaN, 0.5, 1, Infinity]);
        bank.set_channel(0, source, 1, 3); source[1] = 99;
        close(bank.response(0), [NaN, 0.5, 1, Infinity], 0, 'set channel copies inactive values');
        const input = new ArrayType([NaN, 2, 3, Infinity]);
        close(bank.apply(input, 1), [round(Math.pow(4, 0.33))], width === 4 ? 2e-7 : 2e-15, 'independent active-support PLP');
        const clone = bank.cloned(), before = bank.response(0), support = bank.support(0);
        for (const action of [() => bank.set_channel(0, new ArrayType([0, NaN, 1, 0]), 1, 3),
            () => bank.set_channel(1, source, 1, 3), () => bank.set_channel(0, new ArrayType(3), 1, 3),
            () => bank.set_channel(0, source, 3, 1), () => bank.set_channel(0, source, 0, 5)]) {
            throws(action); close(bank.response(0), before, 0, 'failed mutation atomic response'); close(bank.support(0), support, 0, 'failed mutation atomic support');
        }
        bank.set_channel(0, new ArrayType([-9, 1, -0.5, 9]), 1, 3);
        close(bank.apply(input, 1), [round(Math.pow(0.5, 0.33))], width === 4 ? 2e-7 : 2e-15, 'signed weights');
        close(clone.response(0), before, 0, 'clone independent of mutation');
        const snapshot = bank.response(0); snapshot[1] = 77;
        assert(bank.response(0)[1] === 1, 'getter independent storage');
        throws(() => bank.apply(new ArrayType(3), 1)); throws(() => bank.apply(new ArrayType([0, NaN, 3, 0]), 1));
        throws(() => bank.apply(input, 9)); throws(() => bank.apply_spectrogram(new ArrayType(5), 1));
        throws(() => bank.apply_frames(input, 2, 1));
        throws(() => bank.apply_spectrogram(input, 9)); throws(() => bank.apply_frames(input, 1, 9));
        assert(bank.bins === 4 && bank.response(0)[1] === 1, 'owner reusable after all compression errors');
        bank.set_channel(0, new ArrayType(4), 0, 0);
        close(bank.apply(new ArrayType([NaN, NaN, NaN, NaN]), 1), [0], 0, 'empty support ignores all source values');
        const survivor = clone.apply(input, 1); bank.free(); clone.free();
        close(survivor, [round(Math.pow(4, 0.33))], width === 4 ? 2e-7 : 2e-15, 'result outlives owner');

        const zeroBins = new Bank(0, Infinity, 2), noChannels = new Bank(4, -Infinity, 0);
        close(zeroBins.apply_frames(new ArrayType(0), 3, 1), new ArrayType(6), 0, 'explicit zero-bin frames');
        const floor = zeroBins.apply(new ArrayType(0), 0);
        assert(floor.length === 2 && Number.isFinite(floor[0]) && floor[0] < 0, 'zero-bin log floor');
        close(zeroBins.apply_frames(new ArrayType(0), 3, 0), ArrayType.from({ length: 6 }, () => floor[0]), 0, 'zero-bin log frames');
        throws(() => zeroBins.apply_spectrogram(new ArrayType(0), 0));
        close(noChannels.apply_frames(new ArrayType([NaN, NaN, NaN, NaN]), 1, 1), [], 0, 'zero channels ignore source');
        zeroBins.free(); noChannels.free();
        const BitArray = width === 4 ? Uint32Array : BigUint64Array;
        const patterns = width === 4 ? [0x80000000, 0x7fc12345, 0x7f812345, 0x7f800000, 0xff800000, 1]
            : [0x8000000000000000n, 0x7ff8123456789abcn, 0x7ff0123456789abcn, 0x7ff0000000000000n, 0xfff0000000000000n, 1n];
        const bits = new BitArray(patterns), arbitrary = new ArrayType(bits.buffer), payloadBank = new Bank(patterns.length, -0, 1);
        payloadBank.set_channel(0, arbitrary, 0, 0);
        const payloadClone = payloadBank.cloned(), retainedBits = payloadBank.response(0);
        assert(Object.is(payloadBank.nyquist_hz, -0), 'signed-zero metadata');
        close(Array.from(new BitArray(retainedBits.buffer)), patterns, 0, 'inactive complete scalar payload bits');
        close(Array.from(new BitArray(payloadClone.response(0).buffer)), patterns, 0, 'clone complete payload bits');
        close(Array.from(bits), patterns, 0, 'set channel source payload bits unchanged');
        payloadBank.free(); payloadClone.free();
        close(Array.from(new BitArray(retainedBits.buffer)), patterns, 0, 'payload storage outlives both owners');
        const overflowFrames = new Bank(0, 0, 2);
        throws(() => overflowFrames.apply_frames(new ArrayType(0), 0xffffffff, 1));
        close(overflowFrames.apply_frames(new ArrayType(0), 1, 1), [0, 0], 0, 'frame count overflow recovery'); overflowFrames.free();
        const emptyPlp = Bank.plp(0, NaN, 2); assert(Number.isNaN(emptyPlp.nyquist_hz), 'empty PLP metadata'); emptyPlp.free();
        throws(() => new Bank(0xffffffff, 0, 0)); throws(() => new Bank(0, 0, 0xffffffff));
        throws(() => Bank.plp(4, NaN, 1)); throws(() => Bank.mel_default(0, 8000, 1, 0, 8000));
    }
    assert(generatedCases === 144 && customCases === 66, 'complete C fixture coverage');
    return { generatedCases, customCases, comparisons, bankOwners: 2, optionOwners: 2, compressionVariants: 2 };
}
