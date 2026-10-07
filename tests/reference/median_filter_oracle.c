#include <stdint.h>
#include "ciglet.h"
static void number(FILE *f,uint32_t value) { if(fwrite(&value,4,1,f)!=1) exit(3); }
int main(int argc,char **argv) {
 if(argc!=2) return 2;FILE *f=fopen(argv[1],"wb");if(!f) return 3;
 fwrite("MED1",1,4,f);number(f,sizeof(FP_TYPE));number(f,96);
 const int lengths[]={25,32,65};const int widths[]={1,2,3,4,5,7,9,25};
 for(int k=0;k<3;k++) for(int shape=0;shape<4;shape++) for(int w=0;w<8;w++) {
  int n=lengths[k];FP_TYPE signal[65];
  for(int i=0;i<n;i++) signal[i]=shape==0?2.25:shape==1?(i%7-3)*0.125:shape==2?(i==n/2?9:0):((i*13)%19-9)*0.25;
  FP_TYPE *out=medfilt1(signal,n,widths[w]);
  number(f,n);number(f,widths[w]);fwrite(signal,sizeof(FP_TYPE),n,f);fwrite(out,sizeof(FP_TYPE),n,f);free(out);
 }
 return fclose(f)!=0?3:0;
}
