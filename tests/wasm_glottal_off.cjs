const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /glottalmodel|glottalprepared|glottalspectrum/i.test(name)), 'all LF owners absent without wasm');
console.log('Ciglet LF feature-disabled WASM: six owners absent');
