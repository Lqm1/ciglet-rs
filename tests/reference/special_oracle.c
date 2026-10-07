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
  const float inputs[] = {-30, -8, -4, -2, -1, -0.5f, -0.1f, -1e-4f,
    -1e-30f, -0.0f, 0.0f, 1e-30f, 1e-4f, 0.1f, 0.5f, 1, 2, 4, 8, 30};
  const float inverse_inputs[] = {-0x1.fffffep-1f, -0.999f, -0.9f, -0.5f,
    -0.01f, -1e-30f, -0.0f, 0.0f, 1e-30f, 0.01f, 0.5f, 0.9f, 0.999f,
    0x1.fffffep-1f};
  float (*functions[])(float) = {fasterfc, fastererfc, fasterf, fastererf,
    fastinverseerf, fasterinverseerf};
  const char *names[] = {"fasterfc", "fastererfc", "fasterf", "fastererf",
    "fastinverseerf", "fasterinverseerf"};
  for (unsigned function = 0; function < 6; ++function) {
    const float *values = function < 4 ? inputs : inverse_inputs;
    unsigned count = function < 4 ? 20 : 14;
    for (unsigned input = 0; input < count; ++input)
      printf("%s %08x %08x\n", names[function], bits(values[input]),
        bits(functions[function](values[input])));
  }
  const float gamma_inputs[] = {0.0001f, 0.01f, 0.1f, 0.5f, 1, 2, 3, 5,
    10, 100, 1000, 1e6f};
  float (*gamma_functions[])(float) = {fastlgamma, fasterlgamma, fastdigamma, fasterdigamma};
  const char *gamma_names[] = {"fastlgamma", "fasterlgamma", "fastdigamma", "fasterdigamma"};
  for (unsigned function = 0; function < 4; ++function)
    for (unsigned input = 0; input < 12; ++input)
      printf("%s %08x %08x\n", gamma_names[function], bits(gamma_inputs[input]),
        bits(gamma_functions[function](gamma_inputs[input])));
  const float negative_inputs[] = {-1.75f, -1.5f, -1.25f, -0.75f, -0.5f, -0.25f};
  for (unsigned input = 0; input < 6; ++input) {
    float value = negative_inputs[input];
    printf("fastdigamma %08x %08x\n", bits(value), bits(fastdigamma(value)));
    if (input < 3)
      printf("fastlgamma %08x %08x\n", bits(value), bits(fastlgamma(value)));
    else
      printf("fasterdigamma %08x %08x\n", bits(value), bits(fasterdigamma(value)));
  }
  union { uint32_t bits; float value; } threshold = { .value = 2.26445f };
  union { uint32_t bits; float value; } below = { .bits = threshold.bits - 1 };
  union { uint32_t bits; float value; } above = { .bits = threshold.bits + 1 };
  const float lambert_inputs[] = {-0.36f, -0.25f, -0.1f, -0.0f, 0.0f, 0.1f,
    1, below.value, threshold.value, above.value, 10, 100};
  float (*lambert_functions[])(float) = {fastlambertw, fasterlambertw};
  const char *lambert_names[] = {"fastlambertw", "fasterlambertw"};
  for (unsigned function = 0; function < 2; ++function)
    for (unsigned input = 0; input < 12; ++input)
      printf("%s %08x %08x\n", lambert_names[function], bits(lambert_inputs[input]),
        bits(lambert_functions[function](lambert_inputs[input])));
  threshold.value = 1.1765631309f;
  below.bits = threshold.bits - 1;
  above.bits = threshold.bits + 1;
  const float exp_inputs[] = {-100, -40, -8, -4, -1, -0.1f, -1e-30f, -0.0f,
    0.0f, 1e-30f, 0.1f, 1, below.value, threshold.value, above.value, 2, 4, 8, 40, 100};
  float (*exp_functions[])(float) = {fastlambertwexpx, fasterlambertwexpx};
  const char *exp_names[] = {"fastlambertwexpx", "fasterlambertwexpx"};
  for (unsigned function = 0; function < 2; ++function)
    for (unsigned input = 0; input < 20; ++input)
      printf("%s %08x %08x\n", exp_names[function], bits(exp_inputs[input]),
        bits(exp_functions[function](exp_inputs[input])));
  return 0;
}
