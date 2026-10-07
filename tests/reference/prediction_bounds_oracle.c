#include "ciglet.h"
int main(int argc,char **argv){
 if(argc!=2)return 2;
 if(argv[1][0]=='s'){
  FP_TYPE correlation[3]={0,0,0};FP_TYPE *a=cig_levinson(correlation,3);
  int invalid=!isfinite(a[1])||!isfinite(a[2]);free(a);return invalid?0:1;
 }
 int n=argv[1][0]=='0'?0:1;
 FP_TYPE *correlation=calloc(n,sizeof(FP_TYPE));if(n)correlation[0]=1;
 FP_TYPE *a=cig_levinson(correlation,n);volatile FP_TYPE value=a[0];free(a);free(correlation);return value==0;
}
