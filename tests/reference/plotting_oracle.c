#define _POSIX_C_SOURCE 200809L
#include <float.h>
#include <math.h>
#include "ciglet.h"

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  figure fg = {file};
  FP_TYPE maximum = sizeof(FP_TYPE) == 4 ? FLT_MAX : DBL_MAX;
  FP_TYPE tiny = sizeof(FP_TYPE) == 4 ? FLT_TRUE_MIN : DBL_TRUE_MIN;
  FP_TYPE y[] = {0, -0.0, 1.23456789, -0.0000005, 1e20, 1e-20,
                 maximum, tiny, NAN, -NAN, INFINITY, -INFINITY};
  FP_TYPE x[12]; for (int i = 0; i < 12; ++i) x[i] = i * 0.25 - 1;
  char colors[] = {'b', 'r', 'g', 'k', 'c', 'y', '?'};
  for (int i = 0; i < 7; ++i) {
    plot(&fg, i % 2 ? x : NULL, y, 12, colors[i]);
  }
  plot(&fg, NULL, y, 0, 'b');
  FP_TYPE* rows[] = {y, y + 6};
  imagesc(&fg, rows, 2, 6);
  imagesc(&fg, rows, 0, 3);
  imagesc(&fg, rows, 2, 0);
  return fclose(file) != 0 ? 3 : 0;
}
