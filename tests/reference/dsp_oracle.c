#include "ciglet.h"

static void emit(const char *name, FP_TYPE *values, int count) {
    printf("%s", name);
    for (int index = 0; index < count; index++) printf(" %.9g", values[index]);
    printf("\n");
}
int main(void) {
    FP_TYPE x[] = { -2, 0.5, 4, 1.25, -0.75, 3 };
    FP_TYPE y[] = { 1, -2, 0.25, 4, 0.5, -1 };
    FP_TYPE statistics[] = { sumfp(x, 6), sumsqrfp(x, 6), meanfp(x, 6), varfp(x, 6), cov(x, y, 6), corr(x, y, 6), dot(x, y, 6) };
    emit("statistics", statistics, 7);
    FP_TYPE *window;
    window = hanning(8); emit("hann", window, 8); free(window);
    window = hamming(8); emit("hamming", window, 8); free(window);
    window = blackman(8); emit("blackman", window, 8); free(window);
    window = blackman_harris(8); emit("blackman_harris", window, 8); free(window);
    window = nuttall98(8); emit("nuttall98", window, 8); free(window);
    window = mltsine(8); emit("mlt_sine", window, 8); free(window);
    window = fetch_frame(x, 6, 0, 8); emit("frame_left", window, 8); free(window);
    window = fetch_frame(x, 6, 5, 7); emit("frame_right", window, 7); free(window);
    return 0;
}
