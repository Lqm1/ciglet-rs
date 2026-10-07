#include "ciglet.h"
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FP_TYPE input[32] = {0};
  FP_TYPE* result = argv[1][0] == 'z' ? rceps(input, 0) : irceps(input, 32, 8);
  free(result);
  return 0;
}
