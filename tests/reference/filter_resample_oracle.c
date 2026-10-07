#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f,uint32_t x) { if(fwrite(&x,4,1,f)!=1) exit(3); }
static void scalar(FILE *f,FP_TYPE x) { if(fwrite(&x,sizeof(x),1,f)!=1) exit(3); }
static void values(FILE *f,FP_TYPE *x,int n) { for(int i=0;i<n;i++) scalar(f,x[i]); }
static FP_TYPE sample(int index,int pattern) {
    return pattern==0 ? 0 : pattern==1 ? (index==0 ? 0.75 : 0) : (index%7-3)*0.125;
}
int main(int argc,char **argv) {
    if(argc!=2) return 2;
    FILE *f=fopen(argv[1],"wb");if(!f) return 3;
    if(fwrite("FRS1",1,4,f)!=4) return 3;
    integer(f,sizeof(FP_TYPE));integer(f,300);integer(f,60);integer(f,105);
    int taps[]={2,3,8,31,32};
    char *windows[]={"boxcar","hanning","hamming","blackman","blackman_harris"};
    char *types[]={"lowpass","highpass","bandpass","bandstop"};
    FP_TYPE lows[]={0,0.125,0.23}, highs[]={0,0.375,0.5};
    for(int size=0;size<5;size++) for(int window=0;window<5;window++)
    for(int type=0;type<4;type++) for(int cutoff=0;cutoff<3;cutoff++) {
        int n=taps[size];FP_TYPE lo=lows[cutoff],hi=highs[cutoff];
        integer(f,n);integer(f,window);integer(f,type);scalar(f,lo);scalar(f,hi);
        FP_TYPE *h=cig_winfir(n,lo,hi,types[type],windows[window]);values(f,h,n);free(h);
    }
    int lengths[]={1,2,5,32,65}, kernels[]={1,2,7,32};
    for(int size=0;size<5;size++) for(int k=0;k<4;k++) for(int pattern=0;pattern<3;pattern++) {
        int nx=lengths[size],nh=kernels[k];FP_TYPE *x=calloc(nx,sizeof(FP_TYPE)),*h=calloc(nh,sizeof(FP_TYPE));
        integer(f,nx);integer(f,nh);
        for(int i=0;i<nx;i++) x[i]=sample(i,pattern);
        for(int i=0;i<nh;i++) h[i]=(i%5-2)*0.0625;
        values(f,x,nx);values(f,h,nh);
        FP_TYPE *y=cig_convolution(x,h,nx,nh);values(f,y,nx+nh-1);free(y);free(x);free(h);
    }
    FP_TYPE ratios[]={0.25,0.5,0.9,1,1.1,1.5,2};
    for(int size=0;size<5;size++) for(int ratio=0;ratio<7;ratio++) for(int pattern=0;pattern<3;pattern++) {
        int nx=lengths[size],ny=0;FP_TYPE *x=calloc(nx,sizeof(FP_TYPE));
        integer(f,nx);scalar(f,ratios[ratio]);
        for(int i=0;i<nx;i++) x[i]=sample(i,pattern);
        values(f,x,nx);FP_TYPE *y=cig_rresample(x,nx,ratios[ratio],&ny);
        integer(f,ny);values(f,y,ny);free(y);free(x);
    }
    return fclose(f)!=0 ? 3 : 0;
}
