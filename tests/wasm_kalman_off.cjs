const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /kalman_filter_|kalman_smooth_|kalmanestimate/i.test(name)), 'all Kalman functions and owners absent without wasm');
console.log('Ciglet Kalman feature-disabled WASM: six functions and two owners absent');
