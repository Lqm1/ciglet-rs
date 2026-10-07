#include "ciglet.h"
int main(void) {
 FP_TYPE signal[1]={1};FP_TYPE *output=medfilt1(signal,1,3);
 printf("%g\n",(double)output[0]);free(output);return 0;
}
