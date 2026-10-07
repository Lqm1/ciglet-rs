#include "ciglet.h"
#include <stdint.h>
#include <stdio.h>
int main(int argc, char** argv) {
  if (argc != 2) return 1;
  FILE* file = fopen(argv[1], "wb");
  if (!file) return 2;
  uint32_t width = sizeof(FP_TYPE), count = 64;
  fwrite("QIF1", 1, 4, file);
  fwrite(&width, 4, 1, file);
  fwrite(&count, 4, 1, file);
  for (int i = 0; i < 64; ++i) {
    FP_TYPE samples[3];
    for (int j = 0; j < 3; ++j)
      samples[j] = ((i * (j + 3) + j * 5) % 17 - 8) / 8.0;
    if (i == 0) samples[0] = samples[1] = samples[2] = 1;
    if (i == 1) { samples[0] = 0; samples[1] = 1; samples[2] = 0; }
    if (i == 2) { samples[0] = 0; samples[1] = 1; samples[2] = 1; }
    if (i == 3) { samples[0] = 1; samples[1] = 0; samples[2] = -1; }
    FP_TYPE position, magnitude = cig_qifft(samples, 1, &position);
    fwrite(samples, sizeof(FP_TYPE), 3, file);
    fwrite(&position, sizeof(FP_TYPE), 1, file);
    fwrite(&magnitude, sizeof(FP_TYPE), 1, file);
  }
  return fclose(file) != 0;
}
