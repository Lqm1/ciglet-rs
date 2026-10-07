#include <stdint.h>
#include <float.h>
#include "ciglet.h"

static uint32_t calls;
static float trig_float(double angle) {
  ++calls; return 0.75 - angle * 0.0625 + angle * angle * 0.00390625;
}
static double trig_double(double angle) {
  ++calls; return 0.75 - angle * 0.0625 + angle * angle * 0.00390625;
}
CIG_DEF_HANNING(custom_hann_float, trig_float)
CIG_DEF_HANNING(custom_hann_double, trig_double)
CIG_DEF_HAMMING(custom_hamming_float, trig_float)
CIG_DEF_HAMMING(custom_hamming_double, trig_double)
CIG_DEF_MLTSINE(custom_sine_float, trig_float)
CIG_DEF_MLTSINE(custom_sine_double, trig_double)
CIG_DEF_BLACKMAN_HARRIS(custom_harris_float, trig_float)
CIG_DEF_BLACKMAN_HARRIS(custom_harris_double, trig_double)
CIG_DEF_NUTTALL98(custom_nuttall_float, trig_float)
CIG_DEF_NUTTALL98(custom_nuttall_double, trig_double)
CIG_DEF_BLACKMAN(ignored_blackman_float, trig_float)

/* The original Blackman macro ignores cosfunc. These two explicit adapters
   change only that ignored token at instantiation; the header stays unchanged. */
#undef cos_3
#define cos_3 trig_float
CIG_DEF_BLACKMAN(custom_blackman_float, trig_float)
#undef cos_3
#define cos_3 trig_double
CIG_DEF_BLACKMAN(custom_blackman_double, trig_double)

static FP_TYPE seed;
static FP_TYPE combine(FP_TYPE accumulator, FP_TYPE value) {
  ++calls; return accumulator * (FP_TYPE)0.5 - value * (FP_TYPE)0.125;
}
CIG_DEF_N_TO_ONE(custom_fold, combine, seed)
static void integer(FILE* file, uint32_t value) { fwrite(&value, 4, 1, file); }

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  calls = 0; FP_TYPE* ignored = ignored_blackman_float(8); free(ignored);
  if (calls != 0) return 4;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  fwrite("CUS1", 1, 4, file); integer(file, sizeof(FP_TYPE)); integer(file, 60);
  FP_TYPE* (*functions[6][2])(int) = {
    {custom_hann_float, custom_hann_double},
    {custom_hamming_float, custom_hamming_double},
    {custom_sine_float, custom_sine_double},
    {custom_blackman_float, custom_blackman_double},
    {custom_harris_float, custom_harris_double},
    {custom_nuttall_float, custom_nuttall_double}
  };
  uint32_t lengths[] = {0, 2, 3, 8, 31};
  for (uint32_t kind = 0; kind < 6; ++kind) for (uint32_t callback = 0; callback < 2; ++callback)
    for (uint32_t size = 0; size < 5; ++size) {
      uint32_t n = lengths[size]; calls = 0;
      FP_TYPE* result = functions[kind][callback](n);
      integer(file, kind); integer(file, callback ? 8 : 4); integer(file, n); integer(file, calls);
      fwrite(result, sizeof(FP_TYPE), n, file); free(result);
    }
  fwrite("RED1", 1, 4, file); integer(file, 12);
  uint32_t sizes[] = {0, 1, 3, 8};
  FP_TYPE maximum = sizeof(FP_TYPE) == 4 ? FLT_MAX : DBL_MAX;
  FP_TYPE seeds[] = {0.25, -0.0, maximum};
  for (uint32_t size = 0; size < 4; ++size) for (uint32_t initial = 0; initial < 3; ++initial) {
    uint32_t n = sizes[size]; FP_TYPE values[8];
    for (uint32_t i = 0; i < n; ++i) values[i] = ((int)i - 4) * 0.25;
    seed = seeds[initial]; calls = 0; FP_TYPE result = custom_fold(values, n);
    integer(file, n); integer(file, calls); fwrite(&seed, sizeof(seed), 1, file);
    fwrite(values, sizeof(FP_TYPE), n, file); fwrite(&result, sizeof(result), 1, file);
  }
  return fclose(file) != 0 ? 3 : 0;
}
