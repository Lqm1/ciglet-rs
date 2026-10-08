const fs = require('node:fs');
const assert = require('node:assert/strict');
const exportsFound = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
const operations = ['dct', 'phase_wrap', 'phase_unwrap', 'phase_difference', 'magnitude', 'argument', 'polar_real', 'polar_imaginary', 'complete_spectrum'];
for (const operation of operations) for (const precision of ['f32', 'f64']) {
    const name = operation + '_' + precision;
    assert(!exportsFound.some(entry => entry.includes(name)), name + ' must be absent without wasm');
}
console.log('Ciglet phase/DCT feature-disabled WASM: all18 functions absent');
