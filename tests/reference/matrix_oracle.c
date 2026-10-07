#include <stdint.h>
#include "ciglet.h"
static void number(FILE *f,uint32_t n) {if(fwrite(&n,4,1,f)!=1)exit(3);}
static void values(FILE *f,FP_TYPE *a,int n) {if(fwrite(a,sizeof(FP_TYPE),n,f)!=(size_t)n)exit(3);}
int main(int argc,char **argv) {
 if(argc!=2)return 2;FILE *f=fopen(argv[1],"wb");if(!f)return 3;
 fwrite("MAT1",1,4,f);number(f,sizeof(FP_TYPE));
#ifdef CORRECTED
 number(f,1);
#else
 number(f,0);
#endif
 number(f,54);
 const int ms[]={1,2,5},ns[]={1,3,7},ls[]={1,2,4};
 for(int mi=0;mi<3;mi++)for(int ni=0;ni<3;ni++)for(int li=0;li<3;li++)for(int seed=0;seed<2;seed++) {
  int m=ms[mi],n=ns[ni],l=ls[li];FP_TYPE a[35],b[28],c[20],v[5];
  for(int i=0;i<m*n;i++)a[i]=((i*13+seed*7)%23-11)*0.125;
  for(int i=0;i<n*l;i++)b[i]=((i*7+seed*3)%17-8)*0.0625;
  cig_matmul(a,b,c,m,n,l);
#ifdef CORRECTED
  mvecmul(a,b,v,m,n);
#else
  cig_mvecmul(a,b,v,m,n);
#endif
  number(f,m);number(f,n);number(f,l);values(f,a,m*n);values(f,b,n*l);values(f,c,m*l);values(f,v,m);
 }
#ifdef CORRECTED
 number(f,12);const int rows[]={2,3,5},columns[]={1,2,4,7};
 for(int r=0;r<3;r++)for(int c=0;c<4;c++){int m=rows[r],n=columns[c];
#else
 number(f,5);const int sizes[]={1,2,3,5,8};
 for(int r=0;r<5;r++){int m=sizes[r],n=m;
#endif
  FP_TYPE a[64],v[8];int swaps[8];for(int i=0;i<m*n;i++)a[i]=(i-10)*0.25;
  for(int i=0;i<m;i++){v[i]=i+1;swaps[i]=(i*3+1)%m;}
  number(f,m);number(f,n);values(f,a,m*n);values(f,v,m);for(int i=0;i<m;i++)number(f,swaps[i]);
  permm(a,swaps,m,n);permv(v,swaps,m);values(f,a,m*n);values(f,v,m);
 }
#ifdef CORRECTED
 number(f,0);
#else
 number(f,20);const int orders[]={1,2,3,5,8};
 for(int size=0;size<5;size++)for(int kind=0;kind<4;kind++) {
  int n=orders[size];FP_TYPE a[64],b[8],x[8];
  for(int j=0;j<n;j++)for(int i=0;i<n;i++) {
   int row=kind==1?n-1-i:kind==3?(i+1)%n:i;
   FP_TYPE value=row==j?n+2:((row*11+j*3)%17-8)*0.03125;
   a[i+j*n]=kind==2?value*(j+1)*0.5:value;
  }
  for(int i=0;i<n;i++)x[i]=(i-2)*0.25;
  cig_mvecmul(a,x,b,n,n);number(f,n);values(f,a,n*n);values(f,b,n);
  int *swaps=ppivot(a,n);for(int i=0;i<n;i++)number(f,swaps[i]);values(f,a,n*n);
  permv(b,swaps,n);values(f,b,n);lu(a,n);values(f,a,n*n);lusolve(a,b,n);values(f,b,n);free(swaps);
 }
#endif
 return fclose(f)!=0?3:0;
}
