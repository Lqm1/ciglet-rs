const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name.toLowerCase());
for (const owner of ['filterbankf32', 'filterbankf64', 'meloptionsf32', 'meloptionsf64']) {
    assert(!names.some(name => name.includes(owner)), 'filterbank owner absent without wasm');
}
console.log('Ciglet filterbank feature-disabled WASM: all four owners absent');
