const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /matrix_apply_swaps_|matrixf32|matrixf64|luf32|luf64/i.test(name)), 'all matrix/LU owners and vector swaps absent without wasm');
console.log('Ciglet matrix feature-disabled WASM: ten functions and four owners absent');
