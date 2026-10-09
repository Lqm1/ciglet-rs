const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /correlogram_compute_|correlogram_to_frequency_|correlogramframe/i.test(name)), 'all correlogram functions and frame absent without wasm');
console.log('Ciglet correlogram feature-disabled WASM: six functions and frame owner absent');
