#include <stdint.h>
#include "ciglet.h"
static void integer(FILE* file, uint32_t value) { if (fwrite(&value, 4, 1, file) != 1) exit(3); }
static void bytes(FILE* file, void* value, size_t n) { if (fwrite(value, 1, n, file) != n) exit(3); }
int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* file = fopen(argv[1], "wb"); if (!file) return 3;
  fwrite("ARR1", 1, 4, file); integer(file, 240);
  const int rows[] = {1, 2, 3, 4, 7, 9}, columns[] = {1, 2, 3, 5, 8};
  for (int r = 0; r < 6; ++r) for (int c = 0; c < 5; ++c) for (int width = 1; width <= 8; width *= 2) for (int pattern = 0; pattern < 2; ++pattern) {
    int m = rows[r], n = columns[c]; size_t size = (size_t)m * n * width;
    unsigned char* input = malloc(size);
    for (size_t i = 0; i < size; ++i) input[i] = pattern == 0 ? (unsigned char)(i * 37 + 11) : (unsigned char)(255 - (i * 13 % 256));
    void** zero = malloc2d_(m, n, width), **original = reshape_(input, m, n, width);
    void** copy = copy2d_(original, m, n, width), **transpose = cig_transpose(original, m, n, width);
    if (!transpose) return 4;
    void* zero_flat = flatten_(zero, m, n, width), *copy_flat = flatten_(copy, m, n, width);
    void* reshape_flat = flatten_(original, m, n, width), *transpose_flat = flatten_(transpose, n, m, width);
    integer(file, m); integer(file, n); integer(file, width); bytes(file, input, size);
    bytes(file, zero_flat, size); bytes(file, copy_flat, size); bytes(file, reshape_flat, size); bytes(file, transpose_flat, size);
    free(input); free(zero_flat); free(copy_flat); free(reshape_flat); free(transpose_flat);
    free2d_(zero, m); free2d_(original, m); free2d_(copy, m); free2d_(transpose, n);
  }
  return fclose(file) != 0 ? 3 : 0;
}
