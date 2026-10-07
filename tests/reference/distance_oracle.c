#include <stdint.h>
#include <float.h>
#include "ciglet.h"
static void record(FILE* file, FP_TYPE* source, FP_TYPE* reference, uint32_t n) {
  fwrite(&n, 4, 1, file); fwrite(source, sizeof(FP_TYPE), n, file); fwrite(reference, sizeof(FP_TYPE), n, file);
  FP_TYPE result = itakura_saito(source, reference, n); fwrite(&result, sizeof(result), 1, file);
}
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  uint32_t width = sizeof(FP_TYPE), cases = 53;
  fwrite("ISD1", 1, 4, file); fwrite(&width, 4, 1, file); fwrite(&cases, 4, 1, file);
  uint32_t sizes[] = {1, 2, 3, 8, 17, 32};
  for (int size = 0; size < 6; ++size) for (int shape = 0; shape < 8; ++shape) {
    FP_TYPE source[32], reference[32]; uint32_t n = sizes[size];
    for (uint32_t j = 0; j < n; ++j) {
      reference[j] = 0.5 + (j % 5) * 0.125;
      FP_TYPE ratio = shape == 0 ? 1 : shape == 1 ? 2 : shape == 2 ? 0.5 : shape == 3 ? 0.25 + (j % 7) * 0.5 : shape == 4 ? (j % 2 ? 1.000001 : 0.999999) : shape == 5 ? 1e-6 : shape == 6 ? 1e6 : 0;
      source[j] = reference[j] * ratio;
    }
    record(file, source, reference, n);
  }
  FP_TYPE maximum = sizeof(FP_TYPE) == 4 ? FLT_MAX : DBL_MAX;
  FP_TYPE tiny = sizeof(FP_TYPE) == 4 ? FLT_TRUE_MIN : DBL_TRUE_MIN;
  FP_TYPE source[] = {maximum, tiny, 1, 0, maximum}, reference[] = {tiny, maximum, 0, 0, maximum};
  for (int i = 0; i < 5; ++i) record(file, &source[i], &reference[i], 1);
  return fclose(file) != 0 ? 3 : 0;
}
