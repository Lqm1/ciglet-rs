# Attribution

Reimplemented from ciglet, copyright 2016-2019 Kanru Hua, whose source header contains the BSD 3-Clause license. Audited upstream commit: `895ba9b1c0eabee83d3544208bbc82420efa3206`.

Retain upstream and bundled-code notices when adapting the respective algorithms. Bundled code provenance must be reviewed per module; the upstream library also contains third-party FFT, WAV, median, and approximate-math code. The current Rust modules implement the algorithms exposed by ciglet's own header and do not link its C code.
