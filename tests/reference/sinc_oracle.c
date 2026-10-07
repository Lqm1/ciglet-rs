#include <stdint.h>
#include "ciglet.h"
static void value(FILE* file, FP_TYPE value) { fwrite(&value, sizeof(value), 1, file); }
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  uint32_t width = sizeof(FP_TYPE), count = 192;
  fwrite("SNC1", 1, 4, file); fwrite(&width, 4, 1, file); fwrite(&count, 4, 1, file);
  FP_TYPE lengths[] = {-3, -2, 0, 0.5, 1, 2, 3, 17};
  double angles[] = {-8,-6.283185307179586,-3.141592653589793,-1,-0.1,-0.001,-0.00021,-0.0002,-0.000199,-0.000021,-0.00002,-0.000019,-1e-8,-0.0,0.0,1e-8,0.000019,0.00002,0.000021,0.000199,0.0002,0.00021,1,6.283185307179586};
  for (int m = 0; m < 8; ++m) for (int w = 0; w < 24; ++w) {
    FP_TYPE angle = angles[w];
    value(file, lengths[m]); value(file, angle);
    value(file, safe_aliased_sinc(lengths[m], angle));
    value(file, safe_aliased_dsinc(lengths[m], angle));
  }
  return fclose(file) != 0 ? 3 : 0;
}
