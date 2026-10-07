#include <stdint.h>
#include "ciglet.h"
static void integer(FILE* file,uint32_t x) {fwrite(&x,4,1,file);}
int main(int argc,char** argv) {
  if(argc != 2)return 2;
  FILE* file = fopen(argv[1],"wb"); if(!file)return 3;
  fwrite("ENV1",1,4,file); integer(file,sizeof(FP_TYPE)); integer(file,144);
  int sizes[] = {8,16,32,64,128,256};
  FP_TYPE fundamentals[] = {0.0625,0.125,0.25};
  for(int size=0;size<6;++size)for(int shape=0;shape<4;++shape)for(int pitch=0;pitch<3;++pitch)for(int mode=0;mode<2;++mode) {
    int n=sizes[size],bins=n/2+1; FP_TYPE spectrum[129],coefficients[129];
    for(int i=0;i<bins;++i)spectrum[i]=shape==0?0:shape==1?0.5:shape==2?0.25+(i%7)*0.125:(i==bins/2?2.0:0.125);
    integer(file,n);integer(file,mode==0?5:0);fwrite(&fundamentals[pitch],sizeof(FP_TYPE),1,file);fwrite(spectrum,sizeof(FP_TYPE),bins,file);
    FP_TYPE* envelope=mode==0?cig_spec2env(spectrum,n,fundamentals[pitch],5,coefficients):spec2env(spectrum,n,fundamentals[pitch],coefficients);
    fwrite(envelope,sizeof(FP_TYPE),n,file);fwrite(coefficients,sizeof(FP_TYPE),bins,file);
    free(envelope);
  }
  return fclose(file)!=0?3:0;
}
