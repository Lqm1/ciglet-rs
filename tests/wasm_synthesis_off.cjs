const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /sinusoid_f|sinusoids_f|sinusoidf/i.test(name)), 'all synthesis functions and owners absent without wasm');
console.log('Ciglet synthesis feature-disabled WASM: four functions and two owners absent');
