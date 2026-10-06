#include <stdint.h>
#include "ciglet.h"

static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb");
    if (!file) return 3;
    const int lengths[] = {1, 2, 3, 5, 8, 11, 16, 31, 64, 127};
    if (fwrite("CZT1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE)); integer(file, 1200);
    for (int size = 0; size < 10; size++) {
        int n = lengths[size];
        FP_TYPE *real = malloc(n * sizeof(FP_TYPE));
        FP_TYPE *imaginary = malloc(n * sizeof(FP_TYPE));
        FP_TYPE *out_real = malloc(n * sizeof(FP_TYPE));
        FP_TYPE *out_imaginary = malloc(n * sizeof(FP_TYPE));
        if (!real || !imaginary || !out_real || !out_imaginary) return 3;
        FP_TYPE steps[] = {2.0 * M_PI / n, 0, 0.125, -0.25, 2.0 * M_PI / (n * 1.5)};
        for (int scenario = 0; scenario < 3; scenario++) {
            for (int i = 0; i < n; i++) {
                if (scenario == 0) {
                    real[i] = i == n / 3 ? 0.75 : 0;
                    imaginary[i] = i == n / 4 ? -0.25 : 0;
                } else if (scenario == 1) {
                    real[i] = (i % 17 - 8) * 0.125;
                    imaginary[i] = (i % 11 - 5) * 0.0625;
                } else {
                    real[i] = 2048.0 + (i % 7) * 0.125;
                    imaginary[i] = -1024.0 + (i % 5) * 0.0625;
                }
            }
            for (int step = 0; step < 5; step++) {
                for (int mask = 0; mask < 4; mask++) {
                    for (int inverse = 0; inverse < 2; inverse++) {
                        cig_czt(mask & 1 ? real : NULL, mask & 2 ? imaginary : NULL,
                            out_real, out_imaginary, inverse ? -steps[step] : steps[step], n);
                        if (inverse) for (int i = 0; i < n; i++) { out_real[i] /= n; out_imaginary[i] /= n; }
                        integer(file, n); integer(file, scenario); integer(file, mask); integer(file, inverse);
                        if (fwrite(&steps[step], sizeof(FP_TYPE), 1, file) != 1) return 3;
                        for (int i = 0; i < n; i++) {
                            if (fwrite(&out_real[i], sizeof(FP_TYPE), 1, file) != 1) return 3;
                            if (fwrite(&out_imaginary[i], sizeof(FP_TYPE), 1, file) != 1) return 3;
                        }
                    }
                }
            }
        }
        free(real); free(imaginary); free(out_real); free(out_imaginary);
    }
    return fclose(file) != 0 ? 3 : 0;
}
