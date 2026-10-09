//! Natural C array descriptors for portable lane operations.
/// 4 ordered f32 lanes with natural C array layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsFloats4 {
    pub values: [f32; 4],
}
/// 4 ordered i32 lanes with natural C array layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsIntegers4 {
    pub values: [i32; 4],
}
/// 4 ordered u32 lanes with natural C array layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsBits4 {
    pub values: [u32; 4],
}
/// 2 ordered i64 lanes with natural C array layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CigletRsIntegers64x2 {
    pub values: [i64; 2],
}
