use crate::dimension::dim::Dimension;

pub struct Stride<DimType: Dimension> {
    offset: usize, // Offset cannot be negative
    strides: DimType,
}
