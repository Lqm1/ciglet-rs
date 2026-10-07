#include <stdint.h>
#include "ciglet.h"
static void integer(FILE* file,uint32_t x){fwrite(&x,4,1,file);}
static void record(FILE* file,lfmodel model,FP_TYPE shape,uint32_t rate,uint32_t n) {
  uint32_t nf=10; FP_TYPE frequencies[]={0,-100,50,100,200,400,800,1600,3200,8000};
  FP_TYPE input[]={shape,model.T0,model.Ee}; fwrite(input,sizeof(FP_TYPE),3,file);
  FP_TYPE fields[]={model.T0,model.te,model.tp,model.ta,model.Ee};fwrite(fields,sizeof(FP_TYPE),5,file);
  integer(file,rate);integer(file,n);integer(file,nf);fwrite(frequencies,sizeof(FP_TYPE),nf,file);
  FP_TYPE phase[10];FP_TYPE* magnitude=lfmodel_spectrum(model,frequencies,nf,phase);FP_TYPE* wave=lfmodel_period(model,rate,n);
  fwrite(magnitude,sizeof(FP_TYPE),nf,file);fwrite(phase,sizeof(FP_TYPE),nf,file);fwrite(wave,sizeof(FP_TYPE),n,file);
  free(magnitude);free(wave);
}
int main(int argc,char** argv){
  if(argc!=2)return 2;FILE* file=fopen(argv[1],"wb");if(!file)return 3;
  fwrite("LFM1",1,4,file);integer(file,sizeof(FP_TYPE));integer(file,53);
  FP_TYPE shapes[]={0.1,0.21,0.5,1,2,2.7,3,5};
  FP_TYPE periods[]={0.005,0.001,0.00125};FP_TYPE excitations[]={0.5,2};
  for(int s=0;s<8;++s)for(int p=0;p<3;++p)for(int a=0;a<2;++a){
    lfmodel model=lfmodel_from_rd(shapes[s],periods[p],excitations[a]);
    record(file,model,shapes[s],16000,128);
  }
  lfmodel manual[]={ {.T0=0.004,.te=0.65,.tp=0.4,.ta=0.05,.Ee=1}, {.T0=0.02,.te=0.35,.tp=0.2,.ta=0.04,.Ee=0.5}, {.T0=0.005,.te=0.6,.tp=0.4,.ta=0.05,.Ee=0}, {.T0=0.001,.te=0.6,.tp=0.4,.ta=0.05,.Ee=1} };
  uint32_t rates[]={8000,44100,16000,8000},lengths[]={40,900,80,0};
  for(int i=0;i<4;++i)record(file,manual[i],0,rates[i],lengths[i]);
  lfmodel inverted={.T0=0.005,.te=0.6,.tp=0.4,.ta=0.05,.Ee=-1};
  record(file,inverted,0,16000,80);
  return fclose(file)!=0?3:0;
}
