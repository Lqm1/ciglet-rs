#include <float.h>
#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f, uint32_t x) { if (fwrite(&x,4,1,f)!=1) exit(3); }
static void scalar(FILE *f, FP_TYPE x) { if (fwrite(&x,sizeof(x),1,f)!=1) exit(3); }
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *f=fopen(argv[1],"wb"); if (!f) return 3;
    fwrite("SMC1",1,4,f); integer(f,sizeof(FP_TYPE)); integer(f,48);
    FP_TYPE maximum=sizeof(FP_TYPE)==4 ? FLT_MAX : DBL_MAX;
    FP_TYPE tiny=sizeof(FP_TYPE)==4 ? (FP_TYPE)0x1p-149 : (FP_TYPE)0x1p-1074;
    FP_TYPE offset=sizeof(FP_TYPE)==4 ? (FP_TYPE)0x1p20 : (FP_TYPE)0x1p50;
    FP_TYPE epsilon=sizeof(FP_TYPE)==4 ? FLT_EPSILON : DBL_EPSILON;
    for (int kind=0;kind<48;++kind) {
        int n=kind==4 || kind==7 ? 3 : kind==6 ? 7 : kind<16 ? 4 : 3+(kind*7)%63;
        FP_TYPE x[65],y[65];
        for (int i=0;i<n;++i) {
            FP_TYPE v=i-1.5;
            x[i]=v; y[i]=-v;
            if(kind==0 || kind==10) { x[i]=offset+v; y[i]=offset-v; }
            if(kind==1) { x[i]=maximum; y[i]=maximum; }
            if(kind==2) { x[i]=(i%2 ? -1 : 1)*(maximum/2); y[i]=-x[i]; }
            if(kind==3) { x[i]=v*(sizeof(FP_TYPE)==4 ? 0x1p60 : 0x1p500); y[i]=v*(sizeof(FP_TYPE)==4 ? 0x1p-60 : 0x1p-500); }
            if(kind==4) { x[i]=(i-1)*tiny; y[i]=x[i]; }
            if(kind==5) { x[i]=(i<2 ? maximum/4 : tiny)*(i%2 ? -1 : 1); y[i]=(i<2 ? tiny : maximum/4)*(i%2 ? -1 : 1); }
            if(kind==6) { x[i]=i==0 ? -maximum : maximum; y[i]=-x[i]; }
            if(kind==7) { x[i]=i==0 ? maximum : i==1 ? -maximum : 1; y[i]=x[i]; }
            if(kind==8) { x[i]=i%2 ? -1 : 1; y[i]=i<2 ? 1 : -1; }
            if(kind==9) { x[i]=i%2 ? -0.0 : 0.0; y[i]=i; }
            if(kind==11) { x[i]=1+i*epsilon; y[i]=1+(3-i)*epsilon; }
            if(kind==12) { x[i]=(i%2 ? -1 : 1)*tiny; y[i]=(i%2 ? -1 : 1)*maximum; }
            if(kind==13) { FP_TYPE big=sizeof(FP_TYPE)==4 ? 0x1p60 : 0x1p500; x[i]=i==0 ? big : i==1 ? -big : 1; y[i]=i<2 ? big : i==2 ? 1 : -1; }
            if(kind==14) { x[i]=maximum/4+(i-1)*ldexp(maximum, sizeof(FP_TYPE)==4 ? -26 : -55); y[i]=x[i]; }
            if(kind==15) { x[i]=(i-1)*tiny; y[i]=2*x[i]; }
            if(kind>=16) {
                int range=sizeof(FP_TYPE)==4 ? 100 : 1800;
                int ex=(kind*31)%range-range/2, ey=(kind*73)%range-range/2;
                int a=((i*37+kind*13)%127)-63, b=((i*19+kind*29)%113)-56;
                x[i]=ldexp(a/64.0,ex); y[i]=ldexp(b/64.0,ey);
            }
        }
        integer(f,n);
        for(int i=0;i<n;++i) scalar(f,x[i]);
        for(int i=0;i<n;++i) scalar(f,y[i]);
        FP_TYPE vx=varfp(x,n),vy=varfp(y,n);
        FP_TYPE result[]={meanfp(x,n),vx,cov(x,y,n),corr(x,y,n)};
        double divisor=sqrt((FP_TYPE)(vx*vy));
        for(int k=0;k<4;++k) {
            int code=isfinite(result[k]) ? 0 : isnan(result[k]) ? 3 : result[k]>0 ? 1 : 2;
            if(k==3 && (!(vx>0) || !(vy>0) || !isfinite(divisor) || divisor==0 || !isfinite(result[k]))) code=4;
            integer(f,code); scalar(f,code==0 ? result[k] : 0);
        }
    }
    return fclose(f)!=0;
}
