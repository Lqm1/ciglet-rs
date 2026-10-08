const fs = require('node:fs');
const assert = require('node:assert/strict');
const artifact = new WebAssembly.Module(fs.readFileSync(process.argv[2]));
const exportsFound = WebAssembly.Module.exports(artifact).map(entry => entry.name.toLowerCase());
for (const name of ['fft_f32', 'fft_f64', 'inverse_dft_f32', 'inverse_dft_f64', 'fft_shift_f32', 'fft_shift_f64', 'fftplanf32', 'fftplanf64', 'complexvaluesf32', 'complexvaluesf64']) {
    assert(!exportsFound.some(entry => entry.includes(name)), name + ' must be absent without wasm');
}
console.log('Ciglet Fourier feature-disabled WASM: all6 functions and4 classes absent');
