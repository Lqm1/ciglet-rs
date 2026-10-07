#include <stdint.h>
#include "ciglet.h"
static void number(FILE *f,uint32_t value) {if(fwrite(&value,4,1,f)!=1) exit(3);}
int main(int argc,char **argv) {
 if(argc!=2) return 2;FILE *f=fopen(argv[1],"wb");if(!f) return 3;
 fwrite("EXT2",1,4,f);number(f,sizeof(FP_TYPE));number(f,180);
 const int lengths[]={9,32,65};
 for(int k=0;k<3;k++) for(int shape=0;shape<5;shape++) for(int a=0;a<3;a++) for(int b=0;b<2;b++) for(int d=0;d<2;d++) {
  int n=lengths[k],start=a==0?1:a==1?n/4:n/2,end=b==0?n/2:n-2;
  int orient=d==0?1:-1;FP_TYPE signal[65];
  for(int i=0;i<n;i++) signal[i]=shape==0?2.25:shape==1?i*0.125:shape==2?-i*0.25:shape==3?((i*13)%19-9)*0.25:(i%5==2?4:0);
  number(f,n);number(f,start);number(f,end);number(f,d);
  fwrite(signal,sizeof(FP_TYPE),n,f);
  number(f,cig_find_peak(signal,start,end,orient));number(f,cig_find_extrema(signal,start,end,orient));
 }
 return fclose(f)!=0?3:0;
}
