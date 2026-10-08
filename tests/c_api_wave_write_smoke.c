/* Actual shared-library caller, preserving all source WAVE writer oracle bytes. */
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
struct Sink { uint8_t bytes[256]; size_t count,writes,flushes; int failure; };
static uint32_t write_bytes(void *opaque,const uint8_t *bytes,uintptr_t length,uintptr_t *written) {
    struct Sink *s=opaque;++s->writes;
    if(s->failure==1)return 9;
    if(s->failure==2){*written=length+1;return 0;}
    if(s->failure==3){*written=0;return 0;}
    if(s->failure==4 && s->count>=8)return 9;
    size_t n=length?1:0;assert(s->count+n<=sizeof(s->bytes));memcpy(s->bytes+s->count,bytes,n);s->count+=n;*written=n;return 0;
}
static uint32_t flush_bytes(void *opaque){++((struct Sink *)opaque)->flushes;return 0;}
static float scalar32(uint32_t bits){float value;memcpy(&value,&bits,4);return value;}
static double scalar64(uint64_t bits){double value;memcpy(&value,&bits,8);return value;}
static const uint32_t input_f32_0[]={0};
static const uint8_t expected_f32_0[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,0,0,0,0};
static const uint32_t input_f32_1[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000)};
static const uint8_t expected_f32_1[]={82,73,70,70,44,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,8,0,0,0,15,63,111,126,127,142,190,238};
static const uint32_t input_f32_2[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000),UINT32_C(0xbf600000)};
static const uint8_t expected_f32_2[]={82,73,70,70,45,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,9,0,0,0,15,63,111,126,127,142,190,238,15};
static const uint32_t input_f32_3[]={0};
static const uint8_t expected_f32_3[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,0,0,0,0};
static const uint32_t input_f32_4[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000)};
static const uint8_t expected_f32_4[]={82,73,70,70,52,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,16,0,0,0,1,144,1,192,1,240,0,0,0,0,255,15,255,63,255,111};
static const uint32_t input_f32_5[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000),UINT32_C(0xbf600000)};
static const uint8_t expected_f32_5[]={82,73,70,70,54,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,18,0,0,0,1,144,1,192,1,240,0,0,0,0,255,15,255,63,255,111,1,144};
static const uint32_t input_f32_6[]={0};
static const uint8_t expected_f32_6[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,0,0,0,0};
static const uint32_t input_f32_7[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000)};
static const uint8_t expected_f32_7[]={82,73,70,70,60,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,24,0,0,0,1,0,144,1,0,192,1,0,240,173,255,255,0,0,0,255,255,15,255,255,63,255,255,111};
static const uint32_t input_f32_8[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000),UINT32_C(0xbf600000)};
static const uint8_t expected_f32_8[]={82,73,70,70,63,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,27,0,0,0,1,0,144,1,0,192,1,0,240,173,255,255,0,0,0,255,255,15,255,255,63,255,255,111,1,0,144};
static const uint32_t input_f32_9[]={0};
static const uint8_t expected_f32_9[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,0,0,0,0};
static const uint32_t input_f32_10[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000)};
static const uint8_t expected_f32_10[]={82,73,70,70,68,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,32,0,0,0,0,0,0,144,0,0,0,192,0,0,0,240,30,172,255,255,0,0,0,0,0,0,0,16,0,0,0,64,0,0,0,112};
static const uint32_t input_f32_11[]={UINT32_C(0xbf600000),UINT32_C(0xbf000000),UINT32_C(0xbe000000),UINT32_C(0xb727c5ac),UINT32_C(0x0),UINT32_C(0x3e000000),UINT32_C(0x3f000000),UINT32_C(0x3f600000),UINT32_C(0xbf600000)};
static const uint8_t expected_f32_11[]={82,73,70,70,72,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,36,0,0,0,0,0,0,144,0,0,0,192,0,0,0,240,30,172,255,255,0,0,0,0,0,0,0,16,0,0,0,64,0,0,0,112,0,0,0,144};
static const uint64_t input_f64_0[]={0};
static const uint8_t expected_f64_0[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,0,0,0,0};
static const uint64_t input_f64_1[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000)};
static const uint8_t expected_f64_1[]={82,73,70,70,44,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,8,0,0,0,15,63,111,126,127,142,190,238};
static const uint64_t input_f64_2[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000),UINT64_C(0xbfec000000000000)};
static const uint8_t expected_f64_2[]={82,73,70,70,45,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,34,86,0,0,1,0,8,0,100,97,116,97,9,0,0,0,15,63,111,126,127,142,190,238,15};
static const uint64_t input_f64_3[]={0};
static const uint8_t expected_f64_3[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,0,0,0,0};
static const uint64_t input_f64_4[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000)};
static const uint8_t expected_f64_4[]={82,73,70,70,52,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,16,0,0,0,1,144,1,192,1,240,0,0,0,0,255,15,255,63,255,111};
static const uint64_t input_f64_5[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000),UINT64_C(0xbfec000000000000)};
static const uint8_t expected_f64_5[]={82,73,70,70,54,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,68,172,0,0,2,0,16,0,100,97,116,97,18,0,0,0,1,144,1,192,1,240,0,0,0,0,255,15,255,63,255,111,1,144};
static const uint64_t input_f64_6[]={0};
static const uint8_t expected_f64_6[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,0,0,0,0};
static const uint64_t input_f64_7[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000)};
static const uint8_t expected_f64_7[]={82,73,70,70,60,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,24,0,0,0,1,0,144,1,0,192,1,0,240,173,255,255,0,0,0,255,255,15,255,255,63,255,255,111};
static const uint64_t input_f64_8[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000),UINT64_C(0xbfec000000000000)};
static const uint8_t expected_f64_8[]={82,73,70,70,63,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,102,2,1,0,3,0,24,0,100,97,116,97,27,0,0,0,1,0,144,1,0,192,1,0,240,173,255,255,0,0,0,255,255,15,255,255,63,255,255,111,1,0,144};
static const uint64_t input_f64_9[]={0};
static const uint8_t expected_f64_9[]={82,73,70,70,36,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,0,0,0,0};
static const uint64_t input_f64_10[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000)};
static const uint8_t expected_f64_10[]={82,73,70,70,68,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,32,0,0,0,1,0,0,144,1,0,0,192,1,0,0,240,30,172,255,255,0,0,0,0,255,255,255,15,255,255,255,63,255,255,255,111};
static const uint64_t input_f64_11[]={UINT64_C(0xbfec000000000000),UINT64_C(0xbfe0000000000000),UINT64_C(0xbfc0000000000000),UINT64_C(0xbee4f8b588e368f1),UINT64_C(0x0),UINT64_C(0x3fc0000000000000),UINT64_C(0x3fe0000000000000),UINT64_C(0x3fec000000000000),UINT64_C(0xbfec000000000000)};
static const uint8_t expected_f64_11[]={82,73,70,70,72,0,0,0,87,65,86,69,102,109,116,32,16,0,0,0,1,0,1,0,34,86,0,0,136,88,1,0,4,0,32,0,100,97,116,97,36,0,0,0,1,0,0,144,1,0,0,192,1,0,0,240,30,172,255,255,0,0,0,0,255,255,255,15,255,255,255,63,255,255,255,111,1,0,0,144};

static void check_f32(uint16_t bits,uint32_t rate,const uint32_t *input,size_t count,const uint8_t *expected,size_t expected_count,int case_index) {
    float samples[16];assert(count<=16);for(size_t i=0;i<count;++i)samples[i]=scalar32(input[i]);
    struct CigletRsWaveWriteOptions options;assert(ciglet_rs_wave_write_options_legacy(bits,&options)==0);
    struct CigletRsBytes *owner=NULL,*cloned=NULL;uintptr_t size=0;uint8_t bytes[256];
    assert(ciglet_rs_wave_encode_f32(samples,count,rate,options,&owner)==0);
    assert(ciglet_rs_bytes_length(owner,&size)==0 && size==expected_count && size<=sizeof(bytes));
    assert(ciglet_rs_bytes_clone(owner,&cloned)==0);assert(ciglet_rs_bytes_release(&owner)==0);
    assert(ciglet_rs_bytes_copy(cloned,0,bytes,size)==0 && memcmp(bytes,expected,size)==0);
    assert(ciglet_rs_bytes_copy(cloned,size,NULL,0)==0);
    memset(bytes,71,size);assert(ciglet_rs_bytes_copy(cloned,1,bytes,size)==2);for(size_t j=0;j<size;++j)assert(bytes[j]==71);
    assert(ciglet_rs_bytes_copy(cloned,1,bytes,size-1)==0 && memcmp(bytes,expected+1,size-1)==0);
    assert(ciglet_rs_bytes_release(&cloned)==0);
    struct Sink sink={0};struct CigletRsWriteStream stream={&sink,write_bytes,flush_bytes};
    assert(ciglet_rs_wave_write_stream_f32(stream,samples,count,rate,options)==0 && sink.count==size && sink.flushes==0 && memcmp(sink.bytes,expected,size)==0);
    char filename[128];snprintf(filename,sizeof(filename),"wave-c-writer-%s-%d-f32-%d.wav",OS_LABEL,(int)PROCESS_ID,case_index);
    uint8_t native[256];size_t native_count;
#ifdef _WIN32
    native_count=2*strlen(filename);for(size_t j=0;j<strlen(filename);++j){native[2*j]=(uint8_t)filename[j];native[2*j+1]=0;}
#else
    native_count=strlen(filename);memcpy(native,filename,native_count);
#endif
    struct CigletRsPath path={native,native_count,CIGLET_RS_PATH_NATIVE};
    assert(ciglet_rs_wave_write_file_f32(path,samples,count,rate,options)==0);
    FILE *file=fopen(filename,"rb");assert(file);assert(fread(bytes,1,sizeof(bytes),file)==size);assert(fclose(file)==0);assert(memcmp(bytes,expected,size)==0);assert(remove(filename)==0);
}

static void check_f64(uint16_t bits,uint32_t rate,const uint64_t *input,size_t count,const uint8_t *expected,size_t expected_count,int case_index) {
    double samples[16];assert(count<=16);for(size_t i=0;i<count;++i)samples[i]=scalar64(input[i]);
    struct CigletRsWaveWriteOptions options;assert(ciglet_rs_wave_write_options_legacy(bits,&options)==0);
    struct CigletRsBytes *owner=NULL,*cloned=NULL;uintptr_t size=0;uint8_t bytes[256];
    assert(ciglet_rs_wave_encode_f64(samples,count,rate,options,&owner)==0);
    assert(ciglet_rs_bytes_length(owner,&size)==0 && size==expected_count && size<=sizeof(bytes));
    assert(ciglet_rs_bytes_clone(owner,&cloned)==0);assert(ciglet_rs_bytes_release(&owner)==0);
    assert(ciglet_rs_bytes_copy(cloned,0,bytes,size)==0 && memcmp(bytes,expected,size)==0);
    assert(ciglet_rs_bytes_copy(cloned,size,NULL,0)==0);
    memset(bytes,71,size);assert(ciglet_rs_bytes_copy(cloned,1,bytes,size)==2);for(size_t j=0;j<size;++j)assert(bytes[j]==71);
    assert(ciglet_rs_bytes_copy(cloned,1,bytes,size-1)==0 && memcmp(bytes,expected+1,size-1)==0);
    assert(ciglet_rs_bytes_release(&cloned)==0);
    struct Sink sink={0};struct CigletRsWriteStream stream={&sink,write_bytes,flush_bytes};
    assert(ciglet_rs_wave_write_stream_f64(stream,samples,count,rate,options)==0 && sink.count==size && sink.flushes==0 && memcmp(sink.bytes,expected,size)==0);
    char filename[128];snprintf(filename,sizeof(filename),"wave-c-writer-%s-%d-f64-%d.wav",OS_LABEL,(int)PROCESS_ID,case_index);
    uint8_t native[256];size_t native_count;
#ifdef _WIN32
    native_count=2*strlen(filename);for(size_t j=0;j<strlen(filename);++j){native[2*j]=(uint8_t)filename[j];native[2*j+1]=0;}
#else
    native_count=strlen(filename);memcpy(native,filename,native_count);
#endif
    struct CigletRsPath path={native,native_count,CIGLET_RS_PATH_NATIVE};
    assert(ciglet_rs_wave_write_file_f64(path,samples,count,rate,options)==0);
    FILE *file=fopen(filename,"rb");assert(file);assert(fread(bytes,1,sizeof(bytes),file)==size);assert(fclose(file)==0);assert(memcmp(bytes,expected,size)==0);assert(remove(filename)==0);
}
int main(void){
struct CigletRsWaveWriteOptions options;assert(sizeof(options)==12);assert(ciglet_rs_wave_write_options_default(&options)==0 && options.bits_per_sample==16 && options.pcm8_quantization==CIGLET_RS_PCM8_CENTERED && options.pad_odd_data==1);
check_f32(8,22050,input_f32_0,0,expected_f32_0,sizeof(expected_f32_0),0);
check_f32(8,22050,input_f32_1,8,expected_f32_1,sizeof(expected_f32_1),1);
check_f32(8,22050,input_f32_2,9,expected_f32_2,sizeof(expected_f32_2),2);
check_f32(16,22050,input_f32_3,0,expected_f32_3,sizeof(expected_f32_3),3);
check_f32(16,22050,input_f32_4,8,expected_f32_4,sizeof(expected_f32_4),4);
check_f32(16,22050,input_f32_5,9,expected_f32_5,sizeof(expected_f32_5),5);
check_f32(24,22050,input_f32_6,0,expected_f32_6,sizeof(expected_f32_6),6);
check_f32(24,22050,input_f32_7,8,expected_f32_7,sizeof(expected_f32_7),7);
check_f32(24,22050,input_f32_8,9,expected_f32_8,sizeof(expected_f32_8),8);
check_f32(32,22050,input_f32_9,0,expected_f32_9,sizeof(expected_f32_9),9);
check_f32(32,22050,input_f32_10,8,expected_f32_10,sizeof(expected_f32_10),10);
check_f32(32,22050,input_f32_11,9,expected_f32_11,sizeof(expected_f32_11),11);
{
float samples[2]={0,0.5};struct Sink sink={0};struct CigletRsWriteStream stream={&sink,write_bytes,flush_bytes};
assert(ciglet_rs_wave_write_options_default(&options)==0);
for(int failure=1;failure<=4;++failure){memset(&sink,0,sizeof(sink));sink.failure=failure;assert(ciglet_rs_wave_write_stream_f32(stream,samples,2,16000,options)==3 && sink.flushes==0 && sink.count==(failure==4?8:0));}
memset(&sink,0,sizeof(sink));options.pad_odd_data=2;assert(ciglet_rs_wave_write_stream_f32(stream,samples,2,16000,options)==3 && sink.writes==0);
}
check_f64(8,22050,input_f64_0,0,expected_f64_0,sizeof(expected_f64_0),0);
check_f64(8,22050,input_f64_1,8,expected_f64_1,sizeof(expected_f64_1),1);
check_f64(8,22050,input_f64_2,9,expected_f64_2,sizeof(expected_f64_2),2);
check_f64(16,22050,input_f64_3,0,expected_f64_3,sizeof(expected_f64_3),3);
check_f64(16,22050,input_f64_4,8,expected_f64_4,sizeof(expected_f64_4),4);
check_f64(16,22050,input_f64_5,9,expected_f64_5,sizeof(expected_f64_5),5);
check_f64(24,22050,input_f64_6,0,expected_f64_6,sizeof(expected_f64_6),6);
check_f64(24,22050,input_f64_7,8,expected_f64_7,sizeof(expected_f64_7),7);
check_f64(24,22050,input_f64_8,9,expected_f64_8,sizeof(expected_f64_8),8);
check_f64(32,22050,input_f64_9,0,expected_f64_9,sizeof(expected_f64_9),9);
check_f64(32,22050,input_f64_10,8,expected_f64_10,sizeof(expected_f64_10),10);
check_f64(32,22050,input_f64_11,9,expected_f64_11,sizeof(expected_f64_11),11);
{
double samples[2]={0,0.5};struct Sink sink={0};struct CigletRsWriteStream stream={&sink,write_bytes,flush_bytes};
assert(ciglet_rs_wave_write_options_default(&options)==0);
for(int failure=1;failure<=4;++failure){memset(&sink,0,sizeof(sink));sink.failure=failure;assert(ciglet_rs_wave_write_stream_f64(stream,samples,2,16000,options)==3 && sink.flushes==0 && sink.count==(failure==4?8:0));}
memset(&sink,0,sizeof(sink));options.pad_odd_data=2;assert(ciglet_rs_wave_write_stream_f64(stream,samples,2,16000,options)==3 && sink.writes==0);
}
return 0;
}
