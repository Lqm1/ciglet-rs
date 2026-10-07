#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f, uint32_t x) { if (fwrite(&x, 4, 1, f) != 1) exit(3); }
static void scalar(FILE *f, FP_TYPE x) { if (fwrite(&x, sizeof(x), 1, f) != 1) exit(3); }
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *f = fopen(argv[1], "wb"); if (!f) return 3;
    fwrite("CFB1", 1, 4, f); integer(f, sizeof(FP_TYPE)); integer(f, 33);
    FP_TYPE metadata[] = {8000, 0, -1, NAN, INFINITY};
    for (int kind = 0; kind < 7; ++kind) for (int m = 0; m < 5; ++m) {
        if (kind == 5 && m >= 3) continue;
        int bins = kind == 4 || kind == 6 ? 0 : 6, channels = kind == 3 ? 0 : 2;
        filterbank *bank = kind >= 5 ? cig_create_plp_filterbank(bins, metadata[m], channels)
                                    : cig_create_empty_filterbank(bins, metadata[m], channels);
        integer(f, kind); integer(f, bins); integer(f, channels); scalar(f, bank->fnyq);
        for (int c = 0; c < channels; ++c) {
            int lower = kind == 1 ? (c == 0 ? 1 : 4) : 0;
            int upper = kind == 1 ? (c == 0 ? 3 : 5) : bins;
            if (kind == 2) lower = upper = 3;
            if (kind < 5) { bank->lower_idx[c] = lower; bank->upper_idx[c] = upper; }
            integer(f, lower); integer(f, upper);
            for (int k = 0; k < bins; ++k) {
                FP_TYPE weight = k == 1 ? -0.125 : (c + 1) * 0.5;
                if (k < lower || k >= upper) weight = k % 2 ? INFINITY : NAN;
                if (kind < 5) bank->fresp[c][k] = weight;
                scalar(f, bank->fresp[c][k]);
            }
        }
        FP_TYPE *spectra[3];
        for (int t = 0; t < 3; ++t) {
            spectra[t] = calloc(bins, sizeof(FP_TYPE));
            for (int k = 0; k < bins; ++k) {
                FP_TYPE x = (t + 1) * (k + 1) * 0.25;
                if ((kind == 1 && (k == 0 || k == 3 || k == 5)) || kind == 2 || kind == 3)
                    x = k % 2 ? -INFINITY : NAN;
                spectra[t][k] = x; scalar(f, x);
            }
        }
        for (int compression = 0; compression < 2; ++compression) {
            for (int t = 0; t < 3; ++t) {
                FP_TYPE *x = cig_filterbank_spectrum(bank, spectra[t], -123, 0, compression);
                for (int c = 0; c < channels; ++c) {
                    if (!isfinite(x[c])) return 4;
                    scalar(f, x[c]);
                }
                free(x);
            }
            FP_TYPE **x = cig_filterbank_spectrogram(bank, spectra, 3, 0, -456, compression);
            for (int t = 0; t < 3; ++t) for (int c = 0; c < channels; ++c) {
                if (!isfinite(x[t][c])) return 4;
                scalar(f, x[t][c]);
            }
            free2d(x, 3);
        }
        for (int t = 0; t < 3; ++t) free(spectra[t]);
        cig_delete_filterbank(bank);
    }
    return fclose(f) != 0;
}
