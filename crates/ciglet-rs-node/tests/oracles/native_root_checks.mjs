export async function verifyRootBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected root error'); };
    let cases = 0, comparisons = 0;
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    for (const width of [4, 8]) {
        const find = api[`root_find_f${width * 8}`], round = width === 4 ? Math.fround : value => value;
        const bytes = await loadFixture(`root-${width === 4 ? 'float' : 'double'}.bin`), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'ROT1' && view.getUint32(4, true) === width, 'root corpus header');
        const count = view.getUint32(8, true); assert(count === 25, 'complete original root count'); let offset = 12;
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        for (let record = 0; record < count; ++record) {
            const kind = view.getUint32(offset, true); offset += 4;
            const start = scalar(), end = scalar(), expected = scalar(); let calls = 0;
            const callback = x => {
                ++calls; equal(x, round(x), 'callback argument selected precision');
                switch (kind) {
                    case 0: return round(round(x * x) - 2);
                    case 1: return round(round(round(round(x * x) * x) - x) - 2);
                    case 2: return round(Math.cos(x) - x);
                    case 3: return round(Math.exp(x) - 3);
                    default: return round(x - round(0.12345));
                }
            };
            equal(find(callback, start, end), expected, 'original C root'); assert(calls > 0, 'callback invoked'); ++cases;
        }
        assert(offset === bytes.byteLength, 'full root corpus consumed');
        let calls = [];
        equal(find(x => { calls.push(x); return x - 1; }, 1, 3), 1, 'start endpoint'); assert(calls.length === 2 && calls[0] === 1 && calls[1] === 3, 'native evaluates both endpoints');
        equal(find(x => x - 3, 1, 3), 3, 'end endpoint'); equal(find(x => x * x + 1, 1, 3), 2, 'nonbracketing fallback');
        const target = round(0.25); equal(find(x => x - target, 0, 1), target, 'captured environment');
        for (const [start, end] of [[3, 1], [NaN, 1], [0, Infinity], [-Infinity, 1]]) {
            calls = []; throws(() => find(x => { calls.push(x); return x; }, start, end)); assert(calls.length === 0, 'invalid bounds before callback');
        }
        for (const returned of [NaN, Infinity, -Infinity, undefined, null, true, '0', {}, 1n]) throws(() => find(() => returned, 0, 1));
        for (const marker of [new Error('root sentinel'), {sentinel:1}, 'thrown string', undefined, null]) {
            let observed = false;
            try { find(() => { throw marker; }, 0, 1); } catch (error) { observed = true; assert(error === marker, 'original callback exception identity'); }
            assert(observed, 'throw surfaced'); equal(find(x => x - target, 0, 1), target, 'recovery after callback throw');
        }
        let evaluations = 0; throws(() => find(x => { ++evaluations; return evaluations === 2 ? NaN : x - 0.25; }, 0, 1)); assert(evaluations === 2, 'late endpoint failure stops');
        throws(() => find(x => x === 0 ? NaN : x, -1, 1));
        const largest = width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE;
        equal(find(() => 1, largest, largest), largest, 'overflow-safe midpoint');
        equal(find(() => -0, -0, 1), -0, 'endpoint signed zero');
        if (width === 4) throws(() => find(() => Number.MAX_VALUE, 0, 1));
        else equal(find(x => (x - target) * 1e-200, 0, 1), 0, 'native absolute residual tolerance');
        const nested = find(x => x - find(y => y - target, 0, 1), 0, 1); equal(nested, target, 'independent nested callback search');
    }
    return {cases, comparisons, functions:2};
}
