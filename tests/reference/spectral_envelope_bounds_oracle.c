#include "ciglet.h"
int main(int argc, char** argv) {
  (void)argv;
  FP_TYPE spectrum[9] = {0.5,1,2,3,4,5,6,7,8};
  FP_TYPE coefficients[9];
  FP_TYPE* output = argc > 1 ? spec2env(spectrum,16,1e-12,coefficients) : cig_spec2env(spectrum,16,0.125,2,coefficients);
  volatile FP_TYPE first = output[0]; (void)first;
  free(output);
  return 0;
}
