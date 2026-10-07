#include <stdint.h>
#include "ciglet.h"
static FP_TYPE function(FP_TYPE x, void *environment) {
 int kind=*(int*)environment;
 if(kind==0) return x*x-2;
 if(kind==1) return x*x*x-x-2;
 if(kind==2) return cos(x)-x;
 if(kind==3) return exp(x)-3;
 return x-(FP_TYPE)0.12345;
}
int main(int argc,char **argv) {
 if(argc!=2)return 2;FILE *f=fopen(argv[1],"wb");if(!f)return 3;
 uint32_t width=sizeof(FP_TYPE),count=25;fwrite("ROT1",1,4,f);fwrite(&width,4,1,f);fwrite(&count,4,1,f);
 for(int kind=0;kind<5;kind++)for(int range=0;range<5;range++) {
  uint32_t id=kind;FP_TYPE a=0,b=2+range;
  FP_TYPE answer=cig_fzero(function,a,b,&kind);
  fwrite(&id,4,1,f);fwrite(&a,sizeof(a),1,f);fwrite(&b,sizeof(b),1,f);fwrite(&answer,sizeof(answer),1,f);
 }
 return fclose(f)!=0?3:0;
}
