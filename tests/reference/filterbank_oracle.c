#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value,sizeof(value),1,file)!=1) exit(3);
}
static void scalar(FILE *file, FP_TYPE value) {
    if (fwrite(&value,sizeof(value),1,file)!=1) exit(3);
}
int main(int argc,char **argv) {
    if(argc!=2) return 2;
    FILE *file=fopen(argv[1],"wb"); if(!file) return 3;
    if(fwrite("FBK1",1,4,file)!=4) return 3;
    integer(file,sizeof(FP_TYPE)); integer(file,72);
    int bins[]={257,513,1025}, channels[]={4,20,32};
    FP_TYPE nyquists[]={8000,11025};
    for(int kind=0;kind<4;kind++)
    for(int size=0;size<3;size++)
    for(int count=0;count<3;count++)
    for(int rate=0;rate<2;rate++) {
        int nf=bins[size], nc=channels[count]; FP_TYPE nyq=nyquists[rate];
        FP_TYPE scale=0.85, width=120;
        filterbank *bank;
        if(kind==0) bank=cig_create_empty_filterbank(nf,nyq,nc);
        else if(kind==1) bank=cig_create_melfreq_filterbank(nf,nyq,nc,50,nyq,scale,width);
        else if(kind==2) bank=create_melfilterbank(nf,nyq,nc,50,nyq);
        else bank=cig_create_plp_filterbank(nf,nyq,nc);
        integer(file,kind);integer(file,nf);integer(file,nc);scalar(file,nyq);
        for(int channel=0;channel<nc;channel++) {
            if(bank->lower_idx[channel]<0 || bank->upper_idx[channel]>nf) return 4;
            integer(file,bank->lower_idx[channel]);integer(file,bank->upper_idx[channel]);
            for(int bin=0;bin<nf;bin++) scalar(file,bank->fresp[channel][bin]);
        }
        FP_TYPE *spectra[3];
        for(int scenario=0;scenario<3;scenario++) {
            spectra[scenario]=calloc(nf,sizeof(FP_TYPE)); if(!spectra[scenario]) return 3;
            for(int bin=0;bin<nf;bin++) {
                if(scenario==1) spectra[scenario][bin]=(bin%17+1)*0.125;
                else if(scenario==2) spectra[scenario][bin]=bin==nf/3?2.0:0.0;
            }
        }
        for(int compression=0;compression<2;compression++) {
            for(int scenario=0;scenario<3;scenario++) {
                FP_TYPE *output=cig_filterbank_spectrum(bank,spectra[scenario],2*(nf-1),2*nyq,compression);
                for(int channel=0;channel<nc;channel++) scalar(file,output[channel]);
                free(output);
            }
            FP_TYPE **output=cig_filterbank_spectrogram(bank,spectra,3,2*(nf-1),2*nyq,compression);
            for(int scenario=0;scenario<3;scenario++) {
                for(int channel=0;channel<nc;channel++) scalar(file,output[scenario][channel]);
                free(output[scenario]);
            }
            free(output);
        }
        for(int scenario=0;scenario<3;scenario++) free(spectra[scenario]);
        cig_delete_filterbank(bank);
    }
    return fclose(file)!=0?3:0;
}
