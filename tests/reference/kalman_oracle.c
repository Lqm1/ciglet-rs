#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *file, uint32_t value) {
    if(fwrite(&value, 4, 1, file) != 1) exit(3);
}
static void values(FILE *file, FP_TYPE *data, int count) {
    if(fwrite(data, sizeof(*data), count, file) != (size_t)count) exit(3);
}
int main(int argc, char **argv) {
    if(argc != 2) return 2;
    FILE *file = fopen(argv[1], "wb"); if(!file) return 3;
    if(fwrite("KLM1", 1, 4, file) != 4) return 3;
    integer(file, sizeof(FP_TYPE)); integer(file, 108);
    int lengths[] = {1, 2, 9, 32};
    for(int size = 0; size < 4; size++)
    for(int initial = 0; initial < 3; initial++)
    for(int pattern = 0; pattern < 3; pattern++)
    for(int noise = 0; noise < 3; noise++) {
        int n = lengths[size];
        FP_TYPE z[32], q[32], r[32], p[32], likelihood;
        for(int i = 0; i < n; i++) {
            z[i] = pattern == 0 ? 0 : pattern == 1 ? (i == 0 ? 0.75 : 0) : (i % 7 - 3) * 0.125;
            q[i] = noise == 1 ? 0.015625 + (i % 3) * 0.03125 : 0.03125;
            r[i] = noise == 2 ? 0 : noise == 1 ? 0.0625 + (i % 5) * 0.015625 : 0.125;
        }
        FP_TYPE x0 = initial == 0 ? 0 : initial == 1 ? z[0] : -0.5;
        integer(file, n); values(file, &x0, 1);
        values(file, z, n); values(file, q, n); values(file, r, n);
        FP_TYPE *filtered = cig_kalmanf1d(z, q, r, n, x0, p, &likelihood);
        FP_TYPE *smoothed = cig_kalmans1d(filtered, p, q, n);
        values(file, filtered, n); values(file, p, n); values(file, smoothed, n);
        values(file, &likelihood, 1);
        free(filtered); free(smoothed);
    }
    return fclose(file) != 0 ? 3 : 0;
}
