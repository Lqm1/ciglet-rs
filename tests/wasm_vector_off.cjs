const fs = require('node:fs');
const assert = require('node:assert/strict');
const exportsFound = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
const operations = ['difference', 'cumulative_sum', 'reverse', 'linear_grid', 'interpolate_between'];
for (const name of ['integer_sequence', ...operations.flatMap(operation => [operation + '_f32', operation + '_f64'])]) {
    assert(!exportsFound.some(entry => entry.includes(name)), name + ' must be absent without wasm');
}
console.log('Ciglet vectors feature-disabled WASM: all11 functions absent');
