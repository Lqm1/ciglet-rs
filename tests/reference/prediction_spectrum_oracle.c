#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file,uint32_t value){if(fwrite(&value,4,1,file)!=1)exit(3);}
static void values(FILE *file,FP_TYPE *value,int n){if(fwrite(value,sizeof(FP_TYPE),n,file)!=(size_t)n)exit(3);}
int main(int argc,char **argv){
 if(argc!=2)return 2;FILE *file=fopen(argv[1],"wb");if(!file)return 3;
 fwrite("LPS1",1,4,file);integer(file,sizeof(FP_TYPE));integer(file,48);
 const int sizes[]={3,5,7,9,17,33};
 for(int size=0;size<6;size++)for(int shape=0;shape<4;shape++)for(int high=0;high<2;high++){
  int ns=sizes[size],nfft=pow(2,ceil(log2(ns-1)+1)),order=high?5:1;if(order>=nfft)order=nfft-1;
  FP_TYPE input[33];for(int i=0;i<ns;i++)input[i]=shape==0?1.0:shape==1?0.5+(i%7)*0.125:shape==2?(i==ns/2?2.0:0.5):(i%2?0.75:1.5);
  FP_TYPE *correlation=NULL,*coefficients=flpc(input,ns,order,&correlation),gain=lpgain(coefficients,correlation,order+1),*spectrum=lpspec(coefficients,gain,order+1,nfft);
  if(!isfinite(gain))return 4;
  integer(file,ns);integer(file,nfft);integer(file,order);values(file,input,ns);values(file,correlation,order+1);values(file,coefficients,order+1);values(file,&gain,1);values(file,spectrum,nfft);
  free(correlation);free(coefficients);free(spectrum);
 }
 integer(file,11);
 const int lengths[]={2,3,3,5,3,4,2,3,3,3,3};
 FP_TYPE polynomials[11][5]={{1,-0.5},{1,-1,0.5},{1,1,0.5},{1,0,0,0,0.25},{2,-2,1},{1,-0.5,0.5,-0.25},{1,0.5},{1,-1.99999,1},{1,-1.999995,1},{1,1.99999,1},{1,1.999995,1}};
 for(int i=0;i<11;i++){
  int n=lengths[i],count=0;cplx poles[4];FP_TYPE *frequencies=lpresf(polynomials[i],n,poles,&count);
  integer(file,n);values(file,polynomials[i],n);integer(file,count);values(file,frequencies,count);
  for(int j=0;j<count;j++){values(file,&poles[j].real,1);values(file,&poles[j].imag,1);}free(frequencies);
 }
 return fclose(file)!=0?3:0;
}
