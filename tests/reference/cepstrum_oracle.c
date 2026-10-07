#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file,uint32_t value) { if(fwrite(&value,sizeof(value),1,file)!=1) exit(3); }
static void scalar(FILE *file,FP_TYPE value) { if(fwrite(&value,sizeof(value),1,file)!=1) exit(3); }
int main(int argc,char **argv) {
    if(argc!=2) return 2;
    FILE *file=fopen(argv[1],"wb");if(!file) return 3;
    if(fwrite("CEP1",1,4,file)!=4) return 3;
    integer(file,sizeof(FP_TYPE));integer(file,48);
    int sizes[]={1,2,5,12,20,40};
    for(int size=0;size<6;size++)
    for(int order_case=0;order_case<4;order_case++)
    for(int include_dc=0;include_dc<2;include_dc++) {
        int bands=sizes[size];int orders[]={0,1,bands-1,bands+5};int order=orders[order_case];
        integer(file,bands);integer(file,order);integer(file,include_dc);
        FP_TYPE *input[3];
        for(int frame=0;frame<3;frame++) {
            input[frame]=calloc(bands,sizeof(FP_TYPE));if(!input[frame]) return 3;
            for(int band=0;band<bands;band++) {
                if(frame==1) input[frame][band]=-15.0;
                else if(frame==2) input[frame][band]=(band%7-3)*0.25;
                scalar(file,input[frame][band]);
            }
        }
        for(int frame=0;frame<3;frame++) {
            FP_TYPE *output=be2cc(input[frame],bands,order,include_dc);
            for(int band=0;band<bands;band++) scalar(file,output[band]);
            free(output);
        }
        FP_TYPE **output=be2ccgm(input,3,bands,order,include_dc);
        for(int frame=0;frame<3;frame++) {
            for(int band=0;band<bands;band++) scalar(file,output[frame][band]);
            free(output[frame]);free(input[frame]);
        }
        free(output);
    }
    return fclose(file)!=0?3:0;
}
