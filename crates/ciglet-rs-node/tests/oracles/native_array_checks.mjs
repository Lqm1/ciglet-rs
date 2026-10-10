export async function verifyArrayBindings(api, loadFixture) {
    const assert = (value, label) => { if (!value) throw new Error(label); };
    const throws = action => { let failed = false; try { action(); } catch { failed = true; } assert(failed, 'expected array error'); };
    let cases = 0, comparisons = 0;
    const equal = (actual, expected, label) => { assert(Object.is(actual, expected), `${label}: ${actual} != ${expected}`); ++comparisons; };
    const bytesOf = array => new Uint8Array(array.buffer, array.byteOffset, array.byteLength);
    const sameBits = (actual, expected, label) => { const a = bytesOf(actual), b = bytesOf(expected); assert(a.length === b.length, label + ' length'); a.forEach((byte, index) => equal(byte, b[index], label)); };
    const types = [[1, 'U8', Uint8Array], [2, 'U16', Uint16Array], [4, 'U32', Uint32Array], [8, 'U64', BigUint64Array], [4, 'F32', Float32Array], [8, 'F64', Float64Array]];
    const bytes = new Uint8Array(await loadFixture('array.bin')), view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    assert(String.fromCharCode(...bytes.subarray(0, 4)) === 'ARR1' && view.getUint32(4, true) === 240, 'full array corpus');
    let offset = 8;
    const integer = () => { const value = view.getUint32(offset, true); offset += 4; return value; };
    for (let record = 0; record < 240; ++record) {
        const rows = integer(), columns = integer(), width = integer(), size = rows * columns * width, fields = [];
        for (let field = 0; field < 5; ++field) { fields.push(bytes.slice(offset, offset + size)); offset += size; }
        for (const [elementWidth, suffix, ArrayType] of types) if (elementWidth === width) {
            const Owner = api[`Array2${suffix}`], source = new ArrayType(fields[0].slice().buffer), saved = source.slice(), owned = new Owner(rows, columns, source);
            equal(owned.rows, rows, 'rows'); equal(owned.columns, columns, 'columns'); equal(owned.length, rows * columns, 'length'); equal(owned.is_empty(), rows * columns === 0, 'empty');
            sameBits(owned.values(), source, 'complete constructor bits');
            const zero = Owner.zeros(rows, columns); sameBits(zero.values(), fields[1], 'C zeros'); zero.free();
            const cloned = owned.cloned(); sameBits(cloned.flatten(), fields[2], 'C copy flatten'); sameBits(owned.flatten(), fields[3], 'C original flatten');
            const transposed = owned.transposed(); equal(transposed.rows, columns, 'transposed rows'); equal(transposed.columns, rows, 'transposed columns'); sameBits(transposed.values(), fields[4], 'C transposition');
            owned.transpose(); sameBits(owned.values(), fields[4], 'in-place transposition'); owned.transpose(); sameBits(owned.values(), source, 'inverse transposition');
            const packed = Owner.from_rows(source, new Uint32Array(rows).fill(columns)); sameBits(packed.values(), source, 'complete native row construction'); equal(packed.rows, rows, 'from rows metadata'); packed.free();
            if (rows) { const row = owned.row(0); sameBits(row, source.subarray(0, columns), 'row snapshot'); }
            assert(owned.row(rows) === null && owned.get(rows, 0) === null && owned.get(0, columns) === null, 'missing row/cell undefined');
            const retained = owned.flatten(); owned.free(); sameBits(retained, source, 'snapshot lifetime'); sameBits(cloned.values(), source, 'clone lifetime');
            const moved = cloned.into_row_major(); sameBits(moved, source, 'consumed flat storage'); throws(() => cloned.values());
            transposed.free(); sameBits(source, saved, 'original input immutable'); ++cases;
        }
    }
    assert(offset === bytes.byteLength, 'full ARR1 corpus consumed');
    for (const [, suffix, ArrayType] of types) {
        const Owner = api[`Array2${suffix}`], scalar = value => suffix === 'U64' ? BigInt(value) : value;
        const source = new ArrayType([scalar(1), scalar(2), scalar(3), scalar(4), scalar(5), scalar(6)]), owned = new Owner(2, 3, source);
        owned.reshape(3, 2); equal(owned.rows, 3, 'reshape rows'); equal(owned.get(2, 1), scalar(6), 'get after reshape');
        assert(owned.set(2, 1, scalar(9)), 'mutable cell'); equal(owned.get(2, 1), scalar(9), 'cell updated'); assert(!owned.set(3, 0, scalar(99)), 'missing mutable cell');
        owned.set_row(0, new ArrayType([scalar(7), scalar(8)])); sameBits(owned.row(0), new ArrayType([scalar(7), scalar(8)]), 'mutable row');
        const before = owned.values();
        for (const action of [() => owned.reshape(2, 2), () => owned.reshape(0xffffffff, 2), () => owned.set_values(new ArrayType(5)), () => owned.set_row(0, new ArrayType(1)), () => owned.set_row(3, new ArrayType(2))]) {
            throws(action); sameBits(owned.values(), before, 'atomic owner errors'); equal(owned.rows, 3, 'atomic shape');
        }
        owned.set_values(source); sameBits(owned.values(), source, 'mutable complete slice'); owned.free();
        const filled = Owner.filled(2, 3, scalar(17)); filled.values().forEach(value => equal(value, scalar(17), 'filled value')); filled.free();
        for (const [rows, columns] of [[0, 0], [0, 3], [5, 0], [0, 0xffffffff], [0xffffffff, 0]]) {
            const empty = Owner.zeros(rows, columns), copied = empty.transposed(); empty.transpose();
            equal(empty.rows, columns, 'empty transpose rows'); equal(empty.columns, rows, 'empty transpose columns'); equal(copied.rows, columns, 'empty copy rows');
            assert(empty.is_empty() && empty.length === 0, 'full empty shape'); if (columns > 0) assert(empty.row(0).length === 0, 'zero-column row'); empty.free(); copied.free();
        }
        for (const action of [() => new Owner(2, 3, new ArrayType(5)), () => Owner.zeros(2 ** 53, 2), () => Owner.filled(2 ** 53, 2, scalar(1)), () => Owner.from_rows(new ArrayType(3), new Uint32Array([1, 2])), () => Owner.from_rows(new ArrayType(1), new Uint32Array([2])), () => Owner.from_rows(new ArrayType(1), new Uint32Array())]) throws(action);
        const emptyRows = Owner.from_rows(new ArrayType(), new Uint32Array([0, 0])); equal(emptyRows.rows, 2, 'two empty rows'); equal(emptyRows.columns, 0, 'empty row columns'); emptyRows.free();
    }
    for (const [width, suffix, ArrayType] of types.filter(type => type[0] >= 4)) {
        const patterns = width === 4 ? [0x7f812345n, 0xffc12345n, 0x80000000n, 0n, 1n, 0x7f800000n] : [0x7ff0123456789abcn, 0xfff8123456789abcn, 0x8000000000000000n, 0n, 1n, 0x7ff0000000000000n];
        const source = new ArrayType(6), view = new DataView(source.buffer);
        patterns.forEach((value, index) => width === 4 ? view.setUint32(index * width, Number(value), true) : view.setBigUint64(index * width, value, true));
        const Owner = api[`Array2${suffix}`], owned = new Owner(2, 3, source); sameBits(owned.values(), source, 'arbitrary full component bits');
        owned.transpose(); owned.transpose(); sameBits(owned.values(), source, 'payload permutation');
        owned.set_values(source); owned.set_row(1, source.subarray(3)); sameBits(owned.values(), source, 'mutable payloads');
        const copied = owned.cloned(), retained = owned.flatten(); owned.free(); sameBits(copied.into_row_major(), source, 'consumed payload bits'); sameBits(retained, source, 'retained payload snapshot');
    }
    return {cases, comparisons, owners:6, originalCRecords:240};
}
