/* Invoke every native Figure export against actual Gnuplot SVG rendering. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ciglet_rs.h"

static CigletRsPath path(const char *text) {
    _Alignas(8) CigletRsPath value = {(const uint8_t *)text, strlen(text), CIGLET_RS_PATH_UTF8};
    return value;
}
static void configuration(CigletRsFigure *figure, const char *text) {
    assert(ciglet_rs_figure_configuration(figure, (const uint8_t *)text, strlen(text)) == 0);
}
static void output_file(CigletRsFigure *figure, const char *directory, int mode, const char *kind, char *filename, size_t size) {
    int n = snprintf(filename, size, "%s/c-figure-%d-%s.svg", directory, mode, kind);
    assert(n > 0 && (size_t)n < size);
    char escaped[4096], command[8192]; size_t used = 0;
    for (size_t i = 0; filename[i]; ++i) {
        char ch = filename[i] == '\\' ? '/' : filename[i];
        assert(used + 2 < sizeof escaped);
        if (ch == '"') escaped[used++] = '\\';
        escaped[used++] = ch;
    }
    escaped[used] = 0;
    n = snprintf(command, sizeof command, "set output \"%s\"\n", escaped);
    assert(n > 0 && (size_t)n < sizeof command);
    configuration(figure, command);
}
static void verify_svg(const char *filename, int image) {
    FILE *file = fopen(filename, "rb"); assert(file != NULL);
    char bytes[262144]; size_t count = fread(bytes, 1, sizeof bytes - 1, file);
    assert(!ferror(file) && feof(file)); assert(fclose(file) == 0); bytes[count] = 0;
    assert(strstr(bytes, "<svg") != NULL);
    if (!image) assert(strstr(bytes, "rgb(255,   0,   0)") != NULL);
    else {
        char *group = strstr(bytes, "id=\"gnuplot_plot_1\""); assert(group != NULL);
        char *end = strstr(group, "</g>"); assert(end != NULL); *end = 0;
        int polygons = 0; const char *cursor = group;
        while ((cursor = strstr(cursor, "<polygon ")) != NULL) { ++polygons; ++cursor; }
        assert(strstr(group, "<image ") != NULL || polygons == 6);
    }
    assert(remove(filename) == 0);
}
static void empty_bytes(CigletRsBytes **owner) {
    uintptr_t length = 99; assert(ciglet_rs_bytes_length(*owner, &length) == 0 && length == 0);
    assert(ciglet_rs_bytes_release(owner) == 0 && *owner == NULL);
}
int main(int argc, char **argv) {
    assert(argc == 3);
    const char *program = argv[1], *directory = argv[2];
    for (int mode = 0; mode < 3; ++mode) {
        CigletRsFigure *figure = NULL;
        if (mode == 0) assert(ciglet_rs_figure_open(&figure) == 0);
        else if (mode == 1) assert(ciglet_rs_figure_open_with(path(program), &figure) == 0);
        else {
            _Alignas(8) CigletRsPath value = path("C caller");
            _Alignas(8) CigletRsPlotEnvironment env = {path("CIGLET_ABI_CALLER"), &value};
            _Alignas(8) CigletRsPath cwd = path(directory);
            _Alignas(8) CigletRsPlotProcessOptions options = {0};
            options.program = path(program); options.environment = &env; options.environment_count = 1;
            options.directory = &cwd; options.stdout_mode = CIGLET_RS_PLOT_CAPTURE; options.stderr_mode = CIGLET_RS_PLOT_CAPTURE;
            assert(ciglet_rs_figure_spawn(options, &figure) == 0);
        }
        uint32_t id = 0; assert(ciglet_rs_figure_id(figure, &id) == 0 && id > 0);
        CigletRsFigure *original = figure;
        assert(ciglet_rs_figure_close(&figure, NULL) == 1 && figure == original);
        assert(ciglet_rs_figure_configuration(figure, NULL, 1) == 1);
        assert(ciglet_rs_figure_configuration(figure, NULL, 0) == 0);
        configuration(figure, "set terminal svg size 320,240\n");
        char files[4][2048];
        float f32[] = {0, 1, 0, -1}; _Alignas(8) double f64[] = {0, 1, 0, -1};
        output_file(figure, directory, mode, "line32", files[0], sizeof files[0]);
        assert(ciglet_rs_figure_line_f32(figure, NULL, 0, f32, 4, 'r') == 0);
        output_file(figure, directory, mode, "line64", files[1], sizeof files[1]);
        assert(ciglet_rs_figure_line_f64(figure, NULL, 0, f64, 4, 'r') == 0);
        float pixels32[] = {0, 1, 2, 3, 4, 5}; _Alignas(8) double pixels64[] = {0, 1, 2, 3, 4, 5};
        CigletRsArrayF32 *array32 = NULL; CigletRsArrayF64 *array64 = NULL;
        assert(ciglet_rs_array_from_slice_f32(2, 3, pixels32, 6, &array32) == 0);
        assert(ciglet_rs_array_from_slice_f64(2, 3, pixels64, 6, &array64) == 0);
        output_file(figure, directory, mode, "image32", files[2], sizeof files[2]);
        assert(ciglet_rs_figure_image_f32(figure, array32) == 0);
        output_file(figure, directory, mode, "image64", files[3], sizeof files[3]);
        assert(ciglet_rs_figure_image_f64(figure, array64) == 0);
        assert(ciglet_rs_array_release_f32(&array32) == 0); assert(ciglet_rs_array_release_f64(&array64) == 0);
        configuration(figure, "unset output\n");
        _Alignas(8) CigletRsPlotExitStatus status = {0};
        if (mode < 2) assert(ciglet_rs_figure_close(&figure, &status) == 0);
        else {
            CigletRsBytes *out = NULL, *err = NULL;
            assert(ciglet_rs_figure_close_output(&figure, &status, &out, &err) == 0);
            empty_bytes(&out); empty_bytes(&err);
        }
        assert(figure == NULL && status.success == 1 && status.has_code == 1 && status.code == 0 && status.raw_status == 0);
#ifdef _WIN32
        assert(status.raw_format == 1);
#else
        assert(status.raw_format == 0);
#endif
        for (int i = 0; i < 4; ++i) verify_svg(files[i], i >= 2);
    }
    CigletRsFigure *figure = NULL;
    assert(ciglet_rs_figure_open_with(path(program), &figure) == 0);
    configuration(figure, "set terminal unknown\n");
    assert(ciglet_rs_figure_release(&figure) == 0 && figure == NULL);
    assert(ciglet_rs_figure_release(&figure) == 0);
    assert(ciglet_rs_figure_open_with(path("missing-program-abi-figure"), &figure) == 3 && figure == NULL);
    return 0;
}
