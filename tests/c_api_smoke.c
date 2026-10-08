/* Link this independent caller against the generated C interface. */
#ifdef NDEBUG
/* Zig's optimized modes define NDEBUG. Always execute this caller's checks. */
#undef NDEBUG
#endif
#include "ciglet_rs.h"
#include <assert.h>
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
    int32_t sequence[] = {91, 91, 91};
    assert(ciglet_rs_integer_sequence(INT32_MAX, 1, sequence, 3) == 3);
    assert(sequence[0] == 91 && sequence[1] == 91 && sequence[2] == 91);
    assert(ciglet_rs_integer_sequence(-2, 3, sequence, 3) == 0);
    assert(sequence[0] == -2 && sequence[1] == 1 && sequence[2] == 4);
    return 0;
}
