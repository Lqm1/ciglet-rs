#include <float.h>
#include "ciglet.h"
int main(void){
 FP_TYPE largest=sizeof(FP_TYPE)==4?FLT_MAX:DBL_MAX;
 FP_TYPE tiny=sizeof(FP_TYPE)==4?nextafterf(0,1):nextafter(0,1);
 cplx large=c_cplx(largest,largest),small=c_cplx(tiny,tiny);
 cplx overflow=c_div(large,large),underflow=c_div(small,small);
 FP_TYPE numerator=sizeof(FP_TYPE)==4?1e-30:1e-250;
 FP_TYPE divisor=sizeof(FP_TYPE)==4?1e-20:1e-100;
 cplx lost=c_div(c_cplx(numerator,0),c_cplx(divisor,0));
 FP_TYPE angle=c_arg_1(c_cplx(tiny,largest));
 if(!isnan(overflow.real)||!isnan(overflow.imag)||!isnan(underflow.real)||!isnan(underflow.imag)||lost.real!=0||!isnan(angle))return 1;
 printf("%zu-byte source division overflow/underflow NaNs, lost finite quotient and fast-argument NaN confirmed\n",sizeof(FP_TYPE));
 return 0;
}
