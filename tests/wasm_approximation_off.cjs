const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const prefix of ['approximation_', 'scalar_minimum_', 'scalar_maximum_', 'legacy_epsilon', 'legacy_infinity_sentinel']) {
    assert(!names.some(name => name.includes(prefix)), 'scalar approximation absent without wasm');
}
console.log('Ciglet scalar approximation feature-disabled WASM: all32 functions absent');
