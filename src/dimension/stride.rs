use crate::dimension::shape::{DimensionType, MAX_SHAPE_DIMS};

pub struct Stride {
    offset: DimensionType,
    strides: [DimensionType; MAX_SHAPE_DIMS],
}
