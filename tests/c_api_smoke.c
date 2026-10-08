/* Link this independent caller against the generated C interface. */
#include "ciglet_rs.h"
#include <assert.h>
#include <string.h>

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
    int32_t sequence[] = {91, 91, 91};
    assert(ciglet_rs_integer_sequence(INT32_MAX, 1, sequence, 3) == 3);
    assert(sequence[0] == 91 && sequence[1] == 91 && sequence[2] == 91);
    assert(ciglet_rs_integer_sequence(-2, 3, sequence, 3) == 0);
    assert(sequence[0] == -2 && sequence[1] == 1 && sequence[2] == 4);
    return 0;
}
