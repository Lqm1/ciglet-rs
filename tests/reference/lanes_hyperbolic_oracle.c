#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

int main(void) {
  const v4sf inputs[] = {{-40,-4,-0.25f,-0.0f}, {0,1e-30f,-1e-30f,1e-4f},
    {-1e-4f,0.25f,1,4}, {-8,8,20,40}, {-1,-2,2,-4},
    {-0.6931472f,0.6931472f,-0.5f,0.5f}};
  v4sf (*functions[])(v4sf) = {vfastsinh,vfastersinh,vfastcosh,vfastercosh,
    vfasttanh,vfastertanh,vfastsigmoid,vfastersigmoid};
  const char *names[] = {"vfastsinh","vfastersinh","vfastcosh","vfastercosh",
    "vfasttanh","vfastertanh","vfastsigmoid","vfastersigmoid"};
  for (unsigned function=0;function<8;++function)
    for (unsigned group=0;group<6;++group) {
      v4sf result=functions[function](inputs[group]);
      uint32_t before[4],after[4];
      memcpy(before,&inputs[group],sizeof before);
      memcpy(after,&result,sizeof after);
      printf("%s",names[function]);
      for (unsigned lane=0;lane<4;++lane) printf(" %08x",before[lane]);
      for (unsigned lane=0;lane<4;++lane) printf(" %08x",after[lane]);
      puts("");
    }
  return 0;
}
