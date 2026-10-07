#include <float.h>
#include "ciglet.h"
int main(int argc,char **argv){
 if(argc!=2)return 2;
 if(argv[1][0]=='d'){
  FP_TYPE input[3]={1,1,NAN},*correlation=NULL,*a=flpc(input,3,1,&correlation);
  int valid=a[0]==1&&a[1]==0&&correlation[0]==1&&correlation[1]==0;free(a);free(correlation);return valid?0:1;
 }
 if(argv[1][0]=='m'){FP_TYPE value=1,*a=flpc(&value,1,1,NULL);free(a);return 0;}
 if(argv[1][0]=='c'){FP_TYPE coefficients[17];for(int i=0;i<17;i++)coefficients[i]=1;FP_TYPE *s=lpspec(coefficients,1,17,4);free(s);return 0;}
 FP_TYPE maximum=sizeof(FP_TYPE)==4?FLT_MAX:DBL_MAX;
 FP_TYPE tiny=sizeof(FP_TYPE)==4?nextafterf(0,1):nextafter(0,1);
 FP_TYPE *large=lpspec(&maximum,maximum,1,4),*small=lpspec(&tiny,tiny,1,4);
 FP_TYPE pole[2]={1,-1},*zero=lpspec(pole,0,2,4);
 int invalid=large[0]==0&&isinf(small[0])&&isnan(zero[0]);free(large);free(small);free(zero);
 return invalid?0:1;
}
