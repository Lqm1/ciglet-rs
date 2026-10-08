/* Independent optimized C caller for source scalar macros and constants. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "ciglet_rs.h"
#include <assert.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
static float ordered_f32(float acc, float value, void *ctx) { ++*(size_t *)ctx; return acc * 10 + value; }
static double ordered_f64(double acc, double value, void *ctx) { ++*(size_t *)ctx; return acc * 10 + value; }
int main(void) {
{
    float output=71, input[3]={1,2,3};
    size_t calls=0;
    assert(ciglet_rs_scalar_fold_f32(input,3,4,ordered_f32,&calls,&output)==0 && output==4123 && calls==3);
    assert(ciglet_rs_scalar_fold_f32(input,3,4,ordered_f32,&calls,input)==0 && input[0]==4123 && calls==6);
    assert(ciglet_rs_scalar_fold_f32(NULL,0,-0.0,NULL,NULL,&output)==0 && signbit(output));
    output=71;
    assert(ciglet_rs_scalar_fold_f32(input,3,4,NULL,&calls,&output)==1 && output==71 && calls==6);
    assert(ciglet_rs_scalar_add_f32(2,3,&output)==0 && output==5);
    assert(ciglet_rs_scalar_add_f32(1,2,NULL)==1);
    assert(ciglet_rs_scalar_accumulate_square_f32(5,-3,&output)==0 && output==14);
    assert(ciglet_rs_scalar_accumulate_square_f32(1,2,NULL)==1);
    assert(ciglet_rs_scalar_minimum_f32(-5,7,&output)==0 && output==-5);
    assert(ciglet_rs_scalar_minimum_f32(1,2,NULL)==1);
    assert(ciglet_rs_scalar_maximum_f32(-5,7,&output)==0 && output==7);
    assert(ciglet_rs_scalar_maximum_f32(1,2,NULL)==1);
    assert(ciglet_rs_scalar_minimum_f32(0.0,-0.0,&output)==0 && signbit(output));
    assert(ciglet_rs_scalar_maximum_f32(NAN,3,&output)==0 && output==3);
    assert(ciglet_rs_scalar_add_f32(INFINITY,-INFINITY,&output)==0 && isnan(output));
}
{
    double output=71, input[3]={1,2,3};
    size_t calls=0;
    assert(ciglet_rs_scalar_fold_f64(input,3,4,ordered_f64,&calls,&output)==0 && output==4123 && calls==3);
    assert(ciglet_rs_scalar_fold_f64(input,3,4,ordered_f64,&calls,input)==0 && input[0]==4123 && calls==6);
    assert(ciglet_rs_scalar_fold_f64(NULL,0,-0.0,NULL,NULL,&output)==0 && signbit(output));
    output=71;
    assert(ciglet_rs_scalar_fold_f64(input,3,4,NULL,&calls,&output)==1 && output==71 && calls==6);
    assert(ciglet_rs_scalar_add_f64(2,3,&output)==0 && output==5);
    assert(ciglet_rs_scalar_add_f64(1,2,NULL)==1);
    assert(ciglet_rs_scalar_accumulate_square_f64(5,-3,&output)==0 && output==14);
    assert(ciglet_rs_scalar_accumulate_square_f64(1,2,NULL)==1);
    assert(ciglet_rs_scalar_minimum_f64(-5,7,&output)==0 && output==-5);
    assert(ciglet_rs_scalar_minimum_f64(1,2,NULL)==1);
    assert(ciglet_rs_scalar_maximum_f64(-5,7,&output)==0 && output==7);
    assert(ciglet_rs_scalar_maximum_f64(1,2,NULL)==1);
    assert(ciglet_rs_scalar_minimum_f64(0.0,-0.0,&output)==0 && signbit(output));
    assert(ciglet_rs_scalar_maximum_f64(NAN,3,&output)==0 && output==3);
    assert(ciglet_rs_scalar_add_f64(INFINITY,-INFINITY,&output)==0 && isnan(output));
}
CigletRsMathConstants constants;
assert(sizeof(constants)==4*sizeof(double));
assert(offsetof(CigletRsMathConstants,half_pi)==3*sizeof(double));
assert(ciglet_rs_math_constants(&constants)==0);
assert(ciglet_rs_math_constants(NULL)==1);
uint64_t bits[4]; memcpy(bits,&constants,sizeof(bits));
assert(bits[0]==UINT64_C(0x3cd203af9ee75616));
assert(bits[1]==UINT64_C(0x430c6bf526340000));
assert(bits[2]==UINT64_C(0x400921fb54442d18));
assert(bits[3]==UINT64_C(0x3ff921fb54442d18));
return 0;
}
