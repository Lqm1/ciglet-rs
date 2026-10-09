const fs = require('node:fs');
const assert = require('node:assert/strict');
const names = WebAssembly.Module.exports(new WebAssembly.Module(fs.readFileSync(process.argv[2]))).map(entry => entry.name);
for (const operation of ['frequency_to_mel', 'mel_to_frequency', 'frequency_to_bark', 'bark_to_frequency', 'equal_loudness', 'mel_space', 'itakura_saito', 'log_itakura_saito', 'log_itakura_saito_with']) {
    for (const suffix of ['f32', 'f64']) assert(!names.some(name => name.includes(`${operation}_${suffix}`)), 'frequency/distance absent without wasm');
}
console.log('Ciglet frequency/distance feature-disabled WASM: all18 functions absent');
