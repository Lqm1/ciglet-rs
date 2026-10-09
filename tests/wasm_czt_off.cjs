const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /czt_f32|czt_f64|cztplanf32|cztplanf64/i.test(name)), 'both CZT functions and plans absent without wasm');
console.log('Ciglet CZT feature-disabled WASM: two functions and two plans absent');
