const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.startsWith('lanes_')), 'all sixteen basic lane functions absent without wasm');
console.log('Ciglet lanes feature-disabled WASM: all16 functions absent');
