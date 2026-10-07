#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

static uint32_t bits(float value) {
  if (!isfinite(value)) return 0x7fc00000; /* Only classification is compared. */
  uint32_t result;
  memcpy(&result, &value, sizeof result);
  return result;
}

int main(void) {
  unsigned pattern;
  while (scanf("%x", &pattern) == 1) {
    float value;
    memcpy(&value, &pattern, sizeof value);
    printf("%08x %08x %08x\n", pattern,
      bits(fastlambertwexpx(value)), bits(fasterlambertwexpx(value)));
  }
  return 0;
}
