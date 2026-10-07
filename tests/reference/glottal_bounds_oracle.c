#include <limits.h>
#include "ciglet.h"
int main(int argc,char** argv) {
  (void)argv;
  lfmodel model={.T0=2,.te=0.6,.tp=0.4,.ta=0.05,.Ee=1};
  if(argc>1) {
    model.T0=0.005;
    FP_TYPE frequency=0,phase;
    FP_TYPE* magnitude=lfmodel_spectrum(model,&frequency,1,&phase);
    free(magnitude);
  } else {
    FP_TYPE* wave=lfmodel_period(model,INT_MAX,1);
    free(wave);
  }
  return 0;
}
