#include <stdint.h>
#include "ciglet.h"

static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb");
    if (!file) return 3;
    const int lengths[] = {1, 2, 3, 5, 8, 16, 32, 64};
    if (fwrite("DCT1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE));
    integer(file, 32);
    for (int size = 0; size < 8; size++) {
        int n = lengths[size];
        FP_TYPE *signal = malloc(n * sizeof(FP_TYPE));
        if (!signal) return 3;
        for (int scenario = 0; scenario < 4; scenario++) {
            for (int i = 0; i < n; i++) {
                switch (scenario) {
                    case 0: signal[i] = i == n / 3 ? 0.75 : 0; break;
                    case 1: signal[i] = (i % 17 - 8) * 0.125; break;
                    case 2: signal[i] = 2048.0; break;
                    default: signal[i] = i % 2 ? -0.5 : 0.5; break;
                }
            }
            FP_TYPE *result = cig_dct(signal, n);
            if (!result) return 3;
            integer(file, n); integer(file, scenario);
            if (fwrite(result, sizeof(FP_TYPE), n, file) != (size_t)n) return 3;
            free(result);
        }
        free(signal);
    }
    const float angles[] = {-1000, -201.125, -7, -3.25, -1, -0.0,
        0, 1, 3.25, 7, 201.125, 1000};
    integer(file, 12);
    for (int i = 0; i < 12; i++) {
        float record[] = {angles[i], fastsinfull(angles[i]), fastcosfull(angles[i])};
        if (fwrite(record, sizeof(float), 3, file) != 3) return 3;
    }
    return fclose(file) != 0 ? 3 : 0;
}
