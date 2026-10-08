/* Real shared-library caller for original integer draws and owned samplers. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "ciglet_rs.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <string.h>
static float scalar32(uint32_t bits) { float value;memcpy(&value,&bits,4);return value; }
static double scalar64(uint64_t bits) { double value;memcpy(&value,&bits,8);return value; }
static uint32_t bits32(float value) { uint32_t bits;memcpy(&bits,&value,4);return bits; }
static uint64_t bits64(double value) { uint64_t bits;memcpy(&bits,&value,8);return bits; }
int main(void) {
uint8_t seed[32];memset(seed,37,sizeof(seed));
struct CigletRsRandom *owner=NULL,*clone=NULL;
uint32_t maximum=0,draw=0;
assert(ciglet_rs_random_create(seed,32,&owner)==0);
assert(ciglet_rs_random_maximum(owner,&maximum)==0 && maximum==INT32_MAX);
assert(ciglet_rs_random_release(&owner)==0 && owner==NULL);
assert(ciglet_rs_random_create_with_maximum(seed,32,31,&owner)==0);
assert(ciglet_rs_random_maximum(owner,&maximum)==0 && maximum==31);
assert(ciglet_rs_random_clone(owner,&clone)==0);
assert(ciglet_rs_random_draw(owner,&draw)==0);
uint32_t other=0;assert(ciglet_rs_random_draw(clone,&other)==0 && other==draw);
assert(ciglet_rs_random_release(&clone)==0);
{
float output=71,expected=0;
assert(ciglet_rs_random_uniform_from_draw_f32(1,32767,&output)==0 && bits32(output)==UINT32_C(0xbf7ffc00));
assert(ciglet_rs_random_normal_from_draws_f32(scalar32(UINT32_C(0xbf800000)),scalar32(UINT32_C(0x0)),1,0,32767,CIGLET_RS_RANDOM_LEGACY,&output)==0 && bits32(output)==UINT32_C(0xbf800000));
assert(ciglet_rs_random_white_from_draw_f32(scalar32(UINT32_C(0x0)),1,32767,&output)==0 && bits32(output)==UINT32_C(0x80000000));
uint32_t original[2]={1,0};float samples[7],reference[7];
assert(ciglet_rs_random_white_from_draws_f32(scalar32(UINT32_C(0x0)),original,2,32767,samples)==0);
assert(bits32(samples[0])==UINT32_C(0x80000000) && bits32(samples[1])==UINT32_C(0x80000000));
assert(ciglet_rs_random_clone(owner,&clone)==0);
assert(ciglet_rs_random_uniform_f32(owner,&output)==0);
assert(ciglet_rs_random_draw(clone,&draw)==0);
assert(ciglet_rs_random_uniform_from_draw_f32(draw,31,&expected)==0 && bits32(output)==bits32(expected));
assert(ciglet_rs_random_normal_f32(owner,3,0,&output)==0 && output==3);
assert(ciglet_rs_random_draw(clone,&draw)==0);assert(ciglet_rs_random_draw(clone,&draw)==0);
uint32_t first,second;
assert(ciglet_rs_random_normal_with_f32(owner,1,2,CIGLET_RS_RANDOM_LEGACY,&output)==0);
assert(ciglet_rs_random_draw(clone,&first)==0);assert(ciglet_rs_random_draw(clone,&second)==0);
assert(ciglet_rs_random_normal_from_draws_f32(1,2,first,second,31,CIGLET_RS_RANDOM_LEGACY,&expected)==0 && bits32(output)==bits32(expected));
uint32_t draws[7];for(int i=0;i<7;++i)assert(ciglet_rs_random_draw(clone,&draws[i])==0);
assert(ciglet_rs_random_white_noise_f32(owner,-2,samples,7)==0);
assert(ciglet_rs_random_white_from_draws_f32(-2,draws,7,31,reference)==0);
for(int i=0;i<7;++i)assert(bits32(samples[i])==bits32(reference[i]));
assert(ciglet_rs_random_white_noise_f32(owner,1,NULL,0)==0);
output=71;assert(ciglet_rs_random_normal_with_f32(owner,0,1,99,&output)==3 && output==71);
assert(ciglet_rs_random_uniform_f32(owner,NULL)==1);
assert(ciglet_rs_random_draw(owner,&draw)==0);assert(ciglet_rs_random_draw(clone,&other)==0 && draw==other);
assert(ciglet_rs_random_release(&clone)==0);
}
{
double output=71,expected=0;
assert(ciglet_rs_random_uniform_from_draw_f64(1,32767,&output)==0 && bits64(output)==UINT64_C(0xbfefff7ffefffe00));
assert(ciglet_rs_random_normal_from_draws_f64(scalar64(UINT64_C(0xbff0000000000000)),scalar64(UINT64_C(0x0)),1,0,32767,CIGLET_RS_RANDOM_LEGACY,&output)==0 && bits64(output)==UINT64_C(0xbff0000000000000));
assert(ciglet_rs_random_white_from_draw_f64(scalar64(UINT64_C(0x0)),1,32767,&output)==0 && bits64(output)==UINT64_C(0x8000000000000000));
uint32_t original[2]={1,0};double samples[7],reference[7];
assert(ciglet_rs_random_white_from_draws_f64(scalar64(UINT64_C(0x0)),original,2,32767,samples)==0);
assert(bits64(samples[0])==UINT64_C(0x8000000000000000) && bits64(samples[1])==UINT64_C(0x8000000000000000));
assert(ciglet_rs_random_clone(owner,&clone)==0);
assert(ciglet_rs_random_uniform_f64(owner,&output)==0);
assert(ciglet_rs_random_draw(clone,&draw)==0);
assert(ciglet_rs_random_uniform_from_draw_f64(draw,31,&expected)==0 && bits64(output)==bits64(expected));
assert(ciglet_rs_random_normal_f64(owner,3,0,&output)==0 && output==3);
assert(ciglet_rs_random_draw(clone,&draw)==0);assert(ciglet_rs_random_draw(clone,&draw)==0);
uint32_t first,second;
assert(ciglet_rs_random_normal_with_f64(owner,1,2,CIGLET_RS_RANDOM_LEGACY,&output)==0);
assert(ciglet_rs_random_draw(clone,&first)==0);assert(ciglet_rs_random_draw(clone,&second)==0);
assert(ciglet_rs_random_normal_from_draws_f64(1,2,first,second,31,CIGLET_RS_RANDOM_LEGACY,&expected)==0 && bits64(output)==bits64(expected));
uint32_t draws[7];for(int i=0;i<7;++i)assert(ciglet_rs_random_draw(clone,&draws[i])==0);
assert(ciglet_rs_random_white_noise_f64(owner,-2,samples,7)==0);
assert(ciglet_rs_random_white_from_draws_f64(-2,draws,7,31,reference)==0);
for(int i=0;i<7;++i)assert(bits64(samples[i])==bits64(reference[i]));
assert(ciglet_rs_random_white_noise_f64(owner,1,NULL,0)==0);
output=71;assert(ciglet_rs_random_normal_with_f64(owner,0,1,99,&output)==3 && output==71);
assert(ciglet_rs_random_uniform_f64(owner,NULL)==1);
assert(ciglet_rs_random_draw(owner,&draw)==0);assert(ciglet_rs_random_draw(clone,&other)==0 && draw==other);
assert(ciglet_rs_random_release(&clone)==0);
}
assert(ciglet_rs_random_release(&owner)==0);assert(ciglet_rs_random_release(&owner)==0);return 0;
}
