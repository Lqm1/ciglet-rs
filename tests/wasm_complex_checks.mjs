export async function verifyComplexBindings(api, loadFixture) {
    const assert = (condition, label) => { if (!condition) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected complex error'); };
    let cases = 0, comparisons = 0;
    const hostDifferences = [];
    for (const width of [4, 8]) {
        const Owner = api[`ComplexF${width * 8}`], ArrayType = width === 4 ? Float32Array : Float64Array;
        const bytes = await loadFixture(`complex-${width === 4 ? 'float' : 'double'}.bin`);
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'CPX1' && view.getUint32(4, true) === width && view.getUint32(8, true) === 64, 'full complex corpus header');
        let offset = 12;
        const scalar = () => { const value = width === 4 ? view.getFloat32(offset, true) : view.getFloat64(offset, true); offset += width; return value; };
        const point = () => [scalar(), scalar()];
        const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
        const host = (actual, expected, label) => {
            if (width === 4 || Object.is(actual, expected) || actual === 0 || expected === 0) return equal(actual, expected, label);
            const storage = new DataView(new ArrayBuffer(16)); storage.setFloat64(0, actual); storage.setFloat64(8, expected);
            const first = storage.getBigUint64(0), second = storage.getBigUint64(8);
            const ulp = first > second ? first - second : second - first;
            assert(ulp <= 1n && Math.abs(actual - expected) / (1 + Math.abs(expected)) <= Number.EPSILON, 'existing WASM binary64 atan2 bound');
            hostDifferences.push({ label, actual, expected, ulp: String(ulp), normalized: Math.abs(actual - expected) / (1 + Math.abs(expected)) });
            ++comparisons;
        };
        for (let record = 0; record < 64; ++record) {
            const first = new Owner(...point()), second = new Owner(...point());
            const results = [first.conjugate(), first.add(second), first.subtract(second), first.multiply(second), first.divide(second), first.exponential(0), first.exponential(1), first.exponential(2)];
            for (let field = 0; field < results.length; ++field) {
                const expected = point(), actual = results[field].components();
                for (let component = 0; component < 2; ++component) equal(actual[component], expected[component], `C width${width} record${record} field${field} component${component}`);
                results[field].free();
            }
            equal(first.argument(0), scalar(), 'C Fast argument');
            const angle = scalar();
            host(first.argument(1), angle, 'C Approximate argument');
            host(first.argument(2), angle, 'C Exact argument');
            first.free(); second.free(); ++cases;
        }
        assert(view.getUint32(offset, true) === 15, 'scalar tail count'); offset += 4;
        // The separately verified scalar approximation tail has ten values per record.
        offset += 15 * 10 * width;
        assert(offset === bytes.byteLength, 'complete CPX1 structure consumed');
        const source = new Owner(3, 4), copy = source.cloned(), snapshot = source.components();
        source.real = 5; source.imaginary = -6;
        equal(source.real, 5, 'real setter'); equal(source.imaginary, -6, 'imaginary setter');
        equal(copy.real, 3, 'clone independent'); equal(snapshot[1], 4, 'snapshot independent');
        const self = source.add(source); equal(self.real, 10, 'same owner operands'); equal(self.imaginary, -12, 'same owner imaginary'); self.free();
        for (const length of [0, 1, 3, 4]) {
            throws(() => Owner.from_components(new ArrayType(length)));
            throws(() => source.set_components(new ArrayType(length)));
            equal(source.real, 5, 'invalid replacement atomic');
        }
        for (const mode of [3, 9, 0xffffffff]) {
            throws(() => source.exponential(mode)); throws(() => source.argument(mode));
            equal(source.real, 5, 'invalid mode releases owner borrow');
            const recovered = source.exponential(2); recovered.free();
        }
        const zero = new Owner(0, 0); throws(() => source.divide(zero)); zero.free();
        for (const value of [NaN, Infinity, -Infinity]) {
            const invalid = new Owner(value, 1);
            throws(() => invalid.argument(0)); throws(() => invalid.exponential(2)); throws(() => source.divide(invalid)); throws(() => invalid.divide(source));
            const unchecked = invalid.add(source); assert(!Number.isFinite(unchecked.real), 'ordinary IEEE addition'); unchecked.free(); invalid.free();
        }
        const overflow = new Owner(1000, 0); throws(() => overflow.exponential(2)); overflow.free();
        for (const magnitude of [width === 4 ? 3.4028234663852886e38 : Number.MAX_VALUE, width === 4 ? 1.401298464324817e-45 : Number.MIN_VALUE]) {
            const large = new Owner(magnitude, magnitude), quotient = large.divide(large);
            equal(quotient.real, 1, 'scaled division'); equal(quotient.imaginary, 0, 'scaled division imaginary'); quotient.free(); large.free();
        }
        const patterns = width === 4 ? [0x7fc12345n, 0x7f812345n, 0xffc54321n, 0x80000000n, 0n, 1n, 0x7f800000n] : [0x7ff8123456789abcn, 0x7ff0123456789abcn, 0xfff8abcdef123456n, 0x8000000000000000n, 0n, 1n, 0x7ff0000000000000n];
        const bits = values => {
            const data = new DataView(values.buffer, values.byteOffset, values.byteLength);
            return [0, width].map(position => width === 4 ? BigInt(data.getUint32(position, true)) : data.getBigUint64(position, true));
        };
        for (let index = 0; index < patterns.length; ++index) {
            const storage = new ArrayType(2), data = new DataView(storage.buffer), expected = [patterns[index], patterns[(index + 1) % patterns.length]];
            expected.forEach((value, field) => width === 4 ? data.setUint32(field * width, Number(value), true) : data.setBigUint64(field * width, value, true));
            const owned = Owner.from_components(storage), cloned = owned.cloned(); source.set_components(storage);
            for (const owner of [owned, cloned, source]) assert(bits(owner.components()).every((value, field) => value === expected[field]), 'complete component payloads');
            const conjugate = owned.conjugate(), conjugatedBits = bits(conjugate.components());
            assert(conjugatedBits[0] === expected[0] && conjugatedBits[1] === (expected[1] ^ (1n << BigInt(width * 8 - 1))), 'conjugate flips imaginary sign only');
            const retained = owned.components(); owned.free(); cloned.free(); conjugate.free();
            assert(bits(retained).every((value, field) => value === expected[field]), 'snapshot survives owner release');
        }
        source.free(); copy.free(); equal(snapshot[0], 3, 'original snapshot survives release');
    }
    return { cases, comparisons, owners: 2, hostDifferences };
}
