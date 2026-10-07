#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "external/fastapprox-all.h"

static void record(const char *name, v4sf input, v4sf output) {
  uint32_t before[4], after[4];
  memcpy(before, &input, sizeof before);
  memcpy(after, &output, sizeof after);
  printf("%s", name);
  for (unsigned lane = 0; lane < 4; ++lane) printf(" %08x", before[lane]);
  for (unsigned lane = 0; lane < 4; ++lane) printf(" %08x", after[lane]);
  puts("");
}

int main(void) {
  const float exponents[6][4] = {{-200,-126,-125.75f,-1}, {-0.1f,-0.0f,0,0.1f},
    {0.25f,0.5f,1,2}, {4,8,16,40}, {-80,-40,60,80}, {-125.5f,-3.5f,3.5f,20}};
  const uint32_t logarithms[6][4] = {{1,0x7fffff,0x800000,0x800001},
    {0x3dcccccd,0x3e000000,0x3f000000,0x3f800000},
    {0x3f7fffff,0x3f800001,0x40000000,0x41200000},
    {0x42c80000,0x447a0000,0x49742400,0x7f7fffff},
    {0x3f000001,0x3f7ffeff,0x40000001,0x40000002},
    {0x00800002,0x01000001,0x3fffffff,0x7f000001}};
  v4sf (*functions[])(v4sf) = {vfastpow2,vfasterpow2,vfastexp,vfasterexp,
    vfastlog2,vfasterlog2,vfastlog,vfasterlog};
  const char *names[] = {"vfastpow2","vfasterpow2","vfastexp","vfasterexp",
    "vfastlog2","vfasterlog2","vfastlog","vfasterlog"};
  for (unsigned function = 0; function < 8; ++function)
    for (unsigned group = 0; group < 6; ++group) {
      v4sf input;
      if (function < 4) memcpy(&input, exponents[group], sizeof input);
      else memcpy(&input, logarithms[group], sizeof input);
      record(names[function], input, functions[function](input));
    }
  const v4sf boundaries[] = {{-126.00001f,-126,-125.99999f,-0.0f},
    {127,127.5f,127.75f,127.99999f}};
  for (unsigned function=0;function<2;++function)
    for (unsigned group=0;group<2;++group)
      record(names[function],boundaries[group],functions[function](boundaries[group]));
  for (unsigned group = 0; group < 6; ++group) {
    const v4sf base = {0.125f,0.5f,2,10};
    const v4sf exponent = {-3+group*0.5f,-0.5f+group*0.25f,group*0.5f,3-group*0.5f};
    uint32_t base_bits[4], exponent_bits[4], output[4];
    memcpy(base_bits,&base,sizeof base_bits);
    memcpy(exponent_bits,&exponent,sizeof exponent_bits);
    for (unsigned fast = 0; fast < 2; ++fast) {
      v4sf result = fast ? vfasterpow(base,exponent) : vfastpow(base,exponent);
      memcpy(output,&result,sizeof output);
      printf("%s",fast ? "vfasterpow" : "vfastpow");
      for (unsigned lane=0;lane<4;++lane) printf(" %08x",base_bits[lane]);
      for (unsigned lane=0;lane<4;++lane) printf(" %08x",exponent_bits[lane]);
      for (unsigned lane=0;lane<4;++lane) printf(" %08x",output[lane]);
      puts("");
    }
  }
  const uint32_t conversions[6][4] = {{0,0x80000000,0x3ff33333,0xbff33333},
    {0x4effffff,0x4f000000,0xcf000000,0xcf000001},
    {0x7f800000,0xff800000,0x7fc12345,0xffc12345},
    {1,0x80000001,0x4b800001,0xcb800001},
    {0x4b7fffff,0xcb7fffff,0x4e800001,0xce800001},
    {0x3f7fffff,0xbf7fffff,0x7f7fffff,0xff7fffff}};
  for (unsigned group=0;group<6;++group) {
    v4sf input, output;
    memcpy(&input,conversions[group],sizeof input);
    v4si integers = v4sf_to_v4si(input);
    memcpy(&output,&integers,sizeof output);
    record("to_integers",input,output);
    record("absolute",input,v4sf_fabs(input));
  }
  const int32_t integers[4][4] = {{0,-1,1,16777217},
    {16777219,-16777217,-16777219,2147483647},
    {(-2147483647-1),2147483520,-2147483520,123456789},
    {33554433,33554435,-33554433,-33554435}};
  for (unsigned group=0;group<4;++group) {
    v4si input;
    v4sf bits;
    memcpy(&input,integers[group],sizeof input);
    memcpy(&bits,&input,sizeof bits);
    record("from_integers",bits,v4si_to_v4sf(input));
  }
  return 0;
}
