#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

int main(void) {
  const uint32_t inputs[] = {
    0,0x80000000,0x0da24260,0x8da24260,0x38d1b717,0xb8d1b717,
    0x3e800000,0xbe800000,0x3f800000,0xbf800000,
    0x3fc90fda,0x3fc90fdb,0x3fc90fdc,0xbfc90fda,0xbfc90fdb,0xbfc90fdc,
    0x40490fda,0x40490fdb,0x40490fdc,0xc0490fda,0xc0490fdb,0xc0490fdc,
    0x40c90fda,0x40c90fdb,0x40c90fdc,0xc0c90fda,0xc0c90fdb,0xc0c90fdc,
    0x41200000,0xc1200000,0x42c80000,0xc2c80000,
    0x447a0000,0xc47a0000,0x49742400,0xc9742400,0x4e6e6b28,0xce6e6b28,
    0x3f490fdb,0xbf490fdb,0x3fc90fd9,0xbfc90fd9,0x3fc90fdd,0xbfc90fdd,
    0x40490fd9,0xc0490fd9,0x40490fdd,0xc0490fdd
  };
  v4sf (*functions[])(v4sf) = {vfastsin,vfastersin,vfastcos,vfastercos,
    vfasttan,vfastertan,vfastsinfull,vfastersinfull,vfastcosfull,vfastercosfull,
    vfasttanfull,vfastertanfull};
  const char *names[] = {"vfastsin","vfastersin","vfastcos","vfastercos",
    "vfasttan","vfastertan","vfastsinfull","vfastersinfull",
    "vfastcosfull","vfastercosfull","vfasttanfull","vfastertanfull"};
  for(unsigned function=0;function<12;++function)
    for(unsigned group=0;group<12;++group) {
      v4sf input;
      memcpy(&input,inputs+group*4,sizeof input);
      v4sf result=functions[function](input);
      uint32_t output[4];memcpy(output,&result,sizeof output);
      printf("%s",names[function]);
      for(unsigned lane=0;lane<4;++lane)printf(" %08x",inputs[group*4+lane]);
      for(unsigned lane=0;lane<4;++lane) {
        float value;memcpy(&value,&output[lane],sizeof value);
        printf(" %08x",isnan(value)?0x7fc00000:output[lane]);
      }
      puts("");
    }
  return 0;
}
