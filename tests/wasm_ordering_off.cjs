const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['minimum', 'maximum', 'median', 'select_nth', 'sort']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'ordering absent without wasm');
}
for (const owner of ['sortedvaluesf32', 'sortedvaluesf64']) assert(!names.some(name => name.includes(owner)), 'sort result owner absent without wasm');
console.log('Ciglet ordering feature-disabled WASM: all10 functions/2classes absent');
