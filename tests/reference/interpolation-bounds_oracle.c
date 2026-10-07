#include "ciglet.h"
int main(void) {
 FP_TYPE values[2]={1,3}, position[1]={2};
 FP_TYPE *output=interp1u(0,2,values,2,position,1);
 printf("%g\n",(double)output[0]);free(output);return 0;
}
