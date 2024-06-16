use crate::dimension::shape::MAX_SHAPE_DIMS;

#[cfg(feature = "small_matrix_dimensions")]
pub type StrideType = i16;

#[cfg(not(feature = "small_matrix_dimensions"))]
pub type StrideType = i64;

pub struct Stride {
    offset: usize, // Offset cannot be negative
    strides: [StrideType; MAX_SHAPE_DIMS],
}
