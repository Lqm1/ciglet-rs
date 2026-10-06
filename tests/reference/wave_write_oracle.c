#include <stdint.h>
#include "ciglet.h"

static void number(FILE *file,uint32_t value) {
    if(fwrite(&value,4,1,file)!=1) exit(3);
}
int main(int argc,char **argv) {
    if(argc!=2) return 2;
    FILE *output=fopen(argv[1],"wb"); if(!output) return 3;
    fwrite("WVW1",1,4,output); number(output,sizeof(FP_TYPE)); number(output,12);
    const int depths[]={8,16,24,32}; const int lengths[]={0,8,9};
    const double values[]={-0.875,-0.5,-0.125,-0.00001,0,0.125,0.5,0.875};
    for(int depth=0;depth<4;depth++) for(int size=0;size<3;size++) {
        int n=lengths[size],bits=depths[depth]; FP_TYPE samples[9];
        for(int i=0;i<n;i++) samples[i]=values[i%8];
        number(output,bits); number(output,22050); number(output,n);
        if(fwrite(samples,sizeof(FP_TYPE),n,output)!=(size_t)n) return 3;
        FILE *wave=tmpfile(); if(!wave) return 3;
        wavwrite_fp(samples,n,22050,bits,wave); long length=ftell(wave); number(output,length); rewind(wave);
        for(long i=0;i<length;i++) { int byte=fgetc(wave); if(byte==EOF) return 3; fputc(byte,output); }
        fclose(wave);
    }
    return fclose(output)!=0 ? 3 : 0;
}
