const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /array2u8|array2u16|array2u32|array2u64|array2f32|array2f64/i.test(name)), 'all six array owners absent without wasm');
console.log('Ciglet arrays feature-disabled WASM: six owners absent');
