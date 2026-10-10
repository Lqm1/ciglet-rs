/* Independent C reader exercises memory, borrowed callbacks and actual files. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "ciglet_rs.h"
#include <assert.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#ifdef _WIN32
#include <process.h>
#define PROCESS_ID _getpid()
#define OS_LABEL "win"
#else
#include <unistd.h>
#define PROCESS_ID getpid()
#define OS_LABEL "unix"
#endif
static const uint8_t wave_bytes[]={82,73,70,70,44,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,128,62,0,0,128,62,0,0,1,0,8,0,100,97,116,97,7,0,0,0,0,64,127,128,129,160,255,0};
struct Stream { const uint8_t *data; size_t length; uint64_t position; size_t reads,seeks; int failure; };
static uint32_t read_bytes(void *opaque,uint8_t *output,uintptr_t capacity,uintptr_t *count) {
    struct Stream *s=opaque;++s->reads;
    if(s->failure==1)return 9;
    if(s->failure==2){*count=capacity+1;return 0;}
    if(s->failure==3 || s->position>=s->length){*count=0;return 0;}
    size_t n=capacity?1:0;memcpy(output,s->data+(size_t)s->position,n);s->position+=n;*count=n;return 0;
}
static uint32_t seek_bytes(void *opaque,uint32_t origin,uint64_t absolute,int64_t relative,uint64_t *position) {
    struct Stream *s=opaque;++s->seeks;if(s->failure==4)return 9;
    uint64_t result;
    if(origin==CIGLET_RS_SEEK_START) result=absolute;
    else {
        uint64_t base=origin==CIGLET_RS_SEEK_CURRENT?s->position:s->length;
        if(relative<0){uint64_t distance=(uint64_t)(-(relative+1))+1;if(distance>base)return 9;result=base-distance;}
        else {if(base>UINT64_MAX-(uint64_t)relative)return 9;result=base+(uint64_t)relative;}
    }
    s->position=result;*position=result;return 0;
}
int main(void) {
uint8_t prefixed[sizeof(wave_bytes)+3]={13,14,15};memcpy(prefixed+3,wave_bytes,sizeof(wave_bytes));
char filename[128];snprintf(filename,sizeof(filename),"wave-c-reader-%s-%d.wav",OS_LABEL,(int)PROCESS_ID);
FILE *file=fopen(filename,"wb");assert(file);assert(fwrite(wave_bytes,1,sizeof(wave_bytes),file)==sizeof(wave_bytes));assert(fclose(file)==0);
struct CigletRsPath path={(const uint8_t *)filename,strlen(filename),CIGLET_RS_PATH_UTF8};
uint8_t native[256];size_t native_count;
#ifdef _WIN32
native_count=2*strlen(filename);for(size_t i=0;i<strlen(filename);++i){native[2*i]=(uint8_t)filename[i];native[2*i+1]=0;}
#else
native_count=strlen(filename);memcpy(native,filename,native_count);
#endif
struct CigletRsPath native_path={native,native_count,CIGLET_RS_PATH_NATIVE};
{
struct CigletRsWaveF32 *owner=NULL,*clone=NULL;struct CigletRsWaveInfo info;
float samples[7];const float expected[7]={-1,-0.5,-1.0/128,0,1.0/128,0.25,127.0/128};
assert(ciglet_rs_wave_decode_f32(prefixed,sizeof(prefixed),3,7,&owner)==0);
assert(ciglet_rs_wave_info_f32(owner,&info)==0 && info.sample_count==7 && info.sample_rate==16000 && info.bits_per_sample==8 && info.channels==1 && info.encoding==CIGLET_RS_WAVE_PCM);
assert(ciglet_rs_wave_clone_f32(owner,&clone)==0);assert(ciglet_rs_wave_release_f32(&owner)==0);
assert(ciglet_rs_wave_copy_samples_f32(clone,samples,7)==0);for(int i=0;i<7;++i)assert(samples[i]==expected[i]);
assert(ciglet_rs_wave_copy_samples_f32(clone,samples,6)==2);assert(ciglet_rs_wave_release_f32(&clone)==0);
struct Stream context={prefixed,sizeof(prefixed),3,0,0,0};
struct CigletRsReadStream stream={&context,read_bytes,seek_bytes};
assert(ciglet_rs_wave_read_stream_f32(stream,7,&owner)==0 && context.reads>10 && context.seeks>0);
assert(ciglet_rs_wave_copy_samples_f32(owner,samples,7)==0);for(int i=0;i<7;++i)assert(samples[i]==expected[i]);
assert(ciglet_rs_wave_release_f32(&owner)==0);
for(int failure=1;failure<=4;++failure){context.position=3;context.failure=failure;assert(ciglet_rs_wave_read_stream_f32(stream,7,&owner)==3 && owner==NULL);}
context.failure=0;size_t reads=context.reads,seeks=context.seeks;
assert(ciglet_rs_wave_read_stream_f32(stream,7,NULL)==1 && context.reads==reads && context.seeks==seeks);
assert(ciglet_rs_wave_read_file_f32(path,7,&owner)==0);assert(ciglet_rs_wave_release_f32(&owner)==0);
assert(ciglet_rs_wave_read_file_f32(native_path,7,&owner)==0);assert(ciglet_rs_wave_release_f32(&owner)==0);
assert(ciglet_rs_wave_decode_f32(wave_bytes,sizeof(wave_bytes),0,6,&owner)==3 && owner==NULL);
assert(ciglet_rs_wave_release_f32(&owner)==0);
}
{
struct CigletRsWaveF64 *owner=NULL,*clone=NULL;struct CigletRsWaveInfo info;
_Alignas(8) double samples[7];_Alignas(8) const double expected[7]={-1,-0.5,-1.0/128,0,1.0/128,0.25,127.0/128};
assert(ciglet_rs_wave_decode_f64(prefixed,sizeof(prefixed),3,7,&owner)==0);
assert(ciglet_rs_wave_info_f64(owner,&info)==0 && info.sample_count==7 && info.sample_rate==16000 && info.bits_per_sample==8 && info.channels==1 && info.encoding==CIGLET_RS_WAVE_PCM);
assert(ciglet_rs_wave_clone_f64(owner,&clone)==0);assert(ciglet_rs_wave_release_f64(&owner)==0);
assert(ciglet_rs_wave_copy_samples_f64(clone,samples,7)==0);for(int i=0;i<7;++i)assert(samples[i]==expected[i]);
assert(ciglet_rs_wave_copy_samples_f64(clone,samples,6)==2);assert(ciglet_rs_wave_release_f64(&clone)==0);
struct Stream context={prefixed,sizeof(prefixed),3,0,0,0};
struct CigletRsReadStream stream={&context,read_bytes,seek_bytes};
assert(ciglet_rs_wave_read_stream_f64(stream,7,&owner)==0 && context.reads>10 && context.seeks>0);
assert(ciglet_rs_wave_copy_samples_f64(owner,samples,7)==0);for(int i=0;i<7;++i)assert(samples[i]==expected[i]);
assert(ciglet_rs_wave_release_f64(&owner)==0);
for(int failure=1;failure<=4;++failure){context.position=3;context.failure=failure;assert(ciglet_rs_wave_read_stream_f64(stream,7,&owner)==3 && owner==NULL);}
context.failure=0;size_t reads=context.reads,seeks=context.seeks;
assert(ciglet_rs_wave_read_stream_f64(stream,7,NULL)==1 && context.reads==reads && context.seeks==seeks);
assert(ciglet_rs_wave_read_file_f64(path,7,&owner)==0);assert(ciglet_rs_wave_release_f64(&owner)==0);
assert(ciglet_rs_wave_read_file_f64(native_path,7,&owner)==0);assert(ciglet_rs_wave_release_f64(&owner)==0);
assert(ciglet_rs_wave_decode_f64(wave_bytes,sizeof(wave_bytes),0,6,&owner)==3 && owner==NULL);
assert(ciglet_rs_wave_release_f64(&owner)==0);
}
assert(remove(filename)==0);return 0;
}
