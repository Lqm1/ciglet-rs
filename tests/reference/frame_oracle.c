#include <stdint.h>
#include "ciglet.h"

static void integer(FILE *file, uint32_t value) {
    if (fwrite(&value, 4, 1, file) != 1) exit(3);
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb");
    if (!file) return 3;
    FP_TYPE input[] = {-2, 0.5, 4, 1.25, -0.75, 3};
    int centers[] = {-3, 0, 2, 5, 9};
    int lengths[] = {0, 1, 2, 5, 8};
    fwrite("FRM1", 1, 4, file); integer(file, sizeof(FP_TYPE)); integer(file, 25);
    for (int c = 0; c < 5; ++c) for (int n = 0; n < 5; ++n) {
        int length = lengths[n];
        FP_TYPE *output = fetch_frame(input, 6, centers[c], length);
        integer(file, (uint32_t)centers[c]); integer(file, length);
        if (length && fwrite(output, sizeof(FP_TYPE), length, file) != (size_t)length) return 3;
        free(output);
    }
    return fclose(file) != 0 ? 3 : 0;
}
