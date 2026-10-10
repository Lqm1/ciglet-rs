const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
const { test } = require('node:test');
const api = require('..');
const root = path.resolve(__dirname, '../../..');

test('original C computational family oracles', async t => {
  const { bindingFamilies } = await import(pathToFileURL(path.join(__dirname, 'oracles/native_all_checks.mjs')));
  for (const [family, verify] of bindingFamilies) {
    await t.test(family, async () => {
      await verify(api, name => fs.readFile(path.join(root, 'tests/fixtures', name)));
    });
  }
});

test('callback identity, checked reentrancy, and recovery', () => {
  const context = { total: 2 };
  assert.equal(api.scalar_fold_f64(new Float64Array([1, 2]), 4, (a, b, c) => {
    assert.equal(c, context);
    return a + b;
  }, context), 7);
  const failure = new Error('original callback failure');
  assert.throws(() => api.scalar_fold_f64(new Float64Array([1]), 0, () => { throw failure; }, null), error => error === failure);
  let source;
  let recurse = true;
  const callback = () => recurse ? source.next_u32() : 12;
  source = new api.RandomWordSource(callback);
  assert.equal(source.callback(), callback);
  assert.throws(() => source.next_u32(), /already borrowed/);
  recurse = false;
  assert.equal(source.next_u32(), 12);
  source.close();
  assert.equal(source.is_closed, true);
  assert.throws(() => source.next_u32(), /consumed/);
});

test('copied mutation and checked dimensions', () => {
  const values = new Float64Array([8, 3, 6, 1]);
  const [selected, updated] = api.select_nth_f64(values, 2);
  assert.equal(selected, 6);
  assert.equal(updated[2], 6);
  assert.deepEqual(Array.from(values), [8, 3, 6, 1]);
  for (const invalid of [-1, 1.25, NaN, Infinity, 2 ** 53]) {
    assert.throws(() => new api.FftPlanF32(invalid));
  }
  assert.equal(api.version(), require('../package.json').version);
});

test('native process ownership captures output and errors', () => {
  const program = process.execPath;
  const figure = api.Figure.spawn(program, ['-e', 'process.stdin.pipe(process.stdout)']);
  assert.ok(figure.id() > 0);
  figure.append(new Uint8Array([1, 2, 3]));
  const output = figure.close_with_output();
  assert.equal(output.success, true);
  assert.deepEqual(Array.from(output.stdout()), [1, 2, 3]);
  assert.equal(figure.is_closed, true);
  assert.throws(() => figure.flush(), /consumed/);
});
