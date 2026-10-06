#include <stdint.h>
#include "ciglet.h"

static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb");
    if (!file) return 3;
    const int lengths[] = {2, 3, 5, 16, 31, 128, 512, 1001};
    if (fwrite("SYN1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE)); integer(file, 48);
    for (int size = 0; size < 8; size++) {
        int n = lengths[size];
        for (int scenario = 0; scenario < 6; scenario++) {
            int fs = size % 2 ? 44100 : 16000;
            FP_TYPE frequency[3] = {375, 440, 880};
            FP_TYPE amplitude[3] = {1, 0.25, -0.125};
            FP_TYPE phase[3] = {0.25, 0.125, 0.5};
            int count = 1;
            int single = scenario == 0 || scenario == 5;
            if (scenario == 1) { frequency[0] = -250; amplitude[0] = -0.5; phase[0] = -0.25; }
            if (scenario == 2) { count = 3; frequency[0] = 220; amplitude[0] = 0.75; phase[0] = 0; }
            if (scenario == 3) { count = 2; frequency[0] = 0; frequency[1] = fs * 0.5; amplitude[0] = 0.125; amplitude[1] = 0.5; phase[1] = 0; }
            if (scenario == 4) count = 0;
            if (scenario == 5) { frequency[0] = 0; phase[0] = 0; }
            FP_TYPE *result = single ? gensin(frequency[0], amplitude[0], phase[0], n, fs)
                : cig_gensins(frequency, amplitude, phase, count, fs, n);
            if (!result) return 3;
            integer(file, single); integer(file, n); integer(file, fs); integer(file, count);
            for (int i = 0; i < count; i++) {
                FP_TYPE tone[] = {frequency[i], amplitude[i], phase[i]};
                if (fwrite(tone, sizeof(FP_TYPE), 3, file) != 3) return 3;
            }
            if (fwrite(result, sizeof(FP_TYPE), n, file) != (size_t)n) return 3;
            free(result);
        }
    }
    return fclose(file) != 0 ? 3 : 0;
}
