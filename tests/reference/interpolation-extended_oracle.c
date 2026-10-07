#include <stdint.h>
#include "ciglet.h"
static void number(FILE *file,uint32_t value) { if(fwrite(&value,4,1,file)!=1) exit(3); }
int main(int argc,char **argv) {
 if(argc!=2) return 2;
 FILE *file=fopen(argv[1],"wb");if(!file) return 3;
 fwrite("EXT1",1,4,file);number(file,sizeof(FP_TYPE));number(file,28);
 const int sizes[]={2,3,9,32};
 for(int k=0;k<4;k++) for(int shape=0;shape<3;shape++) {
  int n=sizes[k];FP_TYPE values[32],positions[21],start=-2.5,end=11.75;
  for(int i=0;i<n;i++) values[i]=shape==0?2.25:shape==1?(i%3-1)*0.75:i*i*0.125-i*0.5;
  for(int i=0;i<21;i++) positions[i]=start+(end-start)*(i-3)/12.0;
  FP_TYPE *output=sincinterp1u(start,end,values,n,positions,21);
  number(file,0);number(file,n);number(file,21);
  fwrite(&start,sizeof(FP_TYPE),1,file);fwrite(&end,sizeof(FP_TYPE),1,file);
  fwrite(values,sizeof(FP_TYPE),n,file);fwrite(positions,sizeof(FP_TYPE),21,file);fwrite(output,sizeof(FP_TYPE),21,file);free(output);
 }
 for(int k=0;k<4;k++) for(int shape=0;shape<4;shape++) {
  int n=sizes[k];FP_TYPE values[32],blank=shape%2?NAN:-99,unused=0;
  for(int i=0;i<n;i++) {
   int known=shape==0?0:shape==1?i%2:shape==2?i==n/2:1;
   values[i]=known?i*i*0.03125-i*0.25:blank;
  }
  FP_TYPE *output=interp_in_blank(values,n,blank);
  number(file,1);number(file,n);number(file,n);
  fwrite(&blank,sizeof(FP_TYPE),1,file);fwrite(&unused,sizeof(FP_TYPE),1,file);
  fwrite(values,sizeof(FP_TYPE),n,file);fwrite(output,sizeof(FP_TYPE),n,file);free(output);
 }
 return fclose(file)!=0?3:0;
}
