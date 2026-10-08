/* Compile with optimization while retaining every assertion. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ciglet_rs.h"

typedef struct Sink { unsigned char bytes[32768]; uintptr_t count, writes, flushes; uint32_t failure; } Sink;
static uint32_t write_bytes(void *context, const uint8_t *bytes, uintptr_t count, uintptr_t *written) {
    Sink *sink = context;
    ++sink->writes;
    if (sink->failure == 1) return 9;
    if (sink->failure == 2) { *written = count + 1; return 0; }
    if (sink->failure == 3) { *written = 0; return 0; }
    if (sink->failure == 4 && sink->count >= 8) return 9;
    assert(count > 0 && sink->count < sizeof sink->bytes);
    sink->bytes[sink->count++] = bytes[0];
    *written = 1;
    return 0;
}
static uint32_t flush_bytes(void *context) {
    Sink *sink = context;
    ++sink->flushes;
    return sink->failure == 5 ? 9 : 0;
}
static CigletRsWriteStream stream(Sink *sink) {
    CigletRsWriteStream result = {sink, write_bytes, flush_bytes};
    return result;
}
static void append(Sink *sink, CigletRsBytes **owner) {
    uintptr_t size = 0;
    assert(ciglet_rs_bytes_length(*owner, &size) == 0);
    assert(size <= sizeof sink->bytes - sink->count);
    assert(ciglet_rs_bytes_copy(*owner, 0, sink->bytes + sink->count, size) == 0);
    sink->count += size;
    assert(ciglet_rs_bytes_release(owner) == 0 && *owner == NULL);
}
static void compare_fixture(const Sink *memory, const Sink *external, const char *path) {
    unsigned char expected[32768];
    FILE *file = fopen(path, "rb");
    assert(file != NULL);
    size_t count = fread(expected, 1, sizeof expected, file);
    assert(!ferror(file) && feof(file));
    assert(fclose(file) == 0);
    assert(memory->count == count && external->count == count);
    assert(memcmp(memory->bytes, expected, count) == 0);
    assert(memcmp(external->bytes, expected, count) == 0);
    assert(external->writes == count && external->flushes == 11);
}
#define CHECK_PRECISION(SUFFIX, TYPE, HANDLE, MAXIMUM, BITS, FIXTURE) \
static void check_##SUFFIX(void) { \
    BITS bits = 1; TYPE tiny; memcpy(&tiny, &bits, sizeof tiny); \
    TYPE y[] = {0.0, -0.0, 1.23456789, -0.0000005, 1e20, 1e-20, MAXIMUM, tiny, NAN, -NAN, INFINITY, -INFINITY}; \
    TYPE x[12]; for (int i = 0; i < 12; ++i) x[i] = (TYPE)(i * 0.25 - 1.0); \
    const uint8_t colors[] = "brgkcy?"; \
    Sink memory = {0}, external = {0}; CigletRsBytes *owner = NULL; \
    for (int i = 0; i < 7; ++i) { \
        const TYPE *xp = i % 2 ? x : NULL; uintptr_t xc = i % 2 ? 13 : 0; \
        assert(ciglet_rs_plot_line_bytes_##SUFFIX(xp, xc, y, 12, colors[i], &owner) == 0); append(&memory, &owner); \
        assert(ciglet_rs_plot_line_stream_##SUFFIX(stream(&external), xp, xc, y, 12, colors[i]) == 0); \
    } \
    assert(ciglet_rs_plot_line_bytes_##SUFFIX(NULL, 0, NULL, 0, 'b', &owner) == 0); append(&memory, &owner); \
    assert(ciglet_rs_plot_line_stream_##SUFFIX(stream(&external), NULL, 0, NULL, 0, 'b') == 0); \
    for (int i = 0; i < 3; ++i) { \
        HANDLE *array = NULL; \
        if (i == 0) { const TYPE *rows[] = {y, y + 6}; uintptr_t counts[] = {6, 6}; \
            assert(ciglet_rs_array_from_rows_##SUFFIX(rows, counts, 2, &array) == 0); \
        } else assert(ciglet_rs_array_from_slice_##SUFFIX(i == 1 ? 0 : 2, i == 1 ? 3 : 0, NULL, 0, &array) == 0); \
        assert(ciglet_rs_plot_image_bytes_##SUFFIX(array, &owner) == 0); append(&memory, &owner); \
        assert(ciglet_rs_plot_image_stream_##SUFFIX(stream(&external), array) == 0); \
        assert(ciglet_rs_array_release_##SUFFIX(&array) == 0 && array == NULL); \
    } \
    compare_fixture(&memory, &external, FIXTURE); \
}
CHECK_PRECISION(f32, float, CigletRsArrayF32, FLT_MAX, uint32_t, "tests/fixtures/plotting-float.txt")
CHECK_PRECISION(f64, double, CigletRsArrayF64, DBL_MAX, uint64_t, "tests/fixtures/plotting-double.txt")

int main(void) {
    check_f32(); check_f64();
    const uint8_t configuration[] = {'s','e','t','\n',0,255};
    Sink memory = {0}, external = {0}; CigletRsBytes *owner = NULL;
    assert(ciglet_rs_plot_configuration_bytes(configuration, sizeof configuration, &owner) == 0);
    append(&memory, &owner);
    assert(ciglet_rs_plot_configuration_stream(stream(&external), configuration, sizeof configuration) == 0);
    assert(memory.count == sizeof configuration && memcmp(memory.bytes, configuration, sizeof configuration) == 0);
    assert(external.count == memory.count && memcmp(external.bytes, memory.bytes, memory.count) == 0);
    assert(ciglet_rs_plot_configuration_stream(stream(&external), NULL, 0) == 0 && external.flushes == 2);
    assert(ciglet_rs_plot_configuration_bytes(NULL, 0, &owner) == 0);
    assert(ciglet_rs_bytes_release(&owner) == 0);
    for (uint32_t failure = 1; failure <= 5; ++failure) {
        Sink failed = {0}; failed.failure = failure;
        assert(ciglet_rs_plot_configuration_stream(stream(&failed), (const uint8_t *)"0123456789", 10) == 3);
        if (failure <= 3) assert(failed.count == 0);
        if (failure == 4) assert(failed.count == 8);
        assert(failed.flushes == (failure == 5));
    }
    Sink invalid = {0}; double y[] = {1, 2};
    assert(ciglet_rs_plot_line_stream_f64(stream(&invalid), y, 1, y, 2, 'r') == 2);
    CigletRsWriteStream missing = stream(&invalid); missing.flush = NULL;
    assert(ciglet_rs_plot_configuration_stream(missing, NULL, 0) == 1);
    assert(invalid.writes == 0 && invalid.flushes == 0);
    return 0;
}
