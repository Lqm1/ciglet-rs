#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

static uint32_t bits(float value) {
  uint32_t result;
  memcpy(&result, &value, sizeof result);
  return result;
}

int main(void) {
  const uint32_t inputs[] = {
    0, 0x80000000, 0x0da24260, 0x8da24260, 0x38d1b717, 0xb8d1b717,
    0x3e800000, 0xbe800000, 0x3f800000, 0xbf800000,
    0x3fc90fda, 0x3fc90fdb, 0x3fc90fdc,
    0xbfc90fda, 0xbfc90fdb, 0xbfc90fdc,
    0x40490fda, 0x40490fdb, 0x40490fdc,
    0xc0490fda, 0xc0490fdb, 0xc0490fdc,
    0x40c90fda, 0x40c90fdb, 0x40c90fdc,
    0xc0c90fda, 0xc0c90fdb, 0xc0c90fdc,
    0x41200000, 0xc1200000, 0x42c80000, 0xc2c80000
  };
  float (*functions[])(float) = {fastsin, fastersin, fastcos, fastercos,
    fasttan, fastertan, fasttanfull, fastertanfull,
    fastsinfull, fastersinfull, fastcosfull, fastercosfull};
  const char *names[] = {"fastsin", "fastersin", "fastcos", "fastercos",
    "fasttan", "fastertan", "fasttanfull", "fastertanfull",
    "fastsinfull", "fastersinfull", "fastcosfull", "fastercosfull"};
  for (unsigned function = 0; function < 12; ++function)
    for (unsigned input = 0; input < sizeof inputs / sizeof *inputs; ++input) {
      float value;
      memcpy(&value, &inputs[input], sizeof value);
      printf("%s %08x %08x\n", names[function], inputs[input], bits(functions[function](value)));
    }
  const float bases[] = {0.125f, 0.5f, 1, 2, 10, 100};
  const float exponents[] = {-3, -0.5f, 0, 0.5f, 2, 3};
  for (unsigned base = 0; base < 6; ++base)
    for (unsigned exponent = 0; exponent < 6; ++exponent) {
      printf("fastpow %08x %08x %08x\n", bits(bases[base]), bits(exponents[exponent]),
        bits(fastpow(bases[base], exponents[exponent])));
      printf("fasterpow %08x %08x %08x\n", bits(bases[base]), bits(exponents[exponent]),
        bits(fasterpow(bases[base], exponents[exponent])));
    }
  return 0;
}
