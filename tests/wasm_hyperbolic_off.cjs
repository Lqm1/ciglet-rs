const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.includes('hyperbolic_')), 'all eight hyperbolic functions absent without wasm');
console.log('Ciglet hyperbolic feature-disabled WASM: all8 functions absent');
