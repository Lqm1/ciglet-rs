# Upstream bugs to address

These findings refer to ciglet commit 895ba9b1c0eabee83d3544208bbc82420efa3206. Recording a finding does not establish that its Rust replacement is implemented or verified.

## Single-sinusoid amplitude and short lengths

The inline gensin function in ciglet.h accepts an amplitude but never uses it. Its initial samples and recurrence produce a unit-amplitude sinusoid regardless of the amplitude argument. It also unconditionally writes indices 0 and 1, so lengths below two access outside the allocated buffer. The Rust synthesis::sinusoid implementation now applies amplitude to its initial recurrence samples. Both synthesis entry points handle zero and one sample lengths safely. tests/synthesis.rs verifies zero, fractional, negative, and greater-than-one amplitudes against an independent centered cosine formula, plus short inputs. Compatibility fixtures use the unmodified single-sinusoid C function only with unit amplitude; multisinusoid C fixtures include nonunit amplitudes. These are intentional corrections, not claims that the original ignored-amplitude behavior is retained.
