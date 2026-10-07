#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file,uint32_t value){if(fwrite(&value,4,1,file)!=1)exit(3);}
static void vector(FILE *file,FP_TYPE *values,int n){if(fwrite(values,sizeof(FP_TYPE),n,file)!=(size_t)n)exit(3);}
int main(int argc,char **argv){
 if(argc!=2)return 2;FILE *file=fopen(argv[1],"wb");if(!file)return 3;
 fwrite("LPC1",1,4,file);integer(file,sizeof(FP_TYPE));integer(file,64);
 const int lengths[]={2,3,4,7,8,16,32,65};
 for(int size=0;size<8;size++)for(int shape=0;shape<4;shape++)for(int high=0;high<2;high++){
  int nx=lengths[size],order=high?nx-1:1;FP_TYPE samples[65];
  for(int i=0;i<nx;i++)samples[i]=shape==0?((i*7)%19-9)*0.125:shape==1?(i==nx/2?2.0:0.0):shape==2?0.75:(i%2?-0.5:1.0);
  FP_TYPE *autocorrelation=NULL,*coefficients=lpc(samples,nx,order,&autocorrelation);
  FP_TYPE gain=lpgain(coefficients,autocorrelation,order+1);
  if(!isfinite(gain))return 4;
  integer(file,nx);integer(file,order);vector(file,samples,nx);vector(file,autocorrelation,order+1);vector(file,coefficients,order+1);vector(file,&gain,1);
  free(autocorrelation);free(coefficients);
 }
 return fclose(file)!=0?3:0;
}
