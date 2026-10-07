#include <stdint.h>
#include "ciglet.h"
#include "polynomial_points.h"
static void number(FILE *f,uint32_t n) {if(fwrite(&n,4,1,f)!=1)exit(3);}
static void record(FILE *f,cplx *poly,int degree) {
 number(f,degree+1);fwrite(poly,sizeof(cplx),degree+1,f);
 cplx positions[]={c_cplx(0.25,0.2),c_cplx(-0.75,0.1),c_cplx(1,-0.3)};
 for(int i=0;i<3;i++){cplx value=cig_polyval(poly,degree+1,positions[i]);fwrite(&positions[i],sizeof(cplx),1,f);fwrite(&value,sizeof(cplx),1,f);}
 cplx *roots=cig_roots(poly,degree+1);fwrite(roots,sizeof(cplx),degree,f);free(roots);
}
int main(int argc,char **argv) {
 if(argc!=2)return 2;FILE *f=fopen(argv[1],"wb");if(!f)return 3;
 fwrite("POL1",1,4,f);number(f,sizeof(FP_TYPE));number(f,sizeof(FP_TYPE)==8?33:32);
 for(int degree=1;degree<=8;degree++)for(int shape=0;shape<4;shape++) {
  cplx poly[9]={0};poly[0]=c_cplx(1,0);
  for(int i=0;i<degree;i++) {
   cplx root=shape==0?c_cplx(-0.8+0.2*i,0):shape==1?c_cplx(-0.6+0.15*i,0.2+0.07*i):
      shape==2?c_cplx(0.1+0.7*circle_points[degree-1][i][0],0.7*circle_points[degree-1][i][1]):
      c_cplx(-0.5+0.13*i,(i%2?1:-1)*0.3);
   for(int j=i+1;j>0;j--)poly[j]=c_sub(poly[j],c_mul(root,poly[j-1]));
  }
  cplx scale=shape==1?c_cplx(0.75,0.25):c_cplx(1.25,0);
  for(int i=0;i<=degree;i++)poly[i]=c_mul(poly[i],scale);
  record(f,poly,degree);
 }
 if(sizeof(FP_TYPE)==8) {
  FP_TYPE real[]={1,-36,546,-4536,22449,-67284,118124,-109584,40320};cplx poly[9];
  for(int i=0;i<9;i++)poly[i]=c_cplx(real[i],0);
  record(f,poly,8);
 }
 number(f,61);
 for(int i=-30;i<=30;i++){FP_TYPE x=i*0.125, s=sin_1(x),c=cos_1(x);fwrite(&x,sizeof(x),1,f);fwrite(&s,sizeof(s),1,f);fwrite(&c,sizeof(c),1,f);}
 return fclose(f)!=0?3:0;
}
