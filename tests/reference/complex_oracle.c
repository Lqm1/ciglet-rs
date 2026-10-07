#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file,uint32_t value){if(fwrite(&value,4,1,file)!=1)exit(3);}
static void scalar(FILE *file,FP_TYPE value){if(fwrite(&value,sizeof(value),1,file)!=1)exit(3);}
static void point(FILE *file,cplx value){scalar(file,value.real);scalar(file,value.imag);}
int main(int argc,char **argv){
 if(argc!=2)return 2;FILE *file=fopen(argv[1],"wb");if(!file)return 3;
 fwrite("CPX1",1,4,file);integer(file,sizeof(FP_TYPE));integer(file,64);
 const double reals[]={-3,-1,-0.0,0.0,0.2,0.75,1.5,3};
 const double angles[]={-9,-3.14,-1,-0.0,0.0,0.25,1,5};
 for(int i=0;i<64;i++){
  cplx a=c_cplx(reals[i/8],angles[i%8]),b=c_cplx((i%7-3)*0.375+0.125,(i%5-2)*0.2);
  point(file,a);point(file,b);
  point(file,c_conj(a));point(file,c_add(a,b));point(file,c_sub(a,b));point(file,c_mul(a,b));point(file,c_div(a,b));
  point(file,c_exp_1(a));point(file,c_exp_2(a));point(file,c_exp_3(a));
  scalar(file,c_arg_1(a));scalar(file,c_arg_2(a));
 }
 integer(file,15);
 const double values[]={0x1p-149,0x1p-126,1e-30,1e-10,0.01,0.125,0.5,1,1.5,2,10,100,1e10,1e20,1e30};
 const double exponents[]={-200,-127,-126,-100,-50,-10,-3,-1,-0.0,0.0,0.1,1,10,50,80};
 for(int i=0;i<15;i++){
  FP_TYPE value=values[i],exponent=exponents[i];scalar(file,value);scalar(file,exponent);
  scalar(file,log_1(value));scalar(file,log_2(value));scalar(file,log_3(value));
  scalar(file,exp_1(exponent));scalar(file,exp_2(exponent));scalar(file,exp_3(exponent));
  scalar(file,fasterlog2(value));scalar(file,fasterpow2(exponent));
 }
 return fclose(file)!=0?3:0;
}
