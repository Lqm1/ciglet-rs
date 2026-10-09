const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /complexf32|complexf64/i.test(name)), 'both complex owners absent without wasm');
console.log('Ciglet complex feature-disabled WASM: both owners absent');
