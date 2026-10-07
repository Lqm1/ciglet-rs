#include <stdio.h>
#include "external/fastapprox-all.h"

int main(void) {
  const float inputs[] = {-40, -8, -4, -1, -0.25f, -1e-4f, -1e-30f, -0.0f,
                          0.0f, 1e-30f, 1e-4f, 0.25f, 1, 4, 8, 40};
  float (*functions[])(float) = {fastsinh, fastersinh, fastcosh, fastercosh,
                               fasttanh, fastertanh, fastsigmoid, fastersigmoid};
  const char* names[] = {"fastsinh", "fastersinh", "fastcosh", "fastercosh",
                        "fasttanh", "fastertanh", "fastsigmoid", "fastersigmoid"};
  for (int function = 0; function < 8; ++function)
    for (int input = 0; input < 16; ++input)
      printf("%s %.9g %.9g\n", names[function], inputs[input], functions[function](inputs[input]));
  return 0;
}
