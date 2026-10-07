#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>
static int oracle_maximum;
static int draws[2];
static unsigned consumed;
static int oracle_rand(void) { return draws[consumed++ % 2]; }
#undef RAND_MAX
#define RAND_MAX oracle_maximum
#define rand oracle_rand
#include "ciglet.h"

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  uint32_t width = sizeof(FP_TYPE), cases = 128;
  fwrite("RND1", 1, 4, file); fwrite(&width, 4, 1, file); fwrite(&cases, 4, 1, file);
  for (int grid = 0; grid < 2; ++grid) {
    oracle_maximum = grid ? 2147483647 : 32767;
    int choices[] = {0, 1, 2, oracle_maximum / 4, oracle_maximum / 2,
                     oracle_maximum - 2, oracle_maximum - 1, oracle_maximum};
    for (int i = 0; i < 8; ++i) for (int variant = 0; variant < 8; ++variant) {
      draws[0] = choices[i]; draws[1] = choices[variant];
      uint32_t integers[] = {(uint32_t)oracle_maximum, draws[0], draws[1]};
      FP_TYPE mean = (variant - 4) * 0.25;
      FP_TYPE variance = variant == 0 ? 0 : variant == 1 ? 0.125 : variant == 2 ? 4 : 1;
      FP_TYPE amplitude = variant == 0 ? 0 : variant == 1 ? -2 : variant == 2 ? 1e-6 : 0.125;
      FP_TYPE inputs[] = {mean, variance, amplitude};
      consumed = 0; FP_TYPE uniform = randu(); if (consumed != 1) return 4;
      consumed = 0; FP_TYPE normal = randn(mean, variance); if (consumed != 2) return 4;
      consumed = 0; FP_TYPE* white = white_noise(amplitude, 2); if (!white || consumed != 2) return 4;
      FP_TYPE outputs[] = {uniform, normal, white[0], white[1]}; free(white);
      consumed = 0; white = white_noise(amplitude, 0); free(white); if (consumed != 0) return 4;
      fwrite(integers, sizeof(uint32_t), 3, file);
      fwrite(inputs, sizeof(FP_TYPE), 3, file); fwrite(outputs, sizeof(FP_TYPE), 4, file);
    }
  }
  return fclose(file) != 0 ? 3 : 0;
}
