const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /real_cepstrum_|realcepstrumplan/i.test(name)), 'all real cepstrum functions and plans absent without wasm');
console.log('Ciglet real cepstrum feature-disabled WASM: six functions and two plans absent');
