#include <stdint.h>
#include "ciglet.h"

static void number(FILE *file, uint64_t value, int bytes) {
    for (int i = 0; i < bytes; i++) if (fputc((value >> (i * 8)) & 255, file) == EOF) exit(3);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *output = fopen(argv[1], "wb"); if (!output) return 3;
    fwrite("WAV1", 1, 4, output); number(output, sizeof(FP_TYPE), 4); number(output, 36, 4);
    const int widths[] = {8,16,24,32,32,64};
    for (int kind = 0; kind < 6; kind++) for (int channels = 1; channels <= 2; channels++) for (int variant = 0; variant < 3; variant++) {
        int bits = widths[kind], floating = kind >= 4, bytes = bits / 8;
        int fmt_size = variant == 0 ? 16 : variant == 1 ? 18 : 40;
        int data_size = 7 * channels * bytes;
        FILE *wave = tmpfile(); if (!wave) return 3;
        fwrite("RIFF",1,4,wave); number(wave,20+fmt_size+data_size+(data_size&1),4); fwrite("WAVEfmt ",1,8,wave);
        number(wave,fmt_size,4); number(wave,variant==2 ? 65534 : floating ? 3 : 1,2); number(wave,channels,2);
        number(wave,16000,4); number(wave,16000*channels*bytes,4); number(wave,channels*bytes,2); number(wave,bits,2);
        if (variant) number(wave,variant==2 ? 22 : 0,2);
        if (variant==2) {
            number(wave,bits,2); number(wave,channels==1 ? 4 : 3,4); number(wave,floating ? 3 : 1,4);
            const unsigned char tail[] = {0,0,16,0,128,0,0,170,0,56,155,113}; fwrite(tail,1,12,wave);
        }
        fwrite("data",1,4,wave); number(wave,data_size,4);
        int64_t scale = INT64_C(1) << (floating ? 30 : bits-1);
        const int64_t integers[] = {-scale,-scale/2,-1,0,1,scale/4,scale-1};
        const double floats[] = {-1.25,-0.5,-0.0000001,0,0.25,1.5,1e-30};
        for (int i=0;i<7;i++) for (int channel=0;channel<channels;channel++) {
            if (!floating) number(wave,channel ? 0 : bits==8 ? integers[i]+128 : integers[i],bytes);
            else if (bits==32) { float value=channel ? 0.75f : (float)floats[i]; fwrite(&value,4,1,wave); }
            else { double value=channel ? 0.75 : floats[i]; fwrite(&value,8,1,wave); }
        }
        if (data_size&1) fputc(0,wave);
        long length=ftell(wave); number(output,length,4); rewind(wave);
        for (long i=0;i<length;i++) { int byte=fgetc(wave); if(byte==EOF) return 3; fputc(byte,output); }
        rewind(wave); int rate=0, depth=0, samples=0; FP_TYPE *result=wavread_fp(wave,&rate,&depth,&samples);
        if (!result || samples!=7 || rate!=16000 || depth!=bits) return 4;
        if (fwrite(result,sizeof(FP_TYPE),samples,output)!=(size_t)samples) return 3;
        free(result); fclose(wave);
    }
    return fclose(output)!=0 ? 3 : 0;
}
