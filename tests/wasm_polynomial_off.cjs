const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /polynomial_evaluate_|polynomial_roots_|polynomialroots/i.test(name)), 'all polynomial functions and reports absent without wasm');
console.log('Ciglet polynomial feature-disabled WASM: eight functions and two reports absent');
