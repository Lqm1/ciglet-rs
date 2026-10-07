#include <stdint.h>
#include "ciglet.h"
static void integer(FILE* file, uint32_t value) { fwrite(&value, 4, 1, file); }
static void value(FILE* file, FP_TYPE value) { fwrite(&value, sizeof(value), 1, file); }
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  fwrite("SCM1", 1, 4, file); integer(file, sizeof(FP_TYPE)); integer(file, 64);
  for (int i = 0; i < 64; ++i) {
    FP_TYPE angle = (i - 32) * 0.25, y = (i % 9 - 4) * 0.25, x = (i % 11 - 5) * 0.5;
    if (i == 0) { angle = -0.0; y = -0.0; x = 0.0; }
    if (i == 1) { angle = 0.0; y = 0.0; x = -0.0; }
    value(file, angle); value(file, y); value(file, x);
    value(file, sin_1(angle)); value(file, cos_1(angle));
    value(file, sin_2(angle)); value(file, cos_2(angle));
    value(file, sin_3(angle)); value(file, cos_3(angle));
    value(file, atan2_1(y, x)); value(file, atan2_2(y, x)); value(file, atan2_3(y, x));
  }
  integer(file, 16);
  FP_TYPE values[] = {0.0, -0.0, 1.0, -1.0, INFINITY, -INFINITY, 0, 0};
  if (sizeof(FP_TYPE) == 4) { uint32_t a = 0x7fc00042, b = 0xffc00099; memcpy(&values[6], &a, 4); memcpy(&values[7], &b, 4); }
  else { uint64_t a = UINT64_C(0x7ff8000000000042), b = UINT64_C(0xfff8000000000099); memcpy(&values[6], &a, 8); memcpy(&values[7], &b, 8); }
  int pairs[16][2] = {{0,1},{1,0},{2,2},{2,3},{3,2},{4,5},{5,4},{6,2},{2,6},{7,6},{6,7},{4,4},{5,5},{7,7},{0,2},{3,1}};
  for (int i = 0; i < 16; ++i) {
    FP_TYPE a = values[pairs[i][0]], b = values[pairs[i][1]];
    value(file, a); value(file, b); value(file, min(a, b)); value(file, max(a, b));
  }
  return fclose(file) != 0 ? 3 : 0;
}
