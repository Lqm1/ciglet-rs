#include "ciglet.h"
int main(void) {
  FP_TYPE** periods = malloc2d(1,2,sizeof(FP_TYPE));
  periods[0][0] = 1; periods[0][1] = 2;
  FP_TYPE frequencies[] = {1,2,3,4,5,6,7,8};
  FP_TYPE** mapped = cig_invcrgm(periods,1,2,16,frequencies,8);
  volatile FP_TYPE last = mapped[0][7]; (void)last;
  free2d(mapped,1); free2d(periods,1);
  return 0;
}
