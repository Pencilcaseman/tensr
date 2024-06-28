use crate::types::{DimLen, UDim};

#[macro_export]
macro_rules! repeat_for_dims {
    ($macro: tt) => {
        $macro!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    };
}

/// Traits and functions required for anything that can represent the
/// dimensions of an object.
pub trait Dimension:
    std::fmt::Debug + Clone + std::ops::Index<DimLen, Output = Self::IndexScalar>
{
    type IndexScalar: From<u16>
        + std::ops::Mul
        + std::ops::MulAssign
        + Copy
        + std::fmt::Debug;
    type Index: std::ops::Index<usize, Output = Self::IndexScalar>
        + std::ops::IndexMut<usize>
        + Clone;

    fn zero() -> Self;
    fn len(&self) -> DimLen;
    fn size(&self) -> usize;

    unsafe fn get_mut(&mut self) -> &mut Self::Index;
}

/// Represents the number of dimensions stored by an object
pub struct Dim<Index> {
    pub(crate) index: Index,
}

impl<Index> Dim<Index> {
    /// Creates a new [`Dim<Index>`].
    pub fn new(index: Index) -> Self {
        Self { index }
    }

    /// Access to the value of type `Index` in this [`Dim<Index>`]
    ///
    /// # Example
    /// ```rust
    /// use tensr::dimension::dim::Dim2;
    ///
    /// // Represents a 3x4 structure
    /// let dim_2d = Dim2::new([3, 4]);
    /// assert_eq!(dim_2d.get()[0], 3);
    /// assert_eq!(dim_2d.get()[1], 4);
    /// ```
    pub fn get(&self) -> &Index {
        &self.index
    }
}

/*impl<Index> std::ops::Index<DimLen> for Dim<Index> {
    type Output = Index;

    fn index(&self, index: DimLen) -> &Self::Output {
        &self.index[index as usize]
    }
}*/

macro_rules! dim_def {
    ($($n: literal),*) => {
       $(
        paste::paste! {
            pub type [< Dim $n >] = Dim<[UDim; $n]>;
            impl Dimension for [< Dim $n >] {
                type IndexScalar = UDim;
                type Index = [UDim; $n];

                fn zero() -> Self {
                    Self::new([UDim::from(0u16); $n])
                }

                fn len(&self) -> DimLen {
                    $n
                }

                fn size(&self) -> usize {
                    (0..$n).into_iter().fold(1, |acc, i| acc * self.index[i])
                }

                unsafe fn get_mut(&mut self) -> &mut Self::Index {
                    &mut self.index
                }
            }

            impl std::fmt::Debug for [< Dim $n >] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{:?}", self.index))
                }
            }

            impl std::ops::Index<DimLen> for [< Dim $n >] {
                type Output = UDim;

                fn index(&self, index: DimLen) -> &Self::Output {
                    &self.index[index as usize]
                }
            }

            impl Clone for [< Dim $n >] {
                fn clone(&self) -> Self {
                    Self::new(self.index.clone())
                }
            }
        }
       )*
    };
}

repeat_for_dims!(dim_def);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_dim {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim $dim >]() {
                        let mut data = [0; $dim];
                        for i in 0..$dim {
                            data[i] = i + 1;
                        }
                        let dim = [< Dim $dim >]::new(data);

                        assert_eq!(dim.len(), $dim);

                        for i in 0..$dim {
                            assert_eq!(dim.get()[i], i + 1);
                            assert_eq!(dim[i as DimLen], i + 1);
                        }
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_mut {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim $dim _mut >]() {
                        let mut data = [0; $dim];
                        for i in 0..$dim {
                            data[i as usize] = i + 1;
                        }
                        let mut dim = [< Dim $dim >]::new(data);

                        assert_eq!(dim.len(), $dim);

                        for i in 0..$dim {
                            assert_eq!(dim.get()[i], i + 1);
                            assert_eq!(dim[i as DimLen], i + 1);
                        }

                        unsafe {
                            for i in 0..$dim {
                                dim.get_mut()[i as usize] = i + 2;
                            }
                        }

                        for i in 0..$dim {
                            assert_eq!(dim.get()[i], i + 2);
                            assert_eq!(dim[i as DimLen], i + 2);
                        }
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_size {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_size_ $dim >]() {
                        let mut data = [0; $dim];
                        let mut target = 1;
                        for i in 0..$dim {
                            data[i] = i + 1;
                            target *= data[i];
                        }
                        let dim = [< Dim $dim >]::new(data);

                        assert_eq!(dim.size(), target);
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_str {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_str_ $dim >]() {
                        let mut data = [0; $dim];
                        for i in 0..$dim {
                            data[i] = i + 1;
                        }
                        let dim = [< Dim $dim >]::new(data);

                        assert_eq!(format!("{dim:?}"), format!("{data:?}"));
                    }
                }
            )*
        };
    }

    repeat_for_dims!(test_dim);
    repeat_for_dims!(test_dim_mut);
    repeat_for_dims!(test_dim_size);
    repeat_for_dims!(test_dim_str);
}
