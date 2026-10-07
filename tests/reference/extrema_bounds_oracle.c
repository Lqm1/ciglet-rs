#include "ciglet.h"
int main(void) {
  FP_TYPE signal[1] = {1};
  printf("%d\n", find_peak(signal, 0, 0));
  return 0;
}
