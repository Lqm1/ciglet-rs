#include "ciglet.h"
int main(int argc,char **argv) {
 FP_TYPE *a=malloc(6*sizeof(FP_TYPE));FP_TYPE x[3]={1,2,3},b[2];int swaps[2]={1,1};
 for(int i=0;i<6;i++)a[i]=i+1;
 if(argc==2 && argv[1][0]=='p')permm(a,swaps,2,3);
 else mvecmul(a,x,b,2,3);
 free(a);return 0;
}
