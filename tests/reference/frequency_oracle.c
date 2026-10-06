#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}
static void scalar(FILE *file, FP_TYPE value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}
static void single(FILE *file, float value) {
    if (fwrite(&value, sizeof(value), 1, file) != 1) exit(3);
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb"); if (!file) return 3;
    FP_TYPE frequencies[] = {0,50,100,440,600,1000,4000,8000,16000,22050,48000,96000};
    if (fwrite("FRQ1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE)); integer(file, 12);
    for (int i = 0; i < 12; i++) {
        FP_TYPE mel = freq2mel(frequencies[i]), bark = freq2bark(frequencies[i]);
        scalar(file, frequencies[i]); scalar(file, mel); scalar(file, mel2freq(mel));
        scalar(file, bark); scalar(file, bark2freq(bark)); scalar(file, eqloud(frequencies[i]));
    }
    int intervals[] = {1,2,5,20,40}; integer(file, 5);
    for (int i = 0; i < 5; i++) {
        FP_TYPE *points = melspace(50,8000,intervals[i]);
        integer(file, intervals[i]);
        for (int j = 0; j <= intervals[i]; j++) scalar(file, points[j]);
        free(points);
    }
    float logs[] = {1e-15f,1e-6f,0.001f,0.125f,0.5f,1.0f,1.25f,2.0f,10.0f,100.0f,10000.0f,1e15f};
    integer(file,12);
    for (int i=0;i<12;i++) { single(file,logs[i]); single(file,fastlog2(logs[i])); single(file,fastlog(logs[i])); }
    float exponents[] = {-130,-126,-100,-5,-0.25f,0,0.25f,1,5,10,40,80};
    integer(file,12);
    for (int i=0;i<12;i++) { single(file,exponents[i]); single(file,fastpow2(exponents[i])); single(file,fastexp(exponents[i])); single(file,fastpow(10.0f,exponents[i]/40.0f)); }
    return fclose(file) != 0 ? 3 : 0;
}
