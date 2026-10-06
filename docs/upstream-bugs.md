# Upstream bugs to address

These findings refer to ciglet commit 895ba9b1c0eabee83d3544208bbc82420efa3206. Recording a finding does not establish that its Rust replacement is implemented or verified.

## Single-sinusoid amplitude and short lengths

The inline gensin function in ciglet.h accepts an amplitude but never uses it. Its initial samples and recurrence produce a unit-amplitude sinusoid regardless of the amplitude argument. It also unconditionally writes indices 0 and 1, so lengths below two access outside the allocated buffer. The pending Rust synthesis implementation must apply the requested amplitude and define short-input behavior, with independent formula checks. Disclose this intentional correction separately from compatibility checks using unit amplitude.
