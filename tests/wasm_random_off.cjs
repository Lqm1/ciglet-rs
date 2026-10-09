const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /random_uniform_from_draw_|random_white_from_draw_|random_normal_from_draws_|randomwordsource|randomsampler/i.test(name)), 'all random functions and owners absent without wasm');
console.log('Ciglet random feature-disabled WASM: six functions and two owners absent');
