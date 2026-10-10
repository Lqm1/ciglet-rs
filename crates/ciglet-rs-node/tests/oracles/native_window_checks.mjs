// Shared actual Node.js/browser checks against unchanged upstream C fixtures.
export async function verifyWindowBindings(api, loadFixture) {
    let comparisons = 0;
    function assert(condition, message) {
        if (!condition) throw new Error(message);
    }
    function reader(bytes, width) {
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        let offset = 0;
        return {
            tag() {
                const value = String.fromCharCode(...bytes.subarray(offset, offset + 4));
                offset += 4;
                return value;
            },
            integer() { const value = view.getUint32(offset, true); offset += 4; return value; },
            scalar() {
                const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true);
                offset += width;
                return value;
            },
            finished() { return offset === bytes.byteLength; },
        };
    }
    function equalBits(actual, expected, width) {
        const storage = new ArrayBuffer(width * 2);
        const view = new DataView(storage);
        if (width === 4) {
            view.setFloat32(0, actual, true); view.setFloat32(4, expected, true);
            assert(view.getUint32(0, true) === view.getUint32(4, true), 'binary32 callback weight mismatch');
        } else {
            view.setFloat64(0, actual, true); view.setFloat64(8, expected, true);
            assert(view.getBigUint64(0, true) === view.getBigUint64(8, true), 'binary64 callback weight mismatch');
        }
    }
    for (const width of [4, 8]) {
        const builtin = api[width === 4 ? 'window_f32' : 'window_f64'];
        const arrayType = width === 4 ? Float32Array : Float64Array;
        const corpus = reader(await loadFixture(`c-window-f${width * 8}.bin`), width);
        assert(corpus.tag() === 'WIN1' && corpus.integer() === width && corpus.integer() === 112, 'window corpus header');
        for (let record = 0; record < 112; ++record) {
            const kind = corpus.integer(), precision = corpus.integer(), length = corpus.integer();
            const values = builtin(kind, length, precision);
            assert(values instanceof arrayType && values.length === length, 'window typed-array shape');
            for (const actual of values) {
                const expected = corpus.scalar();
                assert(Math.abs(actual - expected) <= (width === 4 ? 1e-7 : 1e-15), 'original window numerical bound');
                ++comparisons;
            }
        }
        assert(corpus.finished(), 'window corpus trailing bytes');
        const custom = reader(await loadFixture(`custom_window-${width === 4 ? 'float' : 'double'}.bin`), width);
        assert(custom.tag() === 'CUS1' && custom.integer() === width && custom.integer() === 60, 'custom corpus header');
        for (let record = 0; record < 60; ++record) {
            const kind = [1, 2, 6, 3, 4, 5][custom.integer()], callbackWidth = custom.integer();
            const length = custom.integer(), expectedCalls = custom.integer();
            let calls = 0;
            const callback = angle => {
                assert(typeof angle === 'number' && Number.isFinite(angle), 'callback radian argument');
                ++calls;
                const result = 0.75 - angle * 0.0625 + angle * angle * 0.00390625;
                return callbackWidth === 4 ? Math.fround(result) : result;
            };
            const values = api[`window_custom_f${width * 8}_f${callbackWidth * 8}`](kind, length, callback);
            assert(values instanceof arrayType && values.length === length && calls === expectedCalls, 'custom shape/call count');
            for (const value of values) { equalBits(value, custom.scalar(), width); ++comparisons; }
        }
        assert(custom.tag() === 'RED1', 'unrelated reduction corpus boundary');
    }
    const customNames = ['window_custom_f32_f32', 'window_custom_f32_f64', 'window_custom_f64_f32', 'window_custom_f64_f64'];
    for (const name of customNames) {
        const custom = api[name];
        const forbidden = () => { throw new Error('degenerate windows must not call callback'); };
        for (let kind = 0; kind < 7; ++kind) {
            assert(custom(kind, 0, forbidden).length === 0, 'empty custom window');
            assert(custom(kind, 1, forbidden)[0] === 1, 'singleton custom window');
        }
        assert([...custom(0, 8, forbidden)].every(value => value === 1), 'boxcar callback bypass');
        for (const thrown of [{ name: 'original callback exception' }, undefined, null, 73]) {
            let calls = 0, caught = false;
            try { custom(1, 8, () => { if (++calls === 3) throw thrown; return 0.5; }); }
            catch (error) { caught = true; assert(error === thrown, 'original exception identity'); }
            assert(caught && calls === 3, 'callback stop on exception');
        }
        for (const invalid of [NaN, Infinity, -Infinity, '0.5', null, undefined, {}]) {
            let caught = false, calls = 0;
            try { custom(1, 8, () => { ++calls; return invalid; }); }
            catch { caught = true; }
            assert(caught && calls === 1, 'invalid callback fails on first draw');
        }
        assert(custom(1, 8, () => 0.5).length === 8, 'instance remains usable after callback errors');
    }
    for (const name of ['window_f32', 'window_f64']) {
        const builtin = api[name];
        for (let kind = 0; kind < 7; ++kind) {
            for (const precision of [0, 1]) {
                assert(builtin(kind, 0, precision).length === 0, 'empty builtin window');
                assert(builtin(kind, 1, precision)[0] === 1, 'singleton builtin window');
            }
        }
        let caught = false;
        try { builtin(0, 0xffffffff, 0); } catch { caught = true; }
        assert(caught, 'legacy range validation before allocation');
        const first = builtin(1, 8, 0), second = builtin(1, 8, 0);
        const retained = second[3]; first[3] = 73;
        assert(second[3] === retained && builtin(1, 8, 0)[3] === retained, 'independent owned output arrays');
    }
    assert(api.WindowKind.MltSine === 6 && api.WindowPrecision.Approximate === 1, 'complete enum codes');
    return { comparisons, builtinCases: 224, customCases: 120, exports: 6 };
}
