#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f,uint32_t n) {if(fwrite(&n,4,1,f)!=1)exit(3);}
static void vector(FILE *f,FP_TYPE *x,int n) {if(fwrite(x,sizeof(FP_TYPE),n,f)!=(size_t)n)exit(3);}
int main(int argc,char **argv) {
 if(argc!=2)return 2;FILE *f=fopen(argv[1],"wb");if(!f)return 3;
 fwrite("STA1",1,4,f);integer(f,sizeof(FP_TYPE));integer(f,32);
 const int lengths[]={1,2,3,4,7,8,16,65};
 for(int size=0;size<8;size++)for(int shape=0;shape<4;shape++) {
  int n=lengths[size],lags=n+3;FP_TYPE x[65],y[65],copy[65];
  for(int i=0;i<n;i++) {
   x[i]=shape==0?((i*13)%31-15)*0.125:shape==1?(n-1-i)*0.0625:shape==2?((i*5)%7-3)*0.125:1000+((i*7)%19)*0.015625;
   y[i]=shape==3?2000+((i*11)%17)*0.03125:((i*7)%23-11)*0.0625;
  }
  integer(f,n);integer(f,lags);vector(f,x,n);vector(f,y,n);
  FP_TYPE results[]={sumfp(x,n),sumsqrfp(x,n),meanfp(x,n),varfp(x,n),minfp(x,n),maxfp(x,n),medianfp(x,n),dot(x,y,n),cov(x,y,n)};
  vector(f,results,9);FP_TYPE vx=varfp(x,n),vy=varfp(y,n),correlation=corr(x,y,n);
  double divisor=sqrt(vx*vy);int valid=vx>0&&vy>0&&isfinite(divisor)&&divisor>0&&isfinite(correlation);
  integer(f,valid);if(!valid)correlation=0;vector(f,&correlation,1);
  for(int k=0;k<n;k++){memcpy(copy,x,n*sizeof(FP_TYPE));FP_TYPE selected=selectnth(copy,n,k);vector(f,&selected,1);}
  int indices[65];FP_TYPE *sorted=sort(x,n,indices);vector(f,sorted,n);for(int i=0;i<n;i++)integer(f,indices[i]);free(sorted);
  FP_TYPE *r=cig_xcorr(x,y,n,lags);vector(f,r,lags);free(r);
 }
 return fclose(f)!=0?3:0;
}
