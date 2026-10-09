const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /spectral_envelope_estimate_|spectralenvelope/i.test(name)), 'all envelope functions and owners absent without wasm');
console.log('Ciglet envelope feature-disabled WASM: two functions and four owners absent');
