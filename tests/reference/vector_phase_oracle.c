#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f,uint32_t n){if(fwrite(&n,4,1,f)!=1)exit(3);}
static void vector(FILE *f,FP_TYPE *x,int n){if(fwrite(x,sizeof(FP_TYPE),n,f)!=(size_t)n)exit(3);}
static void owned(FILE *f,FP_TYPE *x,int n){vector(f,x,n);free(x);}
int main(int argc,char **argv){
 if(argc!=2)return 2;FILE *f=fopen(argv[1],"wb");if(!f)return 3;
 fwrite("VPH1",1,4,f);integer(f,sizeof(FP_TYPE));integer(f,24);
 const int lengths[]={1,2,3,4,7,8,16,65};
 for(int size=0;size<8;size++)for(int shape=0;shape<3;shape++){
  int n=lengths[size];FP_TYPE real[65],imag[65],angles[65],copy[65],out[65];
  for(int i=0;i<n;i++){
   real[i]=shape==0?((i*7)%19-9)*0.25:shape==1?(i-3)*0.2:(i%3==0?-0.0:i%3==1?-2.0:2.0);
   imag[i]=shape==0?((i*11)%17-8)*0.125:shape==1?(3-i)*0.3:(i%2?0.0:-0.0);
   angles[i]=shape==2?(i-n/2)*M_PI*0.75:((i*5)%23-11)*0.4;
  }
  integer(f,n);vector(f,real,n);vector(f,imag,n);vector(f,angles,n);
  owned(f,diff(real,n),n);owned(f,cumsum(real,n),n);owned(f,flip(real,n),n);
  owned(f,unwrap(angles,n),n);
  for(int i=0;i<n;i++)out[i]=wrap(angles[i]);vector(f,out,n);
  for(int i=0;i<n;i++)out[i]=phase_diff(angles[i],angles[(i+3)%n]);vector(f,out,n);
  FP_TYPE *magnitude=abscplx(real,imag,n);vector(f,magnitude,n);
  FP_TYPE *argument=argcplx(real,imag,n);
  for(int i=0;i<n;i++){
   cplx point=c_cplx(real[i],imag[i]);FP_TYPE amplitude=c_abs(point),angle=c_arg(point),high=c_arg_3(point);
   if(memcmp(&amplitude,magnitude+i,sizeof(FP_TYPE)) || memcmp(&angle,argument+i,sizeof(FP_TYPE)) || memcmp(&high,argument+i,sizeof(FP_TYPE)))return 4;
  }
  owned(f,argument,n);owned(f,polar2real(magnitude,angles,n),n);owned(f,polar2imag(magnitude,angles,n),n);free(magnitude);
  memcpy(copy,real,n*sizeof(FP_TYPE));complete_symm(copy,n);vector(f,copy,n);
  memcpy(copy,real,n*sizeof(FP_TYPE));complete_asymm(copy,n);vector(f,copy,n);
 }
 integer(f,18);const int counts[]={2,3,4,7,8,16};
 for(int i=0;i<6;i++)for(int shape=0;shape<3;shape++){
  int n=counts[i];FP_TYPE a=shape==0?-1.25:shape==1?0.3:2.75,b=shape==0?3.5:shape==1?2.7:-3.25;
  integer(f,n);vector(f,&a,1);vector(f,&b,1);owned(f,linspace(a,b,n),n);
  FP_TYPE ratio=shape==0?-0.25:shape==1?0.625:1.25,value=linterp(a,b,ratio);vector(f,&ratio,1);vector(f,&value,1);
 }
 integer(f,9);
 for(int shape=0;shape<3;shape++)for(int n=1;n<=9;n+=4){int start=shape==0?-37:shape==1?19:0,step=shape==0?5:shape==1?-7:0;
  integer(f,n);integer(f,(uint32_t)start);integer(f,(uint32_t)step);int *x=iota(start,step,n);for(int i=0;i<n;i++)integer(f,(uint32_t)x[i]);free(x);
 }
 return fclose(f)!=0?3:0;
}
