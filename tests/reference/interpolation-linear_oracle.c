#include <stdint.h>
#include "ciglet.h"
static void number(FILE *file, uint32_t value) { if(fwrite(&value,4,1,file)!=1) exit(3); }
int main(int argc,char **argv) {
 if(argc!=2) return 2;
 FILE *file=fopen(argv[1],"wb"); if(!file) return 3;
 fwrite("INT1",1,4,file);number(file,sizeof(FP_TYPE));number(file,24);
 const int counts[]={2,3,9,32};
 for(int k=0;k<4;k++) for(int shape=0;shape<3;shape++) for(int kind=0;kind<2;kind++) {
  int count=counts[k]; FP_TYPE coordinates[32],values[32],positions[21];
  FP_TYPE start=-2.5,end=11.75;
  for(int i=0;i<count;i++) {
   coordinates[i]=-2.5+i*i*0.125+i*0.5;
   values[i]=shape==0 ? 2.25 : shape==1 ? (i%3-1)*0.75 : i*i*0.125-i*0.5;
  }
  FP_TYPE minimum=kind==0?coordinates[0]:start;
  FP_TYPE maximum=kind==0?coordinates[count-1]:end;
  for(int i=0;i<21;i++) positions[i]=minimum+(maximum-minimum)*(i-3)/12.0;
  FP_TYPE *output=kind==0?interp1(coordinates,values,count,positions,21):interp1u(start,end,values,count,positions,21);
  number(file,kind);number(file,count);number(file,21);
  fwrite(&start,sizeof(FP_TYPE),1,file);fwrite(&end,sizeof(FP_TYPE),1,file);
  fwrite(coordinates,sizeof(FP_TYPE),count,file);fwrite(values,sizeof(FP_TYPE),count,file);
  fwrite(positions,sizeof(FP_TYPE),21,file);fwrite(output,sizeof(FP_TYPE),21,file);
  free(output);
 }
 return fclose(file)!=0?3:0;
}
