const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.includes('trigonometric_')), 'all ten trigonometric functions absent without wasm');
console.log('Ciglet trigonometric feature-disabled WASM: all10 functions absent');
