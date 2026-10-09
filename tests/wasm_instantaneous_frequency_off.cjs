const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /instantaneousfrequency/i.test(name)), 'all instantaneous frequency owners absent without wasm');
console.log('Ciglet instantaneous frequency feature-disabled WASM: four owners absent');
