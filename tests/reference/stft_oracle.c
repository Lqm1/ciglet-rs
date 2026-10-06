#include <stdint.h>
#include "ciglet.h"

static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}

static void scalar(FILE *file, FP_TYPE value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb");
    if (!file) return 3;
    char *windows[] = {"boxcar", "hanning", "hamming", "blackman", "blackman_harris"};
    int centers[] = {-3, 5, 13, 21};
    FP_TYPE signal[27];
    FP_TYPE magnitude_storage[4][17], phase_storage[4][17];
    FP_TYPE *magnitude[4], *phase[4];
    for (int frame = 0; frame < 4; frame++) {
        magnitude[frame] = magnitude_storage[frame];
        phase[frame] = phase_storage[frame];
    }
    if (fwrite("STF1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE));
    integer(file, 480);
    for (int window = 0; window < 5; window++)
    for (int approximate = 0; approximate < 2; approximate++)
    for (int subtract_mean = 0; subtract_mean < 2; subtract_mean++)
    for (int variable = 0; variable < 2; variable++)
    for (int scenario = 0; scenario < 3; scenario++)
    for (int mask = 0; mask < 4; mask++) {
        int lengths[4];
        for (int frame = 0; frame < 4; frame++)
            lengths[frame] = variable ? 8 + 4 * frame : 16;
        for (int sample = 0; sample < 27; sample++) {
            if (scenario == 0) signal[sample] = sample == 11 ? 0.75 : 0;
            else if (scenario == 1) signal[sample] = (sample % 9 - 4) * 0.125;
            else signal[sample] = 2.0;
        }
        FP_TYPE normalization = 0, weight = 0;
        /* Variable lengths intentionally omit factors: the original reuses
           the first window when either factor is requested. */
        cig_stft_forward(signal, 27, centers, lengths, 4, 32,
            windows[window], subtract_mean, approximate ? 2 : 3,
            variable ? NULL : &normalization, variable ? NULL : &weight,
            mask & 1 ? magnitude : NULL, mask & 2 ? phase : NULL);
        integer(file, window); integer(file, approximate);
        integer(file, subtract_mean); integer(file, variable);
        integer(file, scenario); integer(file, mask);
        scalar(file, normalization); scalar(file, weight);
        for (int frame = 0; frame < 4; frame++)
        for (int bin = 0; bin < 17; bin++) {
            if (mask & 1) scalar(file, magnitude[frame][bin]);
            if (mask & 2) scalar(file, phase[frame][bin]);
        }
    }
    return fclose(file) != 0 ? 3 : 0;
}
