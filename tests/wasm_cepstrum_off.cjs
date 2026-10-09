const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
assert(!names.some(name => /band_to_cepstrum_|band_spectrogram_to_cepstra_|bandcepstraloptions/i.test(name)), 'all six band cepstrum functions and options absent without wasm');
console.log('Ciglet band cepstrum feature-disabled WASM: six functions and one owner absent');
