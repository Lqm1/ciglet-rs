#include <stdint.h>
#include "ciglet.h"
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  uint32_t width = sizeof(FP_TYPE), count = 24;
  fwrite("IFD1", 1, 4, file); fwrite(&width, 4, 1, file); fwrite(&count, 4, 1, file);
  FP_TYPE centers[] = {0, 0.0625, 0.2, -0.125};
  FP_TYPE resolutions[] = {0.5, 0.3, 0.125, 0.06, 0.025, 0.015625};
  for (int c = 0; c < 4; ++c) for (int r = 0; r < 6; ++r) {
    FP_TYPE center = centers[c], resolution = resolutions[r];
    ifdetector* detector = cig_create_ifdetector(center, resolution);
    uint32_t n = detector->nh;
    fwrite(&center, width, 1, file); fwrite(&resolution, width, 1, file); fwrite(&n, 4, 1, file);
    fwrite(detector->hr, width, n, file); fwrite(detector->hi, width, n, file);
    fwrite(detector->hdr, width, n, file); fwrite(detector->hdi, width, n, file);
    for (int shape = 0; shape < 4; ++shape) {
      uint32_t nx = shape == 0 ? n - 1 : n + shape;
      FP_TYPE* samples = calloc(nx, width);
      for (uint32_t i = 0; i < nx; ++i) samples[i] = ((int)(i % 13) - 6) * 0.125 + (shape == 3 ? 0.25 : 0);
      FP_TYPE estimate = cig_ifdetector_estimate(detector, samples, nx);
      fwrite(&nx, 4, 1, file); fwrite(samples, width, nx, file); fwrite(&estimate, width, 1, file);
      free(samples);
    }
    cig_delete_ifdetector(detector);
  }
  return fclose(file) != 0 ? 3 : 0;
}
