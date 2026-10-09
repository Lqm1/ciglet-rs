const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['sum', 'sum_squares', 'mean', 'mean_with', 'variance', 'variance_with', 'dot', 'covariance', 'covariance_with', 'correlation', 'correlation_with', 'cross_correlation']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'statistics absent without wasm');
}
console.log('Ciglet statistics feature-disabled WASM: all24 functions absent');
