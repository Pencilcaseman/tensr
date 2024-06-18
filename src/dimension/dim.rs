use crate::types::{DimLen, UDim};

/// Traits and functions required for anything that can represent the
/// dimensions of an object.
pub(crate) trait Dimension:
    std::fmt::Debug + Clone + std::ops::Index<DimLen> + std::ops::IndexMut<DimLen>
{
    fn len(&self) -> DimLen;
}

/// Represents the number of dimensions stored by an object
#[derive(Debug, Clone)]
pub(crate) struct Dim<Index> {
    index: Index,
}

impl<Index> Dim<Index> {
    /// Creates a new [`Dim<Index>`].
    pub(crate) fn new(index: Index) -> Self {
        Self { index }
    }

    /// Access to the value of type [`Index`] in this [`Dim<Index>`]
    pub(crate) fn get(&self) -> &Index {
        &self.index
    }

    /// Mutable access to the value of type [`Index`] in this [`Dim<Index>`]
    pub(crate) fn get_mut(&mut self) -> &mut Index {
        &mut self.index
    }
}

macro_rules! dim_def {
    ($($n: literal),*) => {
       $(
        paste::paste! {
            pub type [< Dim $n >] = Dim<[UDim; $n]>;
            impl Dimension for [< Dim $n >] {
                fn len(&self) -> DimLen {
                    $n
                }
            }

            impl std::ops::Index<DimLen> for [< Dim $n >] {
                type Output = UDim;

                fn index(&self, index: DimLen) -> &Self::Output {
                    &self.index[index as usize]
                }
            }

            impl std::ops::IndexMut<DimLen> for [< Dim $n >] {
                fn index_mut(&mut self, index: DimLen) -> &mut Self::Output {
                    &mut self.index[index as usize]
                }
            }
        }
       )*
    };
}

dim_def!(0, 1, 2, 3, 4, 5, 6, 7, 8);
