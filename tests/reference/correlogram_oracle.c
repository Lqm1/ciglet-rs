#include <stdint.h>
#include "ciglet.h"
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  uint32_t width = sizeof(FP_TYPE), cases = 32;
  fwrite("CRG1", 1, 4, file); fwrite(&width, 4, 1, file); fwrite(&cases, 4, 1, file);
  int centers[] = {-7,0,5,19,31,47,63,80};
  int windows[] = {3,4,7,8,15,16,2,5};
  FP_TYPE frequencies[] = {0,250,500,800,1600,3200,8000,16000};
  for (int shape = 0; shape < 8; ++shape) for (int method = 0; method < 4; ++method) {
    uint32_t nx = 64, frames = 8, periods = 24, m = method, nf = 8;
    FP_TYPE samples[64];
    for (int i = 0; i < 64; ++i) samples[i] = shape == 0 ? 0 : shape == 1 ? 0.5 : shape == 2 ? i*0.0625 : shape == 3 ? (i%2 ? -0.5 : 0.5) : shape == 4 ? ((i%7)-3)*0.25 : shape == 5 ? (i == 19 ? 2 : 0) : shape == 6 ? 10 + ((i%11)-5)*0.125 : ((i*17%23)-11)*0.0625;
    fwrite(&nx,4,1,file); fwrite(&frames,4,1,file); fwrite(&periods,4,1,file); fwrite(&m,4,1,file);
    fwrite(samples,width,nx,file); fwrite(centers,4,frames,file); fwrite(windows,4,frames,file);
    FP_TYPE** result = malloc2d(frames, periods, width);
    cig_correlogram(samples,nx,centers,windows,frames,periods,method,result);
    for (int row = 0; row < frames; ++row) fwrite(result[row],width,periods,file);
    fwrite(&nf,4,1,file); fwrite(frequencies,width,nf,file);
    FP_TYPE** mapped = cig_invcrgm(result,frames,periods,16000,frequencies,nf);
    for (int row = 0; row < frames; ++row) fwrite(mapped[row],width,nf,file);
    free2d(mapped,frames); free2d(result,frames);
  }
  return fclose(file) != 0 ? 3 : 0;
}
