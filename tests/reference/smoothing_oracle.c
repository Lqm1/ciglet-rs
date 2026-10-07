#include <stdint.h>
#include "ciglet.h"
static void number(FILE *file,uint32_t value) {if(fwrite(&value,4,1,file)!=1) exit(3);}
int main(int argc,char **argv) {
 if(argc!=2) return 2;FILE *file=fopen(argv[1],"wb");if(!file) return 3;
 fwrite("SMO1",1,4,file);number(file,sizeof(FP_TYPE));number(file,60);
 const int lengths[]={9,16,32};
 const FP_TYPE widths[]={0.75,1,1.5,3,7.25};
 for(int length=0;length<3;length++) for(int shape=0;shape<4;shape++) for(int radius=0;radius<5;radius++) {
  int n=lengths[length];FP_TYPE signal[32],width=widths[radius];
  for(int i=0;i<n;i++) signal[i]=shape==0?2.25:shape==1?(i%5-2)*0.25:shape==2?(i==n/2?1:0):i*i*0.03125-i*0.25;
  FP_TYPE *mean=moving_avg(signal,n,width);
  FP_TYPE *rms=moving_rms(signal,n,(int)width+1);
  number(file,n);fwrite(&width,sizeof(FP_TYPE),1,file);fwrite(signal,sizeof(FP_TYPE),n,file);
  fwrite(mean,sizeof(FP_TYPE),n,file);fwrite(rms,sizeof(FP_TYPE),n,file);free(mean);free(rms);
 }
 return fclose(file)!=0?3:0;
}
