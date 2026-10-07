/* Compare unchanged cig_filter and filtfilt with a zero-padded startup adapter.
 * Padding bounds the original generic startup loop for short inputs. */
#include <stdint.h>
#include "ciglet.h"
static void integer(FILE *f, uint32_t value) {
    if(fwrite(&value, sizeof(value), 1, f) != 1) exit(3);
}
static void values(FILE *f, FP_TYPE *x, int n) {
    if(fwrite(x, sizeof(*x), n, f) != (size_t)n) exit(3);
}
static FP_TYPE *bounded(FP_TYPE *b, int nb, FP_TYPE *a, int na,
                        FP_TYPE *x, int nx, int corrected) {
    int prefix = corrected && na == nb && na >= 2 && na <= 6 ? na - 1 : 0;
    int count = max(nx + prefix, max(na, nb));
    FP_TYPE *padded = calloc(count, sizeof(FP_TYPE));
    for(int i = 0; i < nx; i++) padded[prefix + i] = x[i];
    FP_TYPE *filtered = cig_filter(b, nb, a, na, padded, count);
    FP_TYPE *result = calloc(max(nx, 1), sizeof(FP_TYPE));
    for(int i = 0; i < nx; i++) result[i] = filtered[prefix + i];
    free(padded); free(filtered);
    return result;
}
int main(int argc, char **argv) {
    if(argc != 2) return 2;
    FILE *output = fopen(argv[1], "wb"); if(!output) return 3;
    if(fwrite("IIR1", 1, 4, output) != 4) return 3;
    integer(output, sizeof(FP_TYPE)); integer(output, 210);
    int shapes[][2] = {{1,1},{1,2},{2,1},{2,2},{3,3},{4,4},{5,5},
                      {6,6},{7,7},{8,8},{4,7},{7,4},{1,8},{8,1}};
    int lengths[] = {0,1,2,7,32};
    for(int shape = 0; shape < 14; shape++)
    for(int size = 0; size < 5; size++)
    for(int pattern = 0; pattern < 3; pattern++) {
        int nb = shapes[shape][0], na = shapes[shape][1], nx = lengths[size];
        FP_TYPE b[8] = {0}, a[8] = {0}, x[32] = {0};
        a[0] = 1;
        for(int k = 1; k < na; k++) a[k] = (k % 3 - 1) * 0.0625;
        for(int k = 0; k < nb; k++) b[k] = (k % 4 - 1) * 0.125;
        for(int k = 0; k < nx; k++)
            x[k] = pattern == 0 ? 0 : pattern == 1 ? (k == 0 ? 0.75 : 0) : (k % 7 - 3) * 0.125;
        integer(output, nb); integer(output, na); integer(output, nx);
        values(output, b, nb); values(output, a, na); values(output, x, nx);
        for(int corrected = 0; corrected < 2; corrected++) {
            FP_TYPE *filtered = bounded(b, nb, a, na, x, nx, corrected);
            values(output, filtered, nx);
            FP_TYPE reversed[32] = {0};
            for(int k = 0; k < nx; k++) reversed[k] = filtered[nx - k - 1];
            FP_TYPE *backward = bounded(b, nb, a, na, reversed, nx, corrected);
            for(int k = 0; k < nx; k++) reversed[k] = backward[nx - k - 1];
            values(output, reversed, nx);
            free(filtered); free(backward);
        }
    }
    return fclose(output) != 0 ? 3 : 0;
}
