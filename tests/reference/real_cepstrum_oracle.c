#include <stdint.h>
#include "ciglet.h"
static void integer(FILE* file, uint32_t value) { if (fwrite(&value, 4, 1, file) != 1) exit(3); }
static void values(FILE* file, FP_TYPE* value, int n) { if (fwrite(value, sizeof(FP_TYPE), n, file) != (size_t)n) exit(3); }
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  fwrite("RCP1", 1, 4, file); integer(file, sizeof(FP_TYPE)); integer(file, 72);
  const int sizes[] = {2, 4, 8, 16, 32, 64};
  for (int size = 0; size < 6; ++size) for (int shape = 0; shape < 4; ++shape) for (int choice = 0; choice < 3; ++choice) {
    int n = sizes[size], bins = n / 2 + 1, count = choice == 0 ? 1 : choice == 1 ? bins : n;
    FP_TYPE logs[33];
    for (int j = 0; j < bins; ++j) logs[j] = shape == 0 ? -2.0 : shape == 1 ? (j % 7 - 3) * 0.25 : shape == 2 ? (j == bins / 2 ? 2.0 : -0.5) : (j % 2 ? 0.75 : -0.25);
    FP_TYPE* cepstrum = rceps(logs, n), *inverse = irceps(cepstrum, count, n), *phase = minphase(logs, n);
    FP_TYPE** magnitude = (FP_TYPE**)malloc2d(3, bins, sizeof(FP_TYPE));
    for (int i = 0; i < 3; ++i) for (int j = 0; j < bins; ++j) magnitude[i][j] = i == 0 ? 0 : i == 1 ? 0.5 + (j % 5) * 0.125 : (j == bins / 2 ? 2.0 : 0.25);
    FP_TYPE** coefficients = spgm2cegm(magnitude, 3, n, count);
    FP_TYPE** reconstructed = cegm2spgm(coefficients, 3, n, count);
    integer(file, n); integer(file, count); values(file, logs, bins);
    values(file, cepstrum, n); values(file, inverse, n); values(file, phase, n);
    for (int i = 0; i < 3; ++i) values(file, magnitude[i], bins);
    for (int i = 0; i < 3; ++i) values(file, coefficients[i], count);
    for (int i = 0; i < 3; ++i) values(file, reconstructed[i], bins);
    free(cepstrum); free(inverse); free(phase);
    free2d((void**)magnitude, 3); free2d((void**)coefficients, 3); free2d((void**)reconstructed, 3);
  }
  return fclose(file) != 0 ? 3 : 0;
}
