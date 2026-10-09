const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.includes('special_')), 'all 28 special functions absent without wasm');
console.log('Ciglet special feature-disabled WASM: all28 functions absent');
