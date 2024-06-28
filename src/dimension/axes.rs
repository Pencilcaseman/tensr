use crate::dimension::dim::Dimension;
use crate::types::{DimLen, UDim};

pub struct Axes<Dim: Dimension> {
    pub(crate) shape: Dim,
    pub(crate) stride: Dim,
}

impl<Dim: Dimension> Axes<Dim> {
    pub fn new_with_default_stride(shape: Dim) -> Self
    where
        Dim::IndexScalar: std::ops::MulAssign,
    {
        let mut stride = Dim::zero();

        let mut l = shape.ndim();
        let mut s = Dim::IndexScalar::from(1u16);
        for i in 0..l {
            let j = l - i - 1;

            // Safety: We are constructing a new Dim, so we know that the
            // stride is valid for a contiguous array
            unsafe {
                stride.get_mut()[j as usize] = s.clone();
            }

            s *= shape[j as DimLen].clone();
        }

        Self { shape, stride }
    }
}
