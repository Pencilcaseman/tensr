use crate::{
    dimension::dim::{Dim, Dimension},
    types::{DimLen, UDim},
};

/// The number of elements to store on the stack for a [`DynIndex`] type.
///
/// The choice of 4 here is somewhat arbitrary. Using a value of 2 makes
/// the enum [`DynIndex`] a bit nicer, as all values are the same size
/// (24 bytes), though the cost of pointer indirection likely makes this
/// optimisation detremental. Additionally, arrays with two or three
/// dimensions are quite common, so stack-allocating the data for these
/// cases will likely yield a performance improvement in general.
const MAX_STACK_DIMS: usize = 4;

pub enum DynIndex {
    /// Stack-allocated array storing `.0` dimensions
    Stack(DimLen, [UDim; MAX_STACK_DIMS]),

    /// Heap-allocated array for storing arbitrarily high dimensional data
    Heap(Box<[UDim]>),
}

pub type DimDyn = Dim<DynIndex>;

impl Dimension for DimDyn {
    fn len(&self) -> DimLen {
        match self.get() {
            DynIndex::Stack(l, _) => *l,
            DynIndex::Heap(b) => b.len() as DimLen,
        }
    }

    fn size(&self) -> usize {
        match self.get() {
            DynIndex::Stack(_, h) => h.iter().fold(1, |acc, n| acc * n),
            DynIndex::Heap(b) => b.iter().fold(1, |acc, n| acc * n),
        }
    }
}

impl std::fmt::Debug for DimDyn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('[');

        match self.get() {
            DynIndex::Stack(l, h) => {
                for (i, v) in h.iter().enumerate() {
                    s.push_str(&format!("{v}"));
                    if i + 1 < *l as usize {
                        s.push_str(", ");
                    }
                }
            }
            DynIndex::Heap(b) => {
                for (i, v) in b.iter().enumerate() {
                    s.push_str(&format!("{v}"));
                    if i + 1 < b.len() as usize {
                        s.push_str(", ");
                    }
                }
            }
        }

        s.push(']');
        f.write_str(&s)
    }
}

impl Clone for DimDyn {
    fn clone(&self) -> Self {
        match self.get() {
            DynIndex::Stack(l, h) => Self::new(DynIndex::Stack(*l, h.clone())),
            DynIndex::Heap(b) => Self::new(DynIndex::Heap(b.clone())),
        }
    }
}

impl std::ops::Index<DimLen> for DimDyn {
    type Output = UDim;

    fn index(&self, index: DimLen) -> &Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: DimLen, len: DimLen) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        if index >= self.len() {
            assert_failed(index, self.len());
        }

        match self.get() {
            DynIndex::Stack(_, h) => &h[index as usize],
            DynIndex::Heap(b) => &b[index as usize],
        }
    }
}

// impl std::ops::IndexMut<DimLen> for DimDyn {
//     fn index_mut(&mut self, index: DimLen) -> &mut Self::Output {
//         #[cold]
//         #[inline(never)]
//         #[track_caller]
//         fn assert_failed(index: DimLen, len: DimLen) -> ! {
//             panic!("index (is {index}) must be <= len (is {len})");
//         }

//         if index >= self.len() {
//             assert_failed(index, self.len());
//         }

//         unsafe {
//             match self.get_mut() {
//                 DynIndex::Stack(_, h) => &mut h[index as usize],
//                 DynIndex::Heap(b) => &mut b[index as usize],
//             }
//         }
//     }
// }
