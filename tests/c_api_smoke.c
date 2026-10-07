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
    int32_t sequence[] = {91, 91, 91};
    assert(ciglet_rs_integer_sequence(INT32_MAX, 1, sequence, 3) == 3);
    assert(sequence[0] == 91 && sequence[1] == 91 && sequence[2] == 91);
    assert(ciglet_rs_integer_sequence(-2, 3, sequence, 3) == 0);
    assert(sequence[0] == -2 && sequence[1] == 1 && sequence[2] == 4);
    return 0;
}
