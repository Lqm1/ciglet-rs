#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
static int first_draw;
static unsigned consumed;
static int oracle_rand(void) { return consumed++ == 0 ? first_draw : 0; }
#undef RAND_MAX
#define RAND_MAX 8
#define rand oracle_rand
#include "ciglet.h"

int main(void) {
  first_draw = 0; consumed = 0;
  FP_TYPE endpoint = randn(0, 1);
  if (consumed != 2 || !isfinite(endpoint) || endpoint < 10) return 2;
  first_draw = 1; consumed = 0;
  FP_TYPE variance = sizeof(FP_TYPE) == 4 ? FLT_MAX : DBL_MAX;
  FP_TYPE sample = randn(0, variance);
  if (consumed != 2) return 3;
  if (sizeof(FP_TYPE) == 4 ? !isfinite(sample) : !isinf(sample)) return 4;
  double stable = sqrt(-2.0 * log(0.125)) * sqrt((double)variance);
  if (!isfinite(stable)) return 5;
  printf("zero-draw artifact %.17g; source extreme %.17g; stable %.17g\n",
         (double)endpoint, (double)sample, stable);
  return 0;
}
