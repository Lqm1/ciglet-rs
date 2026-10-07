#include "ciglet.h"
int main(int argc,char **argv){
 if(argc!=2)return 2;
 if(argv[1][0]=='i'){int *x=iota(-2000000000,2000000000,3);volatile int value=x[2];free(x);return value==0;}
 FP_TYPE *x=calloc(0,sizeof(FP_TYPE)),*y=NULL;
 if(argv[1][0]=='d')y=diff(x,0);
 else if(argv[1][0]=='c')y=cumsum(x,0);
 else if(argv[1][0]=='u')y=unwrap(x,0);
 else if(argv[1][0]=='s')complete_symm(x,0);
 else complete_asymm(x,0);
 volatile FP_TYPE value=y?y[0]:x[0];free(y);free(x);return value==0;
}
