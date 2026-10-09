const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /spectral_peak_interpolate_|spectralpeak/i.test(name)), 'both peak functions and owners absent without wasm');
console.log('Ciglet spectral peak feature-disabled WASM: two functions and two owners absent');
