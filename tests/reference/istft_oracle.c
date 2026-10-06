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
    FILE *file = fopen(argv[1], "wb"); if (!file) return 3;
    if (fwrite("IST1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE)); integer(file, 288);
    int fades[] = {0,1,8,32}; int offsets[] = {-3,0,5};
    for (int padding = 1; padding <= 2; padding++)
    for (int frame_case = 0; frame_case < 2; frame_case++)
    for (int fade = 0; fade < 4; fade++)
    for (int offset = 0; offset < 3; offset++)
    for (int scenario = 0; scenario < 3; scenario++)
    for (int norm = 0; norm < 2; norm++) {
        int nfft = 32 * padding, bins = nfft / 2 + 1;
        int frames = frame_case == 0 ? 2 : 5;
        FP_TYPE **magnitudes = calloc(frames, sizeof(FP_TYPE*));
        FP_TYPE **phases = calloc(frames, sizeof(FP_TYPE*));
        if (!magnitudes || !phases) return 3;
        integer(file, nfft); integer(file, frames); integer(file, fades[fade]);
        integer(file, (uint32_t)offsets[offset]); integer(file, scenario);
        FP_TYPE normalization = norm == 0 ? 0.75 : 2.0;
        scalar(file, normalization);
        for (int frame = 0; frame < frames; frame++) {
            magnitudes[frame] = calloc(bins, sizeof(FP_TYPE));
            phases[frame] = calloc(bins, sizeof(FP_TYPE));
            if (!magnitudes[frame] || !phases[frame]) return 3;
            for (int bin = 0; bin < bins; bin++) {
                if (scenario == 0) magnitudes[frame][bin] = bin == nfft / 2 ? 1.0 : 0;
                else if (scenario == 1) {
                    magnitudes[frame][bin] = (bin % 7 + frame + 1) * 0.125;
                    phases[frame][bin] = (bin % 11 - 5) * 0.25;
                } else {
                    magnitudes[frame][bin] = bin == 3 ? 2.0 : 0;
                    phases[frame][bin] = frame * 0.125;
                }
                scalar(file, magnitudes[frame][bin]); scalar(file, phases[frame][bin]);
            }
        }
        int count = 0;
        FP_TYPE *output = cig_stft_backward(magnitudes, phases, 4, frames,
            offsets[offset], 8, padding, fades[fade], normalization, &count);
        if (!output) return 3;
        integer(file, count);
        for (int sample = 0; sample < count; sample++) scalar(file, output[sample]);
        free(output);
        for (int frame = 0; frame < frames; frame++) { free(magnitudes[frame]); free(phases[frame]); }
        free(magnitudes); free(phases);
    }
    return fclose(file) != 0 ? 3 : 0;
}
