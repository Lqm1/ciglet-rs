const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.includes('aliased_sinc_')), 'all eight sinc functions absent without wasm');
console.log('Ciglet sinc feature-disabled WASM: all8 functions absent');
