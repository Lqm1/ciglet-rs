/* Include the unchanged implementation to inspect its private LF parameters. */
#include "ciglet.c"
#include <stdint.h>

static void record(FILE* output, lfmodel model) {
  lfparam p = lfparam_from_lfmodel(model);
  FP_TYPE fields[] = {model.T0, model.te, model.tp, model.ta, model.Ee,
    p.T0, p.Te, p.Tp, p.Ta, p.wg, p.sin_wgTe, p.cos_wgTe, p.e, p.A, p.a,
    p.E0, p.scale, efunc(p.e, &p), afunc(p.a, &p), aderiv(p.a, &p)};
  fwrite(fields, sizeof(FP_TYPE), 20, output);
}

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  FILE* output = fopen(argv[1], "wb");
  if (!output) return 3;
  uint32_t width = sizeof(FP_TYPE), count = 208;
  fwrite("LFP1", 1, 4, output);
  fwrite(&width, 4, 1, output);
  fwrite(&count, 4, 1, output);
  FP_TYPE shapes[] = {0.01, 0.05, 0.1, 0.209999, 0.21, 0.210001,
    0.5, 1, 2, 2.699999, 2.7, 2.700001, 3, 5, 10, 100};
  FP_TYPE periods[] = {0.00001, 0.001, 0.00125, 0.005, 0.1};
  for (int s = 0; s < 16; ++s)
    for (int t = 0; t < 5; ++t)
      record(output, lfmodel_from_rd(shapes[s], periods[t], 1));
  FP_TYPE closures[] = {0.2, 0.5, 0.8, 0.95};
  FP_TYPE peaks[] = {0.500001, 0.55, 0.8, 0.999999};
  FP_TYPE returns[] = {0.000001, 0.01, 0.5, 0.999999};
  for (int c = 0; c < 4; ++c)
    for (int p = 0; p < 4; ++p)
      for (int r = 0; r < 4; ++r)
        for (int t = 0; t < 2; ++t) {
          lfmodel model = {.T0 = t == 0 ? 0.001 : 0.005,
            .te = closures[c], .tp = closures[c] * peaks[p],
            .ta = (1 - closures[c]) * returns[r], .Ee = 1};
          record(output, model);
        }
  return fclose(output) != 0 ? 3 : 0;
}
