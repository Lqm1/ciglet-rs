#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

static void record(const char *name,v4sf input,v4sf output) {
  uint32_t before[4],after[4];
  memcpy(before,&input,sizeof before);memcpy(after,&output,sizeof after);
  printf("%s",name);
  for(unsigned lane=0;lane<4;++lane)printf(" %08x",before[lane]);
  for(unsigned lane=0;lane<4;++lane) {
    float value;memcpy(&value,&after[lane],sizeof value);
    printf(" %08x",isnan(value)?0x7fc00000:after[lane]);
  }
  puts("");
}

int main(void) {
  const v4sf error_inputs[]={{-30,-8,-4,-2},{-1,-0.5f,-0.1f,-1e-4f},
    {-1e-30f,-0.0f,0,1e-30f},{1e-4f,0.1f,0.5f,1},{2,4,8,30},{-0.75f,0.75f,-0.25f,0.25f}};
  v4sf (*errors[])(v4sf)={vfasterfc,vfastererfc,vfasterf,vfastererf};
  const char *error_names[]={"vfasterfc","vfastererfc","vfasterf","vfastererf"};
  for(unsigned function=0;function<4;++function)
    for(unsigned group=0;group<6;++group)record(error_names[function],error_inputs[group],errors[function](error_inputs[group]));
  const v4sf inverse_inputs[]={{-0x1.fffffep-1f,-0.999f,-0.9f,-0.5f},
    {-0.01f,-1e-30f,-0.0f,0},{1e-30f,0.01f,0.5f,0.9f},{0.999f,0x1.fffffep-1f,-0.25f,0.25f}};
  for(unsigned group=0;group<4;++group) {
    record("vfastinverseerf",inverse_inputs[group],vfastinverseerf(inverse_inputs[group]));
    record("vfasterinverseerf",inverse_inputs[group],vfasterinverseerf(inverse_inputs[group]));
  }
  const v4sf gamma_inputs[]={{0.0001f,0.01f,0.1f,0.5f},{1,2,3,5},{10,100,1000,1e6f}};
  v4sf (*gammas[])(v4sf)={vfastlgamma,vfasterlgamma,vfastdigamma,vfasterdigamma};
  const char *gamma_names[]={"vfastlgamma","vfasterlgamma","vfastdigamma","vfasterdigamma"};
  for(unsigned function=0;function<4;++function)
    for(unsigned group=0;group<3;++group)record(gamma_names[function],gamma_inputs[group],gammas[function](gamma_inputs[group]));
  const v4sf negative_gamma={-1.75f,-1.5f,-1.25f,1};
  const v4sf negative_digamma={-1.75f,-1.5f,-0.75f,-0.25f};
  const v4sf negative_fast={-0.75f,-0.5f,-0.25f,1};
  record("vfastlgamma",negative_gamma,vfastlgamma(negative_gamma));
  record("vfastdigamma",negative_digamma,vfastdigamma(negative_digamma));
  record("vfasterdigamma",negative_fast,vfasterdigamma(negative_fast));
  union {float value;uint32_t bits;} threshold={.value=2.26445f},below={.bits=threshold.bits-1},above={.bits=threshold.bits+1};
  const v4sf lambert_inputs[]={{-0.36f,-0.25f,-0.1f,-0.0f},{0,0.1f,1,below.value},
    {threshold.value,above.value,10,100},{-0.36787942f,-0.3f,2,3}};
  for(unsigned group=0;group<4;++group) {
    record("vfastlambertw",lambert_inputs[group],vfastlambertw(lambert_inputs[group]));
    record("vfasterlambertw",lambert_inputs[group],vfasterlambertw(lambert_inputs[group]));
  }
  threshold.value=1.1765631309f;below.bits=threshold.bits-1;above.bits=threshold.bits+1;
  union {float value;uint32_t bits;} maximum={.bits=0x7f7fffff};
  const v4sf exp_inputs[]={{-100,-40,-8,-4},{-1,-0.1f,-1e-30f,-0.0f},
    {0,1e-30f,0.1f,1},{below.value,threshold.value,above.value,2},
    {4,8,40,100},{-1000,1e13f,1e20f,maximum.value}};
  for(unsigned group=0;group<6;++group) {
    record("vfastlambertwexpx",exp_inputs[group],vfastlambertwexpx(exp_inputs[group]));
    record("vfasterlambertwexpx",exp_inputs[group],vfasterlambertwexpx(exp_inputs[group]));
  }
  return 0;
}
