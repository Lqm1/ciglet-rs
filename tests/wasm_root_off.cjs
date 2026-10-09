const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => name.includes('root_find_')), 'both root functions absent without wasm');
console.log('Ciglet root feature-disabled WASM: two functions absent');
