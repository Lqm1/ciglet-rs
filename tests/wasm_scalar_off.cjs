const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /scalar_add_|scalar_accumulate_square_|scalar_fold_|scalar_math_constants/i.test(name)), 'all scalar functions absent without wasm');
console.log('Ciglet scalar feature-disabled WASM: seven functions absent');
