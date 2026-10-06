#include <stdint.h>
#include "ciglet.h"

static FP_TYPE *generate(int kind,int approximate,int length) {
    switch(kind) {
        case 0:return boxcar(length);
        case 1:return approximate ? hanning_2(length) : hanning(length);
        case 2:return approximate ? hamming_2(length) : hamming(length);
        case 3:return approximate ? blackman_2(length) : blackman(length);
        case 4:return approximate ? blackman_harris_2(length) : blackman_harris(length);
        case 5:return approximate ? nuttall98_2(length) : nuttall98(length);
        default:return approximate ? mltsine_2(length) : mltsine(length);
    }
}
static void number(FILE *file,uint32_t value) {if(fwrite(&value,4,1,file)!=1) exit(3);}
int main(int argc,char **argv) {
    if(argc!=2) return 2;
    FILE *output=fopen(argv[1],"wb"); if(!output) return 3;
    fwrite("WIN1",1,4,output); number(output,sizeof(FP_TYPE)); number(output,112);
    const int lengths[]={2,3,5,8,16,31,128,1024};
    for(int kind=0;kind<7;kind++) for(int approximate=0;approximate<2;approximate++) for(int size=0;size<8;size++) {
        int length=lengths[size]; FP_TYPE *values=generate(kind,approximate,length); if(!values) return 3;
        number(output,kind); number(output,approximate); number(output,length);
        if(fwrite(values,sizeof(FP_TYPE),length,output)!=(size_t)length) return 3;
        free(values);
    }
    return fclose(output)!=0 ? 3 : 0;
}
