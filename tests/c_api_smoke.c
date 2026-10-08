/* Link this independent caller against the generated C interface. */
#ifdef NDEBUG
/* Zig's optimized modes define NDEBUG. Always execute this caller's checks. */
#undef NDEBUG
#endif
#include "ciglet_rs.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <string.h>

struct WindowContext { size_t calls; size_t fail_on; int nonfinite; double angles[16]; };
static uint32_t trig_f32(double angle, void *opaque, float *output) {
    struct WindowContext *context = opaque;
    if (context->calls < 16) context->angles[context->calls] = angle;
    ++context->calls;
    if (context->calls == context->fail_on) return 9;
    *output = context->nonfinite ? NAN : 0;
    return 0;
}
static uint32_t trig_f64(double angle, void *opaque, double *output) {
    struct WindowContext *context = opaque;
    if (context->calls < 16) context->angles[context->calls] = angle;
    ++context->calls;
    if (context->calls == context->fail_on) return 9;
    *output = context->nonfinite ? NAN : 0;
    return 0;
}

#define CHECK_STATISTICS(type, suffix) do { \
    type data[] = {1, 3, 5}; \
    type result = -71; \
    assert(ciglet_rs_sum_##suffix(data, 3, &result) == 0 && result == 9); \
    assert(ciglet_rs_sum_squares_##suffix(data, 3, &result) == 0 && result == 35); \
    assert(ciglet_rs_mean_##suffix(data, 3, &result) == 0 && result == 3); \
    assert(ciglet_rs_median_##suffix(data, 3, &result) == 0 && result == 3); \
    assert(ciglet_rs_array_minimum_##suffix(data, 3, &result) == 0 && result == 1); \
    assert(ciglet_rs_array_maximum_##suffix(data, 3, &result) == 0 && result == 5); \
    assert(ciglet_rs_variance_##suffix(data, 3, &result) == 0); \
    assert(result > 2.6666 && result < 2.6667); \
    for (uint32_t policy = 0; policy < 2; ++policy) { \
        assert(ciglet_rs_mean_with_##suffix(data, 3, policy, &result) == 0 && result == 3); \
        assert(ciglet_rs_variance_with_##suffix(data, 3, policy, &result) == 0); \
        assert(result > 2.6666 && result < 2.6667); \
    } \
    result = -71; \
    assert(ciglet_rs_mean_with_##suffix(data, 3, 2, &result) == 3 && result == -71); \
    assert(ciglet_rs_linear_grid_##suffix(-1, 1, data, 3) == 0); \
    assert(data[0] == -1 && data[1] == 0 && data[2] == 1); \
} while (0)

int main(void) {
    float single = 9.0f;
    double wide = 9.0;
    uint32_t single_bits = 0;
    uint64_t wide_bits = 0;
    const uint32_t nan_bits = UINT32_C(0x7fc12345);
    float nan_value;
    memcpy(&nan_value, &nan_bits, sizeof nan_value);
    assert(ciglet_rs_abi_version() == 1);
    assert(ciglet_rs_minimum_f32(0.0f, -0.0f, &single) == 0);
    memcpy(&single_bits, &single, sizeof single_bits);
    assert(single_bits == UINT32_C(0x80000000));
    assert(ciglet_rs_maximum_f32(1.0f, nan_value, &single) == 0);
    memcpy(&single_bits, &single, sizeof single_bits);
    assert(single_bits == nan_bits);
    assert(ciglet_rs_minimum_f64(3.0, 2.0, &wide) == 0 && wide == 2.0);
    assert(ciglet_rs_maximum_f64(-0.0, 0.0, &wide) == 0);
    memcpy(&wide_bits, &wide, sizeof wide_bits);
    assert(wide_bits == 0);
    assert(ciglet_rs_minimum_f32(1.0f, 2.0f, NULL) == 1);
    float samples[] = {1.0f, 3.0f, 6.0f};
    assert(ciglet_rs_difference_f32(samples, 3, samples, 3) == 0);
    assert(samples[0] == 1.0f && samples[1] == 2.0f && samples[2] == 3.0f);
    assert(ciglet_rs_cumulative_sum_f32(samples, 3, samples, 3) == 0);
    assert(samples[2] == 6.0f);
    assert(ciglet_rs_reverse_f32(samples, 3, samples, 3) == 0);
    assert(samples[0] == 6.0f && samples[2] == 1.0f);
    double doubles[] = {1.0, 3.0, 6.0};
    assert(ciglet_rs_difference_f64(doubles, 3, doubles, 3) == 0);
    assert(ciglet_rs_cumulative_sum_f64(doubles, 3, doubles, 3) == 0);
    assert(ciglet_rs_reverse_f64(doubles, 3, doubles, 3) == 0);
    assert(doubles[0] == 6.0 && doubles[2] == 1.0);
    assert(ciglet_rs_difference_f32(NULL, 0, NULL, 0) == 0);
    assert(ciglet_rs_difference_f32(samples, 3, samples, 2) == 2);
    CHECK_STATISTICS(float, f32);
    CHECK_STATISTICS(double, f64);
    #define CHECK_PAIR(type, suffix) do { \
        type left[] = {1, 2, 3}; \
        type right[] = {2, 4, 6}; \
        type result = -71; \
        assert(ciglet_rs_dot_##suffix(left, 3, right, 3, &result) == 0 && result == 28); \
        assert(ciglet_rs_covariance_##suffix(left, 3, right, 3, &result) == 0); \
        assert(result > 1.3333 && result < 1.3334); \
        assert(ciglet_rs_correlation_##suffix(left, 3, right, 3, &result) == 0); \
        assert(result > 0.999999 && result < 1.000001); \
        for (uint32_t policy = 0; policy < 2; ++policy) { \
            assert(ciglet_rs_covariance_with_##suffix(left, 3, right, 3, policy, &result) == 0); \
            assert(result > 1.3333 && result < 1.3334); \
            assert(ciglet_rs_correlation_with_##suffix(left, 3, right, 3, policy, &result) == 0); \
            assert(result > 0.999999 && result < 1.000001); \
        } \
        result = -71; \
        assert(ciglet_rs_covariance_with_##suffix(left, 3, right, 3, 2, &result) == 3 && result == -71); \
        assert(ciglet_rs_dot_##suffix(left, 3, right, 2, &result) == 2 && result == -71); \
        type lags[4]; \
        assert(ciglet_rs_cross_correlation_##suffix(left, 3, right, 3, lags, 4) == 0); \
        assert(lags[0] == 28 && lags[1] == 16 && lags[2] == 6 && lags[3] == 0); \
    } while (0)
    CHECK_PAIR(float, f32);
    CHECK_PAIR(double, f64);
    #undef CHECK_PAIR
    #define CHECK_ORDER(type, suffix) do { \
        type values[] = {3, -0.0, 0.0, 3, 1}; \
        size_t indices[5]; \
        assert(ciglet_rs_sort_##suffix(values, 5, values, 5, indices, 5) == 0); \
        assert(indices[0] == 1 && indices[1] == 2 && indices[2] == 4 && indices[3] == 0 && indices[4] == 3); \
        type result = -71; \
        assert(ciglet_rs_select_nth_##suffix(values, 5, 2, &result) == 0 && result == 1); \
        result = -71; \
        assert(ciglet_rs_select_nth_##suffix(values, 5, 5, &result) == 3 && result == -71); \
        assert(ciglet_rs_select_nth_##suffix(values, 5, 2, &values[2]) == 2); \
        assert(ciglet_rs_sort_##suffix(NULL, 0, NULL, 0, NULL, 0) == 0); \
    } while (0)
    CHECK_ORDER(float, f32);
    CHECK_ORDER(double, f64);
    #undef CHECK_ORDER
    #define CHECK_ORDER_BITS(type, word, suffix, positive_nan, negative_nan, negative_zero, infinity) do { \
        word original[] = {positive_nan, negative_zero, 0, negative_nan, infinity}; \
        type values[5]; \
        size_t indices[5]; \
        const size_t expected[] = {3, 1, 2, 4, 0}; \
        memcpy(values, original, sizeof values); \
        assert(ciglet_rs_sort_##suffix(values, 5, values, 5, indices, 5) == 0); \
        for (size_t index = 0; index < 5; ++index) { \
            word actual; \
            memcpy(&actual, &values[index], sizeof actual); \
            assert(indices[index] == expected[index] && actual == original[expected[index]]); \
        } \
        type selected; \
        word actual; \
        assert(ciglet_rs_select_nth_##suffix(values, 5, 0, &selected) == 0); \
        memcpy(&actual, &selected, sizeof actual); \
        assert(actual == negative_nan); \
    } while (0)
    CHECK_ORDER_BITS(float, uint32_t, f32, UINT32_C(0x7fc12345), UINT32_C(0xffc01234), UINT32_C(0x80000000), UINT32_C(0x7f800000));
    CHECK_ORDER_BITS(double, uint64_t, f64, UINT64_C(0x7ff8123456789abc), UINT64_C(0xfff80123456789ab), UINT64_C(0x8000000000000000), UINT64_C(0x7ff0000000000000));
    #undef CHECK_ORDER_BITS
    #define CHECK_ARRAYS(type, handle_type, suffix) do { \
        handle_type *handle = NULL, *copy = NULL, *transposed = NULL, *from_rows = NULL, *filled = NULL; \
        type source[] = {1, 2, 3, 4, 5, 6}; \
        type output[6], result = -71; \
        size_t shape[3]; \
        assert(ciglet_rs_array_from_slice_##suffix(2, 3, source, 6, &handle) == 0); \
        assert(ciglet_rs_array_shape_##suffix(handle, shape, 3) == 0); \
        assert(shape[0] == 2 && shape[1] == 3 && shape[2] == 6); \
        assert(ciglet_rs_array_clone_##suffix(handle, &copy) == 0 && copy != handle); \
        assert(ciglet_rs_array_set_##suffix(copy, 0, 0, -1) == 0); \
        assert(ciglet_rs_array_get_##suffix(handle, 0, 0, &result) == 0 && result == 1); \
        assert(ciglet_rs_array_transposed_##suffix(handle, &transposed) == 0); \
        assert(ciglet_rs_array_copy_flat_##suffix(transposed, output, 6) == 0); \
        assert(output[0] == 1 && output[1] == 4 && output[2] == 2 && output[3] == 5 && output[4] == 3 && output[5] == 6); \
        assert(ciglet_rs_array_reshape_##suffix(handle, 3, 2) == 0); \
        assert(ciglet_rs_array_transpose_##suffix(handle) == 0); \
        assert(ciglet_rs_array_copy_flat_##suffix(handle, output, 6) == 0); \
        assert(output[0] == 1 && output[1] == 3 && output[2] == 5 && output[3] == 2 && output[4] == 4 && output[5] == 6); \
        type row[] = {-2, -3, -4}; \
        assert(ciglet_rs_array_write_row_##suffix(handle, 0, row, 3) == 0); \
        assert(ciglet_rs_array_copy_row_##suffix(handle, 0, output, 3) == 0); \
        assert(output[0] == -2 && output[1] == -3 && output[2] == -4); \
        assert(ciglet_rs_array_write_flat_##suffix(handle, source, 6) == 0); \
        assert(ciglet_rs_array_copy_flat_##suffix(handle, output, 6) == 0 && memcmp(source, output, sizeof source) == 0); \
        assert(ciglet_rs_array_reshape_##suffix(handle, 4, 2) == 3); \
        const type *rows[] = {source, source + 3}; \
        size_t counts[] = {3, 2}; \
        assert(ciglet_rs_array_from_rows_##suffix(rows, counts, 2, &from_rows) == 3 && from_rows == NULL); \
        counts[1] = 3; \
        assert(ciglet_rs_array_from_rows_##suffix(rows, counts, 2, &from_rows) == 0); \
        assert(ciglet_rs_array_copy_flat_##suffix(from_rows, output, 6) == 0 && memcmp(source, output, sizeof source) == 0); \
        assert(ciglet_rs_array_filled_##suffix(2, 2, -0.0, &filled) == 0); \
        assert(ciglet_rs_array_copy_flat_##suffix(filled, output, 4) == 0); \
        type negative_zero = -0.0; \
        assert(memcmp(output, &negative_zero, sizeof negative_zero) == 0); \
        assert(ciglet_rs_array_release_##suffix(&handle) == 0 && handle == NULL); \
        assert(ciglet_rs_array_release_##suffix(&handle) == 0); \
        assert(ciglet_rs_array_release_##suffix(&copy) == 0 && copy == NULL); \
        assert(ciglet_rs_array_release_##suffix(&transposed) == 0 && transposed == NULL); \
        assert(ciglet_rs_array_release_##suffix(&from_rows) == 0 && from_rows == NULL); \
        assert(ciglet_rs_array_release_##suffix(&filled) == 0 && filled == NULL); \
    } while (0)
    CHECK_ARRAYS(float, CigletRsArrayF32, f32);
    CHECK_ARRAYS(double, CigletRsArrayF64, f64);
    #undef CHECK_ARRAYS
    #define CHECK_FOURIER(type, handle_type, suffix) do { \
        type real[] = {1, 2, 3, 4}, imaginary[4] = {0}; \
        assert(ciglet_rs_fft_##suffix(4, real, NULL, 0, real, imaginary) == 0); \
        assert(real[0] == 10 && real[1] == -2 && imaginary[1] == 2 && imaginary[3] == -2); \
        assert(ciglet_rs_fft_##suffix(4, real, imaginary, 1, real, imaginary) == 0); \
        assert(real[0] == 1 && real[1] == 2 && real[2] == 3 && real[3] == 4); \
        assert(ciglet_rs_fft_##suffix(4, real, NULL, 2, real, imaginary) == 3); \
        assert(ciglet_rs_fft_##suffix(4, real, NULL, 0, real, real) == 2); \
        type impulse[] = {3, 0, 0}, inverse[3]; \
        assert(ciglet_rs_inverse_dft_##suffix(3, impulse, NULL, inverse, NULL) == 0); \
        assert(inverse[0] == 1 && inverse[1] == 1 && inverse[2] == 1); \
        type odd[] = {1, 2, 3, 4, 5}; \
        assert(ciglet_rs_fft_shift_##suffix(odd, 5, odd, 5) == 0); \
        assert(odd[0] == 4 && odd[1] == 5 && odd[2] == 1 && odd[3] == 2 && odd[4] == 3); \
        handle_type *plan = NULL; \
        size_t length = 0; \
        assert(ciglet_rs_fft_plan_new_##suffix(4, &plan) == 0); \
        assert(ciglet_rs_fft_plan_length_##suffix(plan, &length) == 0 && length == 4); \
        type unit[] = {1, 0, 0, 0}; \
        assert(ciglet_rs_fft_plan_transform_##suffix(plan, unit, NULL, 4, 0, real, imaginary) == 0); \
        for (size_t index = 0; index < 4; ++index) { assert(real[index] == 1 && imaginary[index] == 0); } \
        assert(ciglet_rs_fft_plan_transform_##suffix(plan, real, imaginary, 4, 1, real, imaginary) == 0); \
        assert(real[0] == 1 && real[1] == 0 && real[2] == 0 && real[3] == 0); \
        assert(ciglet_rs_fft_plan_transform_##suffix(plan, unit, NULL, 3, 0, real, imaginary) == 2); \
        assert(ciglet_rs_fft_plan_release_##suffix(&plan) == 0 && plan == NULL); \
        assert(ciglet_rs_fft_plan_release_##suffix(&plan) == 0); \
    } while (0)
    CHECK_FOURIER(float, CigletRsFftF32, f32);
    CHECK_FOURIER(double, CigletRsFftF64, f64);
    #undef CHECK_FOURIER
    #define CHECK_PHASE(type, suffix) do { \
        type real[] = {3, 0}, imaginary[] = {4, 0}, out[] = {71, 71}; \
        assert(ciglet_rs_magnitude_##suffix(real, 2, imaginary, 2, real, 2) == 0); \
        assert(real[0] == 5 && real[1] == 0); \
        real[0] = 1; \
        assert(ciglet_rs_argument_##suffix(real, 2, imaginary, 2, out, 2) == 0); \
        assert(out[0] > 1 && out[0] < 2); \
        type angle[] = {0, 0}; \
        assert(ciglet_rs_polar_real_##suffix(real, 2, angle, 2, out, 2) == 0); \
        assert(out[0] > (type)0.999 && out[0] < (type)1.001); \
        assert(ciglet_rs_polar_imaginary_##suffix(real, 2, angle, 2, out, 2) == 0); \
        assert(out[0] > (type)-0.001 && out[0] < (type)0.001); \
        assert(ciglet_rs_phase_wrap_##suffix(7, out) == 0 && out[0] > 0 && out[0] < 1); \
        assert(ciglet_rs_phase_difference_##suffix(1, 0, out) == 0 && out[0] > (type)0.999 && out[0] < (type)1.001); \
        type jump[] = {3, -3}; \
        assert(ciglet_rs_phase_unwrap_##suffix(jump, 2, jump, 2) == 0 && jump[1] > 3 && jump[1] < 4); \
        type spectrum[] = {1, 2, 3, 4, 99, 99}; \
        assert(ciglet_rs_complete_spectrum_##suffix(spectrum, 6, 1, 0, spectrum, 6) == 0); \
        assert(spectrum[3] == 4 && spectrum[4] == -3 && spectrum[5] == -2); \
        assert(ciglet_rs_complete_spectrum_##suffix(spectrum, 6, 1, 1, spectrum, 6) == 0 && spectrum[3] == 3); \
        assert(ciglet_rs_complete_spectrum_##suffix(spectrum, 6, 2, 0, spectrum, 6) == 3 && spectrum[3] == 3); \
        type singleton[] = {2}; \
        assert(ciglet_rs_dct_##suffix(singleton, 1, singleton, 1) == 0 && singleton[0] > (type)1.999 && singleton[0] < (type)2.001); \
        assert(ciglet_rs_dct_##suffix(NULL, 0, NULL, 0) == 3); \
        assert(ciglet_rs_phase_unwrap_##suffix(NULL, 0, NULL, 0) == 0); \
        out[0] = out[1] = 71; real[1] = NAN; \
        assert(ciglet_rs_magnitude_##suffix(real, 2, imaginary, 2, out, 2) == 3 && out[0] == 71 && out[1] == 71); \
        assert(ciglet_rs_magnitude_##suffix(real, 2, imaginary, 1, out, 2) == 2); \
    } while (0)
    CHECK_PHASE(float, f32);
    CHECK_PHASE(double, f64);
    #undef CHECK_PHASE
    #define CHECK_WINDOW(type, suffix) do { \
        type output[8], accurate[8]; \
        for (uint32_t kind = 0; kind < 7; ++kind) { \
            for (uint32_t mode = 0; mode < 2; ++mode) { \
                assert(ciglet_rs_window_##suffix(kind, mode, output, 8) == 0); \
                for (size_t i = 0; i < 8; ++i) assert(isfinite(output[i])); \
                assert(ciglet_rs_window_##suffix(kind, mode, NULL, 0) == 0); \
                assert(ciglet_rs_window_##suffix(kind, mode, output, 1) == 0 && output[0] == 1); \
            } \
        } \
        assert(ciglet_rs_window_##suffix(3, 0, accurate, 8) == 0); \
        assert(ciglet_rs_window_##suffix(3, 1, output, 8) == 0); \
        assert(memcmp(accurate, output, sizeof(output)) == 0); \
        for (size_t i = 0; i < 8; ++i) output[i] = 71; \
        assert(ciglet_rs_window_##suffix(7, 0, output, 8) == 3 && output[0] == 71); \
        assert(ciglet_rs_window_##suffix(0, 2, output, 8) == 3 && output[0] == 71); \
    } while (0)
    CHECK_WINDOW(float, f32);
    CHECK_WINDOW(double, f64);
    #undef CHECK_WINDOW
    #define CHECK_CUSTOM_WINDOW(type, suffix, callback_suffix) do { \
        struct WindowContext context = {0}; \
        type output[2] = {71, 71}; \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(3, trig_##callback_suffix, &context, output, 2) == 0); \
        assert(context.calls == 4 && context.angles[0] == 0 && context.angles[1] == 0); \
        assert(fabs(context.angles[2] - 3.141592653589793) < 1e-14 && fabs(context.angles[3] - 6.283185307179586) < 1e-14); \
        assert(fabs((double)output[0] - 0.42) < 1e-7 && output[0] == output[1]); \
        context.calls = 0; context.fail_on = 2; output[0] = output[1] = 71; \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(4, trig_##callback_suffix, &context, output, 2) == 3); \
        assert(context.calls == 2 && output[0] == 71 && output[1] == 71); \
        context.calls = 0; context.fail_on = 0; context.nonfinite = 1; \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(1, trig_##callback_suffix, &context, output, 2) == 3); \
        assert(context.calls == 1 && output[0] == 71 && output[1] == 71); \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(1, NULL, NULL, output, 2) == 3 && output[0] == 71); \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(0, NULL, NULL, output, 2) == 0 && output[0] == 1 && output[1] == 1); \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(1, NULL, NULL, NULL, 0) == 0); \
        assert(ciglet_rs_window_custom_##suffix##_##callback_suffix(1, NULL, NULL, output, 1) == 0 && output[0] == 1); \
    } while (0)
    CHECK_CUSTOM_WINDOW(float, f32, f32);
    CHECK_CUSTOM_WINDOW(float, f32, f64);
    CHECK_CUSTOM_WINDOW(double, f64, f32);
    CHECK_CUSTOM_WINDOW(double, f64, f64);
    #undef CHECK_CUSTOM_WINDOW
    #define CHECK_CZT(type, handle_type, suffix, tolerance) do { \
        type signal[] = {3, 0, 0}, real[3], imaginary[3]; \
        assert(ciglet_rs_czt_##suffix(3, 0, 0, signal, NULL, real, imaginary) == 0); \
        for (size_t i = 0; i < 3; ++i) assert(fabs((double)real[i] - 3) < tolerance && fabs((double)imaginary[i]) < tolerance); \
        assert(ciglet_rs_czt_##suffix(3, 0, 1, signal, NULL, real, NULL) == 0); \
        for (size_t i = 0; i < 3; ++i) assert(fabs((double)real[i] - 1) < tolerance); \
        assert(ciglet_rs_czt_##suffix(3, 0, 0, signal, NULL, real, real) == 2); \
        real[0] = real[1] = real[2] = 71; \
        assert(ciglet_rs_czt_##suffix(3, NAN, 0, signal, NULL, real, NULL) == 3); \
        assert(ciglet_rs_czt_##suffix(3, 0, 2, signal, NULL, real, NULL) == 3); \
        for (size_t i = 0; i < 3; ++i) assert(real[i] == 71); \
        handle_type *plan = NULL; \
        assert(ciglet_rs_czt_plan_new_##suffix(3, 0, 0, &plan) == 0); \
        handle_type *original = plan; \
        assert(ciglet_rs_czt_plan_new_##suffix(3, NAN, 0, &plan) == 3 && plan == original); \
        size_t length = 0; \
        assert(ciglet_rs_czt_plan_length_##suffix(plan, &length) == 0 && length == 3); \
        assert(ciglet_rs_czt_plan_transform_##suffix(plan, signal, NULL, 2, real, NULL) == 2); \
        for (size_t i = 0; i < 3; ++i) assert(real[i] == 71); \
        for (size_t repeat = 0; repeat < 3; ++repeat) { \
            assert(ciglet_rs_czt_plan_transform_##suffix(plan, signal, NULL, 3, real, imaginary) == 0); \
            for (size_t i = 0; i < 3; ++i) assert(fabs((double)real[i] - 3) < tolerance && fabs((double)imaginary[i]) < tolerance); \
        } \
        assert(ciglet_rs_czt_plan_release_##suffix(&plan) == 0 && plan == NULL); \
        assert(ciglet_rs_czt_plan_release_##suffix(&plan) == 0); \
        assert(ciglet_rs_czt_##suffix(3, 0, 0, NULL, NULL, real, NULL) == 0); \
        for (size_t i = 0; i < 3; ++i) assert(real[i] == 0); \
    } while (0)
    CHECK_CZT(float, CigletRsCztF32, f32, 1e-5);
    CHECK_CZT(double, CigletRsCztF64, f64, 1e-12);
    #undef CHECK_CZT
    #define CHECK_FREQUENCY_DISTANCE(type, suffix, tolerance) do { \
        type mel, frequency, bark, loudness, output = 71, grid[3] = {71, 71, 71}; \
        assert(ciglet_rs_frequency_to_mel_##suffix(1000, &mel) == 0 && mel > 0); \
        assert(ciglet_rs_mel_to_frequency_##suffix(mel, &frequency) == 0 && fabs((double)frequency - 1000) < 1); \
        assert(ciglet_rs_frequency_to_bark_##suffix(1000, &bark) == 0 && bark > 0); \
        assert(ciglet_rs_bark_to_frequency_##suffix(bark, &frequency) == 0 && fabs((double)frequency - 1000) < 0.001); \
        assert(ciglet_rs_equal_loudness_##suffix(1000, &loudness) == 0 && loudness > 0 && loudness < 1); \
        assert(ciglet_rs_frequency_to_mel_##suffix(-700, &output) == 3 && output == 71); \
        assert(ciglet_rs_mel_space_##suffix(50, 8000, 2, grid, 2) == 2 && grid[0] == 71); \
        assert(ciglet_rs_mel_space_##suffix(8000, 50, 2, grid, 3) == 3 && grid[0] == 71); \
        assert(ciglet_rs_mel_space_##suffix(50, 8000, 2, grid, 3) == 0 && grid[0] > 0 && grid[1] > grid[0] && grid[2] > grid[1]); \
        type source[] = {2, 2, 2}, reference[] = {1, 1, 1}; \
        assert(ciglet_rs_itakura_saito_##suffix(source, 3, reference, 3, &output) == 0 && fabs((double)output - (1 - log(2.0))) < tolerance); \
        assert(ciglet_rs_log_itakura_saito_##suffix(source, 3, reference, 3, &output) == 0 && fabs((double)output - log(1 - log(2.0))) < tolerance); \
        type explicit_output = 71; \
        assert(ciglet_rs_log_itakura_saito_with_##suffix(source, 3, reference, 3, 0, &explicit_output) == 0 && explicit_output == output); \
        assert(ciglet_rs_log_itakura_saito_with_##suffix(source, 3, reference, 3, 1, &output) == 0 && isfinite(output)); \
        assert(ciglet_rs_itakura_saito_##suffix(reference, 3, reference, 3, &output) == 0 && output == 0); \
        assert(ciglet_rs_log_itakura_saito_##suffix(reference, 3, reference, 3, &output) == 0 && isinf(output) && output < 0); \
        type zero[] = {0}; \
        assert(ciglet_rs_itakura_saito_##suffix(zero, 1, reference, 1, &output) == 0 && isinf(output) && output > 0); \
        output = 71; \
        assert(ciglet_rs_itakura_saito_##suffix(zero, 1, zero, 1, &output) == 3 && output == 71); \
        assert(ciglet_rs_log_itakura_saito_with_##suffix(source, 3, reference, 3, 2, &output) == 3 && output == 71); \
        assert(ciglet_rs_itakura_saito_##suffix(source, 3, reference, 2, &output) == 2 && output == 71); \
    } while (0)
    CHECK_FREQUENCY_DISTANCE(float, f32, 1e-6);
    CHECK_FREQUENCY_DISTANCE(double, f64, 1e-14);
    #undef CHECK_FREQUENCY_DISTANCE
    #define CHECK_FRAME_EXTREMA(type, suffix) do { \
        type input[] = {1, 2, 3, 4, 5, 6}; \
        assert(ciglet_rs_fetch_frame_##suffix(input, 6, 0, input, 4) == 0); \
        assert(input[0] == 0 && input[1] == 0 && input[2] == 1 && input[3] == 2 && input[4] == 5); \
        type frame[3] = {71, 71, 71}; \
        assert(ciglet_rs_fetch_frame_##suffix(NULL, 0, INTPTR_MIN, frame, 3) == 3); \
        assert(frame[0] == 71 && frame[1] == 71 && frame[2] == 71); \
        assert(ciglet_rs_fetch_frame_##suffix(NULL, 0, 0, frame, 3) == 0); \
        assert(frame[0] == 0 && frame[1] == 0 && frame[2] == 0); \
        type signal[] = {8, 1, 5, 0, 5, 1, 9}; size_t index = 71; \
        assert(ciglet_rs_peak_##suffix(signal, 7, 0, 6, 0, &index) == 0 && index == 0); \
        assert(ciglet_rs_peak_##suffix(signal, 7, 1, 5, 0, &index) == 0 && index == 2); \
        assert(ciglet_rs_peak_##suffix(signal, 7, 1, 5, 1, &index) == 0 && index == 3); \
        assert(ciglet_rs_extremum_##suffix(signal, 7, 0, 6, 0, &index) == 0 && index == 6); \
        assert(ciglet_rs_extremum_##suffix(signal, 7, 0, 6, 1, &index) == 0 && index == 3); \
        index = 71; \
        assert(ciglet_rs_peak_##suffix(signal, 7, 5, 3, 0, &index) == 3 && index == 71); \
        assert(ciglet_rs_extremum_##suffix(signal, 7, 0, 7, 0, &index) == 3 && index == 71); \
        assert(ciglet_rs_peak_##suffix(signal, 7, 0, 6, 2, &index) == 3 && index == 71); \
    } while (0)
    CHECK_FRAME_EXTREMA(float, f32);
    CHECK_FRAME_EXTREMA(double, f64);
    #undef CHECK_FRAME_EXTREMA
    #define CHECK_BAND_CEPSTRA(type, suffix, tolerance) do { \
        type input[] = {1, 2, 3, 4}, full[4], selected[2]; \
        assert(ciglet_rs_band_to_cepstrum_legacy_##suffix(input, 4, 1, 1, full, 4) == 0); \
        assert(ciglet_rs_band_to_cepstrum_##suffix(input, 4, 1, 1, selected, 2) == 0); \
        assert(selected[0] == full[0] && selected[1] == full[1]); \
        /* Strict original cos_2(0) is 0.99998641014099121, not exact one. */ \
        assert(fabs((double)selected[1] - 4.999932050704956) < tolerance); \
        type matrix[] = {1, 2, 3, 4, 1, 2, 3, 4}, cepstra[4]; \
        assert(ciglet_rs_band_spectrogram_to_cepstra_##suffix(matrix, 8, 4, 1, 1, cepstra, 4) == 0); \
        assert(cepstra[0] == selected[0] && cepstra[1] == selected[1]); \
        assert(cepstra[2] == selected[0] && cepstra[3] == selected[1]); \
        assert(ciglet_rs_band_to_cepstrum_##suffix(input, 4, 1, 1, input, 2) == 0); \
        assert(input[0] == selected[0] && input[1] == selected[1] && input[2] == 3 && input[3] == 4); \
        type sentinel[] = {71, 71, 71, 71}; matrix[6] = NAN; \
        assert(ciglet_rs_band_spectrogram_to_cepstra_##suffix(matrix, 8, 4, 1, 1, sentinel, 4) == 3); \
        assert(sentinel[0] == 71 && sentinel[1] == 71 && sentinel[2] == 71 && sentinel[3] == 71); \
        assert(ciglet_rs_band_to_cepstrum_##suffix(input, 4, 1, 2, sentinel, 2) == 3 && sentinel[0] == 71); \
        assert(ciglet_rs_band_to_cepstrum_legacy_##suffix(input, 4, 1, 1, sentinel, 2) == 2 && sentinel[0] == 71); \
        assert(ciglet_rs_band_to_cepstrum_##suffix(input, 4, 0, 0, NULL, 0) == 0); \
        assert(ciglet_rs_band_spectrogram_to_cepstra_##suffix(NULL, 0, 4, 1, 1, NULL, 0) == 0); \
    } while (0)
    CHECK_BAND_CEPSTRA(float, f32, 5e-7);
    CHECK_BAND_CEPSTRA(double, f64, 1e-14);
    #undef CHECK_BAND_CEPSTRA

    #define CHECK_FILTERBANK(type, suffix, handle_type) do { \
        handle_type *bank = NULL, *copy = NULL; size_t shape[2] = {71,71}, support[2] = {71,71}; \
        assert(ciglet_rs_filterbank_empty_##suffix(4,8000,1,&bank) == 0); \
        assert(ciglet_rs_filterbank_shape_##suffix(bank,shape,2) == 0 && shape[0] == 4 && shape[1] == 1); \
        type metadata = 71, response[] = {NAN,0.5,1,INFINITY}, copied[4]; \
        assert(ciglet_rs_filterbank_nyquist_##suffix(bank,&metadata) == 0 && metadata == 8000); \
        assert(ciglet_rs_filterbank_set_channel_##suffix(bank,0,response,4,1,3) == 0); \
        assert(ciglet_rs_filterbank_support_##suffix(bank,0,support,2) == 0 && support[0] == 1 && support[1] == 3); \
        assert(ciglet_rs_filterbank_response_##suffix(bank,0,copied,4) == 0); \
        assert(isnan(copied[0]) && copied[1] == 0.5 && copied[2] == 1 && isinf(copied[3])); \
        assert(ciglet_rs_filterbank_clone_##suffix(bank,&copy) == 0); \
        assert(ciglet_rs_filterbank_set_nyquist_##suffix(bank,NAN) == 0); \
        assert(ciglet_rs_filterbank_nyquist_##suffix(bank,&metadata) == 0 && isnan(metadata)); \
        assert(ciglet_rs_filterbank_nyquist_##suffix(copy,&metadata) == 0 && metadata == 8000); \
        type spectrum[] = {NAN,2,3,NAN}, expected = 71, matrix[] = {NAN,2,3,NAN,NAN,2,3,NAN}, output[2]; \
        assert(ciglet_rs_filterbank_apply_##suffix(bank,spectrum,4,1,&expected,1) == 0 && expected > 1 && expected < 2); \
        assert(ciglet_rs_filterbank_apply_spectrogram_##suffix(bank,matrix,8,1,output,2) == 0); \
        assert(output[0] == expected && output[1] == expected); \
        assert(ciglet_rs_filterbank_apply_frames_##suffix(copy,matrix,8,2,1,output,2) == 0); \
        assert(output[0] == expected && output[1] == expected); \
        assert(ciglet_rs_filterbank_apply_##suffix(bank,spectrum,4,1,spectrum,1) == 0 && spectrum[0] == expected); \
        matrix[5] = NAN; output[0] = output[1] = 71; \
        assert(ciglet_rs_filterbank_apply_frames_##suffix(bank,matrix,8,2,0,output,2) == 3 && output[0] == 71 && output[1] == 71); \
        handle_type *retained = bank; \
        assert(ciglet_rs_filterbank_mel_default_##suffix(0,8000,1,0,8000,&bank) == 3 && bank == retained); \
        assert(ciglet_rs_filterbank_release_##suffix(&copy) == 0 && copy == NULL); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0 && bank == NULL); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0); \
        assert(ciglet_rs_filterbank_mel_##suffix(65,8000,8,50,8000,0.85,120,&bank) == 0); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0); \
        assert(ciglet_rs_filterbank_mel_default_##suffix(65,8000,8,50,8000,&bank) == 0); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0); \
        assert(ciglet_rs_filterbank_plp_##suffix(65,8000,8,&bank) == 0); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0); \
        assert(ciglet_rs_filterbank_empty_##suffix(0,NAN,1,&bank) == 0); \
        assert(ciglet_rs_filterbank_apply_frames_##suffix(bank,NULL,0,2,1,output,2) == 0 && output[0] == 0 && output[1] == 0); \
        assert(ciglet_rs_filterbank_apply_spectrogram_##suffix(bank,NULL,0,1,output,2) == 3); \
        assert(ciglet_rs_filterbank_release_##suffix(&bank) == 0); \
    } while (0)
    CHECK_FILTERBANK(float, f32, CigletRsFilterBankF32);
    CHECK_FILTERBANK(double, f64, CigletRsFilterBankF64);
    #undef CHECK_FILTERBANK
    #define CHECK_REAL_CEPSTRUM(type, suffix, handle_type) do { \
        handle_type *plan = NULL; size_t shape[2] = {71,71}; \
        assert(ciglet_rs_real_cepstrum_new_##suffix(4,&plan) == 0); \
        assert(ciglet_rs_real_cepstrum_shape_##suffix(plan,shape,2) == 0 && shape[0] == 4 && shape[1] == 3); \
        type logs[] = {0,0,4}, coefficients[4], spectrum[4], phase[4], modern[4]; \
        assert(ciglet_rs_real_cepstrum_analyze_##suffix(plan,logs,3,0,coefficients,4) == 0); \
        assert(coefficients[0] == 1 && coefficients[1] == -1 && coefficients[2] == 1 && coefficients[3] == -1); \
        assert(ciglet_rs_real_cepstrum_reconstruct_##suffix(plan,coefficients,4,0,spectrum,4) == 0); \
        assert(spectrum[0] == 0 && spectrum[1] == 0 && spectrum[2] == 4 && spectrum[3] == 0); \
        assert(ciglet_rs_real_cepstrum_minimum_phase_##suffix(plan,logs,3,0,0,phase,4) == 0); \
        assert(phase[0] == 0 && phase[1] == 2 && phase[2] == 0 && phase[3] == -2); \
        assert(ciglet_rs_real_cepstrum_from_log_spectrum_##suffix(logs,3,modern,4) == 0); \
        for(size_t i=0;i<4;++i) assert(modern[i] == coefficients[i]); \
        assert(ciglet_rs_real_cepstrum_to_log_spectrum_##suffix(coefficients,4,modern,4) == 0); \
        for(size_t i=0;i<4;++i) assert(modern[i] == spectrum[i]); \
        assert(ciglet_rs_real_cepstrum_minimum_phase_default_##suffix(logs,3,modern,4) == 0); \
        for(size_t i=0;i<4;++i) assert(modern[i] == phase[i]); \
        type ignored[] = {0,0,NAN}; \
        assert(ciglet_rs_real_cepstrum_analyze_##suffix(plan,ignored,3,1,modern,4) == 0); \
        for(size_t i=0;i<4;++i) assert(modern[i] == 0); \
        for(size_t i=0;i<4;++i) modern[i] = 71; \
        assert(ciglet_rs_real_cepstrum_analyze_##suffix(plan,ignored,3,0,modern,4) == 3); \
        for(size_t i=0;i<4;++i) assert(modern[i] == 71); \
        type magnitudes[] = {1,1,1,1,1,1}, matrix[2], reconstructed[6]; \
        assert(ciglet_rs_real_cepstrum_spectrogram_to_cepstra_##suffix(plan,magnitudes,6,1,0,matrix,2) == 0); \
        assert(isfinite(matrix[0]) && matrix[0] == matrix[1]); \
        assert(ciglet_rs_real_cepstrum_cepstra_to_spectrogram_##suffix(plan,NULL,0,2,0,0,reconstructed,6) == 0); \
        for(size_t i=0;i<6;++i) assert(reconstructed[i] == reconstructed[0] && reconstructed[i] > 0.9 && reconstructed[i] < 1.1); \
        magnitudes[3] = NAN; matrix[0] = matrix[1] = 71; \
        assert(ciglet_rs_real_cepstrum_spectrogram_to_cepstra_##suffix(plan,magnitudes,6,1,0,matrix,2) == 3); \
        assert(matrix[0] == 71 && matrix[1] == 71); \
        assert(ciglet_rs_real_cepstrum_analyze_##suffix(plan,logs,3,0,modern,4) == 0 && modern[0] == 1); \
        handle_type *retained = plan; \
        assert(ciglet_rs_real_cepstrum_new_##suffix(3,&plan) == 3 && plan == retained); \
        assert(ciglet_rs_real_cepstrum_release_##suffix(&plan) == 0 && plan == NULL); \
        assert(ciglet_rs_real_cepstrum_release_##suffix(&plan) == 0); \
    } while (0)
    CHECK_REAL_CEPSTRUM(float, f32, CigletRsRealCepstrumF32);
    CHECK_REAL_CEPSTRUM(double, f64, CigletRsRealCepstrumF64);
    #undef CHECK_REAL_CEPSTRUM
    #define CHECK_POLYNOMIAL(type, suffix) do { \
        type coefficients[] = {1,0,1}, imaginary_coefficients[] = {0,0,0}, a = 71, b = 71; \
        assert(ciglet_rs_polynomial_evaluate_##suffix(coefficients,imaginary_coefficients,3,2,1,&a,&b) == 0 && a == 4 && b == 4); \
        assert(ciglet_rs_polynomial_evaluate_real_##suffix(coefficients,3,2,1,&a,&b) == 0 && a == 4 && b == 4); \
        type real[2], imaginary[2]; size_t iterations = 71; uint32_t converged = 71; double residual = 71; \
        assert(ciglet_rs_polynomial_roots_##suffix(coefficients,NULL,3,real,imaginary,2,&iterations,&converged,&residual) == 0); \
        assert(iterations > 0 && iterations <= 50 && converged == 1 && residual < 1e-6); \
        for(size_t i=0;i<2;++i) assert(fabs((double)real[i]) < 1e-6 && fabs(fabs((double)imaginary[i])-1) < 1e-6); \
        type expected_real[2] = {real[0],real[1]}, expected_imaginary[2] = {imaginary[0],imaginary[1]}; \
        size_t expected_iterations = iterations; double expected_residual = residual; \
        assert(ciglet_rs_polynomial_roots_real_##suffix(coefficients,3,real,imaginary,2,&iterations,&converged,&residual) == 0); \
        assert(iterations == expected_iterations && converged == 1 && residual == expected_residual); \
        for(size_t i=0;i<2;++i) assert(real[i] == expected_real[i] && imaginary[i] == expected_imaginary[i]); \
        type constant[] = {1}; \
        assert(ciglet_rs_polynomial_roots_##suffix(NULL,constant,1,NULL,NULL,0,&iterations,&converged,&residual) == 0); \
        assert(iterations == 0 && converged == 1 && residual == 0); \
        type invalid[] = {0,1}; real[0] = imaginary[0] = 71; iterations = converged = 71; residual = 71; \
        assert(ciglet_rs_polynomial_roots_real_##suffix(invalid,2,real,imaginary,1,&iterations,&converged,&residual) == 3); \
        assert(real[0] == 71 && imaginary[0] == 71 && iterations == 71 && converged == 71 && residual == 71); \
        assert(ciglet_rs_polynomial_evaluate_real_##suffix(coefficients,3,2,1,&a,&a) == 2); \
    } while (0)
    CHECK_POLYNOMIAL(float, f32);
    CHECK_POLYNOMIAL(double, f64);
    #undef CHECK_POLYNOMIAL
    double nonconvergent[] = {1,-36,546,-4536,22449,-67284,118124,-109584,40320}, roots_real[8], roots_imaginary[8], roots_residual = 71;
    size_t roots_iterations = 71; uint32_t roots_converged = 71;
    assert(ciglet_rs_polynomial_roots_real_f64(nonconvergent,9,roots_real,roots_imaginary,8,&roots_iterations,&roots_converged,&roots_residual) == 0);
    assert(roots_iterations == 50 && roots_converged == 0 && roots_residual > 1e-6);
    #define CHECK_PREDICTION(type, suffix) do { \
        type correlation[] = {2,0.5}, coefficients[2], gain = 71; \
        for(uint32_t policy=0;policy<2;++policy) { \
            assert(ciglet_rs_prediction_levinson_##suffix(correlation,2,policy,coefficients,2) == 0); \
            assert(coefficients[0] == 1 && coefficients[1] > -0.251 && coefficients[1] < -0.249); \
            assert(ciglet_rs_prediction_gain_##suffix(coefficients,2,correlation,2,policy,&gain) == 0); \
            assert(gain > 1 && gain < 2); \
            type silence[3] = {0,0,0}, silent_coefficients[3]; \
            assert(ciglet_rs_prediction_levinson_##suffix(silence,3,policy,silent_coefficients,3) == 0); \
            assert(silent_coefficients[0] == 1 && silent_coefficients[1] == 0 && silent_coefficients[2] == 0); \
            assert(ciglet_rs_prediction_gain_##suffix(silent_coefficients,3,silence,3,policy,&gain) == 0 && gain == 0); \
        } \
        coefficients[0] = coefficients[1] = gain = 71; \
        assert(ciglet_rs_prediction_levinson_##suffix(correlation,2,2,coefficients,2) == 3); \
        assert(coefficients[0] == 71 && coefficients[1] == 71); \
        assert(ciglet_rs_prediction_gain_##suffix(coefficients,2,correlation,1,0,&gain) == 2 && gain == 71); \
        assert(ciglet_rs_prediction_levinson_##suffix(correlation,2,0,correlation,2) == 0); \
        assert(correlation[0] == 1 && correlation[1] > -0.251 && correlation[1] < -0.249); \
    } while (0)
    CHECK_PREDICTION(float, f32);
    CHECK_PREDICTION(double, f64);
    #undef CHECK_PREDICTION
    #define CHECK_SCALED_PREDICTION(type, suffix, handle_type, maximum) do { \
        type samples[] = {1,0}, coefficients[2], correlations[2], gain = 71; \
        double fractions[2], gain_fraction; int32_t exponents[2], gain_exponent; size_t count = 71; \
        handle_type *model = NULL, *copy = NULL; \
        for(uint32_t arithmetic=0;arithmetic<2;++arithmetic) { \
            assert(ciglet_rs_prediction_from_samples_##suffix(samples,2,1,arithmetic,coefficients,correlations,2) == 0); \
            assert(coefficients[0] == 1 && coefficients[1] == 0 && correlations[0] == 1 && correlations[1] == 0); \
            type magnitudes[] = {0,0,2}; \
            for(uint32_t nyquist=0;nyquist<2;++nyquist) { \
                assert(ciglet_rs_prediction_from_magnitude_##suffix(magnitudes,3,0,nyquist,arithmetic,coefficients,correlations,1) == 0); \
                assert(coefficients[0] == 1 && correlations[0] == (nyquist == 0 ? 1 : 0)); \
            } \
        } \
        assert(ciglet_rs_scaled_prediction_from_samples_##suffix(samples,2,1,&model) == 0); \
        assert(ciglet_rs_scaled_prediction_length_##suffix(model,&count) == 0 && count == 2); \
        assert(ciglet_rs_scaled_prediction_ordinary_##suffix(model,coefficients,correlations,2) == 0); \
        assert(coefficients[0] == 1 && coefficients[1] == 0 && correlations[0] == 1 && correlations[1] == 0); \
        assert(ciglet_rs_scaled_prediction_clone_##suffix(model,&copy) == 0 && copy != model); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&model) == 0 && model == NULL); \
        assert(ciglet_rs_scaled_prediction_coefficients_##suffix(copy,coefficients,2) == 0 && coefficients[0] == 1); \
        assert(ciglet_rs_scaled_prediction_correlation_parts_##suffix(copy,fractions,exponents,2) == 0); \
        assert(fractions[0] == 1 && exponents[0] == 0 && fractions[1] == 0 && exponents[1] == 0); \
        assert(ciglet_rs_scaled_prediction_gain_parts_##suffix(copy,&gain_fraction,&gain_exponent) == 0); \
        assert(gain_fraction == 1 && gain_exponent == 0); \
        assert(ciglet_rs_scaled_prediction_gain_##suffix(copy,&gain) == 0 && gain == 1); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&copy) == 0 && copy == NULL); \
        samples[0] = maximum; coefficients[0] = coefficients[1] = correlations[0] = correlations[1] = 71; \
        assert(ciglet_rs_scaled_prediction_from_samples_##suffix(samples,2,1,&model) == 0); \
        assert(ciglet_rs_scaled_prediction_ordinary_##suffix(model,coefficients,correlations,2) == 3); \
        assert(coefficients[0] == 71 && coefficients[1] == 71 && correlations[0] == 71 && correlations[1] == 71); \
        assert(ciglet_rs_scaled_prediction_gain_##suffix(model,&gain) == 0 && gain == maximum); \
        assert(ciglet_rs_scaled_prediction_correlation_parts_##suffix(model,fractions,exponents,2) == 0); \
        assert(fractions[0] >= 1 && fractions[0] < 2 && exponents[0] > 128); \
        handle_type *retained = model; \
        assert(ciglet_rs_scaled_prediction_from_samples_##suffix(NULL,0,0,&model) == 3 && model == retained); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&model) == 0 && model == NULL); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&model) == 0); \
        type scaled_magnitudes[] = {maximum,maximum,maximum}; \
        assert(ciglet_rs_scaled_prediction_from_magnitude_##suffix(scaled_magnitudes,3,3,0,&model) == 0); \
        assert(ciglet_rs_scaled_prediction_gain_##suffix(model,&gain) == 0 && gain == maximum); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&model) == 0 && model == NULL); \
    } while (0)
    CHECK_SCALED_PREDICTION(float, f32, CigletRsScaledPredictionF32, FLT_MAX);
    CHECK_SCALED_PREDICTION(double, f64, CigletRsScaledPredictionF64, DBL_MAX);
    #undef CHECK_SCALED_PREDICTION
    #define CHECK_PREDICTION_SPECTRUM(type, suffix, handle_type, scaled_type, maximum) do { \
        handle_type *plan = NULL; scaled_type *model = NULL; size_t shape[2] = {71,71}; \
        type input[] = {1,1,1}, coefficients[2], correlations[2], spectrum[4], gain = 71; \
        assert(ciglet_rs_prediction_spectrum_new_##suffix(4,&plan) == 0); \
        assert(ciglet_rs_prediction_spectrum_shape_##suffix(plan,shape,2) == 0 && shape[0] == 4 && shape[1] == 3); \
        for(uint32_t arithmetic=0;arithmetic<2;++arithmetic) { \
            assert(ciglet_rs_prediction_spectrum_estimate_##suffix(plan,input,3,1,0,arithmetic,coefficients,correlations,2) == 0); \
            assert(coefficients[0] == 1 && coefficients[1] == 0 && correlations[0] == 1 && correlations[1] == 0); \
        } \
        assert(ciglet_rs_prediction_spectrum_spectrum_##suffix(plan,coefficients,2,2,spectrum,4) == 0); \
        for(size_t i=0;i<4;++i) assert(spectrum[i] == 2); \
        type pole[] = {1,-1}; \
        for(size_t i=0;i<4;++i) spectrum[i] = 71; \
        assert(ciglet_rs_prediction_spectrum_spectrum_##suffix(plan,pole,2,1,spectrum,4) == 3); \
        for(size_t i=0;i<4;++i) assert(spectrum[i] == 71); \
        assert(ciglet_rs_prediction_spectrum_spectrum_##suffix(plan,coefficients,2,2,spectrum,4) == 0); \
        handle_type *retained = plan; \
        assert(ciglet_rs_prediction_spectrum_new_##suffix(3,&plan) == 3 && plan == retained); \
        type extreme[] = {maximum,maximum,maximum}; \
        assert(ciglet_rs_prediction_spectrum_estimate_scaled_##suffix(plan,extreme,3,3,0,&model) == 0); \
        assert(ciglet_rs_prediction_spectrum_release_##suffix(&plan) == 0 && plan == NULL); \
        assert(ciglet_rs_scaled_prediction_gain_##suffix(model,&gain) == 0 && gain == maximum); \
        assert(ciglet_rs_scaled_prediction_release_##suffix(&model) == 0 && model == NULL); \
        assert(ciglet_rs_prediction_spectrum_release_##suffix(&plan) == 0); \
    } while (0)
    CHECK_PREDICTION_SPECTRUM(float, f32, CigletRsPredictionSpectrumF32, CigletRsScaledPredictionF32, FLT_MAX);
    CHECK_PREDICTION_SPECTRUM(double, f64, CigletRsPredictionSpectrumF64, CigletRsScaledPredictionF64, DBL_MAX);
    #undef CHECK_PREDICTION_SPECTRUM
    #define CHECK_RESONANCES(type, suffix) do { \
        type coefficients[] = {1,0,1}, frequencies[3] = {71,71,71}, real[3] = {71,71,71}, imaginary[3] = {71,71,71}; \
        size_t count = 71, iterations = 71; uint32_t converged = 71; double residual = 71; \
        assert(ciglet_rs_prediction_resonances_##suffix(coefficients,3,frequencies,real,imaginary,3,&count,&iterations,&converged,&residual) == 0); \
        assert(count == 1 && iterations > 0 && iterations <= 50 && converged == 1 && residual < 1e-6); \
        assert(fabs((double)frequencies[0]-0.5) < 1e-6 && fabs((double)real[0]) < 1e-6 && fabs((double)imaginary[0]-1) < 1e-6); \
        assert(frequencies[1] == 71 && frequencies[2] == 71 && real[1] == 71 && real[2] == 71 && imaginary[1] == 71 && imaginary[2] == 71); \
        count = iterations = converged = 71; residual = 71; \
        assert(ciglet_rs_prediction_resonances_##suffix(coefficients,3,NULL,NULL,NULL,0,&count,&iterations,&converged,&residual) == 2); \
        assert(count == 71 && iterations == 71 && converged == 71 && residual == 71); \
        type constant[] = {1}; \
        assert(ciglet_rs_prediction_resonances_##suffix(constant,1,NULL,NULL,NULL,0,&count,&iterations,&converged,&residual) == 0); \
        assert(count == 0 && iterations == 0 && converged == 1 && residual == 0); \
    } while (0)
    CHECK_RESONANCES(float, f32);
    CHECK_RESONANCES(double, f64);
    #undef CHECK_RESONANCES

    #define CHECK_MATRIX(type, suffix, matrix_type, lu_type) do { \
        matrix_type *a = NULL, *b = NULL, *product = NULL, *copy = NULL; \
        lu_type *factor = NULL, *factor_copy = NULL; \
        type input[] = {4,2,1,4,2,0,1,2,3}, values[9], right[] = {15,12,10}, solution[3], value = 71; \
        size_t shape[3], swaps[3]; \
        assert(ciglet_rs_matrix_from_column_major_##suffix(3,3,input,9,&a) == 0); \
        assert(ciglet_rs_matrix_shape_##suffix(a,shape,3) == 0 && shape[0] == 3 && shape[1] == 3 && shape[2] == 9); \
        assert(ciglet_rs_matrix_get_##suffix(a,2,2,&value) == 0 && value == 3); \
        assert(ciglet_rs_matrix_identity_##suffix(3,&b) == 0); \
        assert(ciglet_rs_matrix_multiply_##suffix(a,b,&product) == 0); \
        assert(ciglet_rs_matrix_copy_column_major_##suffix(product,values,9) == 0); \
        for(size_t i=0;i<9;++i) assert(values[i] == input[i]); \
        assert(ciglet_rs_matrix_clone_##suffix(a,&copy) == 0); \
        assert(ciglet_rs_matrix_pivot_rows_##suffix(copy,swaps,3) == 0); \
        assert(swaps[0] == 0 && swaps[1] == 1 && swaps[2] == 2); \
        assert(ciglet_rs_lu_factor_##suffix(copy,1,&factor) == 3 && factor == NULL); \
        assert(ciglet_rs_lu_factor_##suffix(a,0,&factor) == 0); \
        assert(ciglet_rs_lu_clone_##suffix(factor,&factor_copy) == 0); \
        assert(ciglet_rs_lu_release_##suffix(&factor) == 0 && factor == NULL); \
        assert(ciglet_rs_lu_shape_##suffix(factor_copy,shape,3) == 0 && shape[0] == 3 && shape[1] == 3 && shape[2] == 9); \
        assert(ciglet_rs_lu_copy_packed_##suffix(factor_copy,values,9) == 0); \
        assert(ciglet_rs_lu_copy_row_swaps_##suffix(factor_copy,swaps,3) == 0); \
        assert(ciglet_rs_matrix_apply_row_swaps_##suffix(copy,swaps,3) == 0); \
        assert(ciglet_rs_matrix_apply_swaps_##suffix(right,3,swaps,3,solution,3) == 0); \
        assert(ciglet_rs_lu_solve_##suffix(factor_copy,right,3,solution,3) == 0); \
        for(size_t i=0;i<3;++i) assert(fabs((double)solution[i]-(double)(i+1)) < 2e-6); \
        assert(ciglet_rs_matrix_multiply_vector_##suffix(a,solution,3,solution,3) == 0); \
        for(size_t i=0;i<3;++i) assert(fabs((double)solution[i]-(double)right[i]) < 2e-6); \
        assert(ciglet_rs_lu_release_##suffix(&factor_copy) == 0 && factor_copy == NULL); \
        assert(ciglet_rs_matrix_release_##suffix(&a) == 0 && a == NULL); \
        assert(ciglet_rs_matrix_release_##suffix(&b) == 0 && b == NULL); \
        assert(ciglet_rs_matrix_release_##suffix(&copy) == 0 && copy == NULL); \
        assert(ciglet_rs_matrix_release_##suffix(&product) == 0 && product == NULL); \
        assert(ciglet_rs_matrix_zeros_##suffix(2,0,&a) == 0); \
        assert(ciglet_rs_matrix_multiply_vector_##suffix(a,NULL,0,solution,2) == 0 && solution[0] == 0 && solution[1] == 0); \
        assert(ciglet_rs_matrix_release_##suffix(&a) == 0 && a == NULL); \
        assert(ciglet_rs_matrix_release_##suffix(&a) == 0); \
    } while (0)
    CHECK_MATRIX(float, f32, CigletRsMatrixF32, CigletRsLuF32);
    CHECK_MATRIX(double, f64, CigletRsMatrixF64, CigletRsLuF64);
    #undef CHECK_MATRIX

    #define CHECK_FILTERING(type, suffix) do { \
        type impulse[] = {1,0,0}, numerator[] = {0.5,0}, denominator[] = {1,-0.5}, output[4] = {71,71,71,71}; \
        assert(ciglet_rs_filter_##suffix(impulse,3,numerator,2,denominator,2,0,output,3) == 0); \
        assert(output[0] == 0.5 && output[1] == 0.25 && output[2] == 0.125); \
        assert(ciglet_rs_filter_##suffix(impulse,3,numerator,2,denominator,2,1,output,3) == 0); \
        for(size_t i=0;i<3;++i) assert(output[i] == 0); \
        type signal[] = {1,2,3}, coefficients[] = {1,2}, kernel[] = {2,-1}; \
        assert(ciglet_rs_forward_backward_filter_##suffix(signal,3,coefficients,2,NULL,0,0,output,3) == 0); \
        assert(output[0] == 9 && output[1] == 18 && output[2] == 7); \
        assert(ciglet_rs_convolve_##suffix(signal,3,kernel,2,output,4) == 0); \
        assert(output[0] == 2 && output[1] == 3 && output[2] == 4 && output[3] == -3); \
        type taps[4], normalized[4]; \
        for(uint32_t band=0;band<4;++band) { \
            assert(ciglet_rs_windowed_fir_##suffix(4,band,0.125,0.375,0,taps,4) == 0); \
            if(band<2) { \
                assert(ciglet_rs_fir_##suffix(4,0.25,band,0,normalized,4) == 0); \
                for(size_t i=0;i<4;++i) assert(taps[i] == normalized[i]); \
            } else if(band==2) { \
                assert(ciglet_rs_bandpass_fir_##suffix(4,0.25,0.75,0,normalized,4) == 0); \
                for(size_t i=0;i<4;++i) assert(taps[i] == normalized[i]); \
            } \
        } \
        for(size_t i=0;i<4;++i) output[i] = 71; \
        assert(ciglet_rs_filter_##suffix(impulse,3,numerator,2,denominator,2,2,output,3) == 3); \
        assert(ciglet_rs_windowed_fir_##suffix(4,4,0,0,0,output,4) == 3); \
        for(size_t i=0;i<4;++i) assert(output[i] == 71); \
        assert(ciglet_rs_convolve_##suffix(NULL,0,kernel,2,NULL,0) == 0); \
        assert(ciglet_rs_forward_backward_filter_##suffix(NULL,0,NULL,0,NULL,0,0,NULL,0) == 0); \
    } while (0)
    CHECK_FILTERING(float, f32);
    CHECK_FILTERING(double, f64);
    #undef CHECK_FILTERING

    #define CHECK_RESAMPLING(type, suffix) do { \
        type input[] = {0.75}, output[2] = {71,71}; size_t count = 71; \
        assert(ciglet_rs_resample_output_length_##suffix(1,2,&count) == 0 && count == 2); \
        assert(ciglet_rs_resample_##suffix(input,1,2,output,2) == 0 && output[0] == 0.75); \
        assert(ciglet_rs_resample_with_##suffix(input,1,2,1,output,2) == 0 && output[0] == 0 && output[1] == 0); \
        for(uint32_t boundary=0;boundary<2;++boundary) for(uint32_t kernel=0;kernel<2;++kernel) { \
            assert(ciglet_rs_resample_with_kernel_##suffix(input,1,2,boundary,kernel,output,2) == 0); \
            assert(output[0] == (boundary == 0 ? 0.75 : 0)); \
        } \
        output[0] = output[1] = 71; \
        assert(ciglet_rs_resample_with_kernel_##suffix(input,1,2,0,2,output,2) == 3 && output[0] == 71 && output[1] == 71); \
        assert(ciglet_rs_resample_##suffix(input,1,2,output,1) == 2 && output[0] == 71 && output[1] == 71); \
        count = 71; assert(ciglet_rs_resample_output_length_##suffix(1,0,&count) == 3 && count == 71); \
        assert(ciglet_rs_resample_##suffix(NULL,0,2,NULL,0) == 0); \
        assert(ciglet_rs_resample_##suffix(input,1,0.25,NULL,0) == 0); \
        type signed_zero[] = {-0.0}; \
        assert(ciglet_rs_resample_##suffix(signed_zero,1,1,signed_zero,1) == 0 && signbit(signed_zero[0])); \
    } while (0)
    CHECK_RESAMPLING(float, f32);
    CHECK_RESAMPLING(double, f64);
    #undef CHECK_RESAMPLING

    #define CHECK_INTERPOLATION(type, suffix, maximum) do { \
        type scalar = 71, coordinates[] = {0,2,5}, values[] = {1,5,-1}, queries[] = {4,1,-9,10}, output[4] = {71,71,71,71}; \
        assert(ciglet_rs_interpolate_between_##suffix(-maximum,maximum,0.5,&scalar) == 0 && scalar == 0); \
        assert(ciglet_rs_interpolate_linear_##suffix(coordinates,values,3,queries,4,output,4) == 0); \
        assert(output[0] == 1 && output[1] == 3 && output[2] == 1 && output[3] == -1); \
        type uniform[] = {1,3,5}, positions[] = {1.5,-1,8,0}; \
        assert(ciglet_rs_interpolate_uniform_##suffix(0,3,uniform,3,positions,4,output,4) == 0); \
        assert(output[0] == 4 && output[1] == 1 && output[2] == 5 && output[3] == 1); \
        type impulse[] = {7,0}, distant[] = {0,100,-100}; \
        assert(ciglet_rs_interpolate_sinc_##suffix(0,2,impulse,2,distant,3,output,3) == 0); \
        assert(output[0] == 7 && output[1] == 0 && output[2] == 0); \
        for(uint32_t boundary=0;boundary<2;++boundary) { \
            assert(ciglet_rs_interpolate_sinc_with_##suffix(0,2,impulse,2,distant,3,boundary,output,3) == 0); \
            assert(output[0] == (boundary == 0 ? 7 : 0)); \
            for(uint32_t kernel=0;kernel<2;++kernel) { \
                assert(ciglet_rs_interpolate_sinc_with_kernel_##suffix(0,2,impulse,2,distant,3,boundary,kernel,output,3) == 0); \
                assert(output[0] == (boundary == 0 ? 7 : 0) && output[1] == 0 && output[2] == 0); \
            } \
        } \
        type gaps[] = {NAN,2,NAN,6,NAN}; \
        assert(ciglet_rs_interpolate_fill_missing_##suffix(gaps,5,NAN,gaps,5) == 0); \
        assert(gaps[0] == 2 && gaps[1] == 2 && gaps[2] == 4 && gaps[3] == 6 && gaps[4] == 6); \
        for(size_t i=0;i<4;++i) output[i] = 71; \
        assert(ciglet_rs_interpolate_sinc_with_kernel_##suffix(0,2,impulse,2,distant,3,0,2,output,3) == 3); \
        assert(ciglet_rs_interpolate_linear_##suffix(coordinates,values,3,queries,4,output,3) == 2); \
        for(size_t i=0;i<4;++i) assert(output[i] == 71); \
        assert(ciglet_rs_interpolate_uniform_##suffix(0,2,impulse,2,NULL,0,NULL,0) == 0); \
        assert(ciglet_rs_interpolate_fill_missing_##suffix(NULL,0,NAN,NULL,0) == 0); \
    } while (0)
    CHECK_INTERPOLATION(float, f32, FLT_MAX);
    CHECK_INTERPOLATION(double, f64, DBL_MAX);
    #undef CHECK_INTERPOLATION

    #define CHECK_SMOOTHING(type, suffix, maximum) do { \
        type signal[] = {1,2,8}, output[4] = {71,71,71,71}, selected[3]; \
        assert(ciglet_rs_moving_average_##suffix(signal,3,1,output,3) == 0); \
        assert(output[0] == 1.5 && output[1] == 5 && output[2] == 8); \
        assert(ciglet_rs_moving_average_with_##suffix(signal,3,1,0,selected,3) == 0); \
        for(size_t i=0;i<3;++i) assert(output[i] == selected[i]); \
        assert(ciglet_rs_moving_average_with_##suffix(signal,3,0,1,selected,3) == 0); \
        for(size_t i=0;i<3;++i) assert(signal[i] == selected[i]); \
        type signed_input[] = {-3,4}; \
        assert(ciglet_rs_moving_rms_##suffix(signed_input,2,0,output,2) == 0 && output[0] == 3 && output[1] == 4); \
        assert(ciglet_rs_moving_rms_with_##suffix(signed_input,2,0,0,selected,2) == 0 && selected[0] == 3 && selected[1] == 4); \
        type peak[] = {maximum}; \
        assert(ciglet_rs_moving_rms_##suffix(peak,1,1,output,1) == 0 && output[0] == maximum); \
        output[0] = 71; \
        assert(ciglet_rs_moving_rms_with_##suffix(peak,1,1,1,output,1) == 3 && output[0] == 71); \
        type median[] = {4,1,9,2}; \
        assert(ciglet_rs_median_filter_##suffix(median,4,2,median,4) == 0); \
        assert(median[0] == 4 && median[1] == 2.5 && median[2] == 5 && median[3] == 2); \
        for(size_t i=0;i<4;++i) output[i] = 71; \
        assert(ciglet_rs_moving_average_with_##suffix(signal,3,1,2,output,3) == 3); \
        assert(ciglet_rs_median_filter_##suffix(median,4,0,output,4) == 3); \
        for(size_t i=0;i<4;++i) assert(output[i] == 71); \
        assert(ciglet_rs_moving_average_##suffix(NULL,0,1,NULL,0) == 0); \
        assert(ciglet_rs_moving_rms_##suffix(NULL,0,1,NULL,0) == 0); \
        assert(ciglet_rs_median_filter_##suffix(NULL,0,3,NULL,0) == 0); \
    } while (0)
    CHECK_SMOOTHING(float, f32, FLT_MAX);
    CHECK_SMOOTHING(double, f64, DBL_MAX);
    #undef CHECK_SMOOTHING
    #define CHECK_SINC(type, suffix, maximum, tolerance) do { \
        type value = 71, selected = 71, derivative = 71; \
        assert(ciglet_rs_sinc_aliased_##suffix(2,0,&value) == 0 && value == 2); \
        assert(ciglet_rs_sinc_aliased_with_##suffix(2,0,0,&selected) == 0 && selected == value); \
        assert(ciglet_rs_sinc_aliased_with_##suffix(2,0,1,&selected) == 0 && selected == 2); \
        assert(ciglet_rs_sinc_derivative_##suffix(2,1,&derivative) == 0); \
        assert(fabs((double)derivative + sin(0.5)) < tolerance); \
        assert(ciglet_rs_sinc_derivative_with_##suffix(2,1,0,&selected) == 0 && selected == derivative); \
        assert(ciglet_rs_sinc_derivative_with_##suffix(3,0,1,&selected) == 0 && selected == 0); \
        assert(ciglet_rs_sinc_aliased_##suffix(maximum,0,&value) == 0 && value == maximum); \
        value = selected = derivative = 71; \
        assert(ciglet_rs_sinc_aliased_##suffix(NAN,0,&value) == 3 && value == 71); \
        assert(ciglet_rs_sinc_derivative_##suffix(2,INFINITY,&derivative) == 3 && derivative == 71); \
        assert(ciglet_rs_sinc_aliased_with_##suffix(2,0,2,&selected) == 3 && selected == 71); \
        assert(ciglet_rs_sinc_derivative_with_##suffix(2,0,2,&selected) == 3 && selected == 71); \
        assert(ciglet_rs_sinc_aliased_##suffix(2,0,NULL) == 1); \
        assert(ciglet_rs_sinc_derivative_##suffix(2,0,NULL) == 1); \
    } while (0)
    CHECK_SINC(float, f32, FLT_MAX, 1e-7);
    CHECK_SINC(double, f64, DBL_MAX, 2e-15);
    #undef CHECK_SINC
    #define CHECK_SYNTHESIS(type, suffix) do { \
        type frequencies[] = {0,0}, amplitudes[] = {0.25,-0.5}, phases[] = {0,0}, output[3] = {71,71,71}; \
        assert(ciglet_rs_sinusoid_##suffix(0,0.25,0,16000,3,output,3) == 0); \
        for(size_t i=0;i<3;++i) assert(output[i] == 0.25); \
        assert(ciglet_rs_sinusoids_##suffix(frequencies,amplitudes,phases,2,16000,3,output,3) == 0); \
        for(size_t i=0;i<3;++i) assert(output[i] == -0.25); \
        assert(ciglet_rs_sinusoids_##suffix(output,amplitudes,phases,0,16000,3,output,3) == 0); \
        for(size_t i=0;i<3;++i) assert(output[i] == 0); \
        output[0] = output[1] = output[2] = 71; \
        assert(ciglet_rs_sinusoid_##suffix(NAN,1,0,16000,3,output,3) == 3); \
        assert(ciglet_rs_sinusoid_##suffix(0,1,0,16000,3,output,2) == 2); \
        assert(ciglet_rs_sinusoids_##suffix(NULL,amplitudes,phases,2,16000,3,output,3) == 1); \
        assert(ciglet_rs_sinusoids_##suffix(frequencies,amplitudes,phases,2,0,3,output,3) == 3); \
        for(size_t i=0;i<3;++i) assert(output[i] == 71); \
        assert(ciglet_rs_sinusoid_##suffix(0,1,0,16000,0,NULL,0) == 0); \
        assert(ciglet_rs_sinusoids_##suffix(NULL,NULL,NULL,0,16000,0,NULL,0) == 0); \
        type shared[] = {0,0,0}; \
        assert(ciglet_rs_sinusoids_##suffix(shared,amplitudes,phases,1,16000,3,shared,3) == 0); \
        for(size_t i=0;i<3;++i) assert(shared[i] == 0.25); \
    } while (0)
    CHECK_SYNTHESIS(float, f32);
    CHECK_SYNTHESIS(double, f64);
    #undef CHECK_SYNTHESIS
    int32_t sequence[] = {91, 91, 91};
    assert(ciglet_rs_integer_sequence(INT32_MAX, 1, sequence, 3) == 3);
    assert(sequence[0] == 91 && sequence[1] == 91 && sequence[2] == 91);
    assert(ciglet_rs_integer_sequence(-2, 3, sequence, 3) == 0);
    assert(sequence[0] == -2 && sequence[1] == 1 && sequence[2] == 4);
    return 0;
}
