use std::ops::Deref;

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

#[derive(Debug)]
pub enum DynIndex {
    /// Stack-allocated array storing `.0` dimensions
    Stack(DimLen, [UDim; MAX_STACK_DIMS]),

    /// Heap-allocated array for storing arbitrarily high dimensional data
    Heap(Box<[UDim]>),
}

impl Clone for DynIndex {
    fn clone(&self) -> Self {
        match self {
            Self::Stack(l, h) => Self::Stack(*l, h.clone()),
            Self::Heap(b) => Self::Heap(b.clone()),
        }
    }
}

impl DynIndex {
    pub fn zero() -> Self {
        Self::Stack(0, [0; MAX_STACK_DIMS])
    }

    pub fn len(&self) -> DimLen {
        match self {
            Self::Stack(l, _) => *l,
            Self::Heap(h) => h.len() as DimLen,
        }
    }
}

impl Deref for DynIndex {
    type Target = [UDim];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Stack(len, stack) => &stack[0..(*len as usize)],
            Self::Heap(heap) => &heap[0..heap.len()],
        }
    }
}

macro_rules! dyn_index_index {
    ($t: ty) => {
        impl std::ops::Index<$t> for DynIndex {
            type Output = UDim;

            fn index(&self, index: $t) -> &Self::Output {
                #[cold]
                #[inline(never)]
                #[track_caller]
                fn assert_failed(index: DimLen, len: DimLen) -> ! {
                    panic!("index (is {index}) must be <= len (is {len})");
                }

                if index as DimLen >= self.len() {
                    assert_failed(index as DimLen, self.len())
                }

                match self {
                    Self::Stack(_, s) => &s[index as usize],
                    Self::Heap(h) => &h[index as usize],
                }
            }
        }

        impl std::ops::IndexMut<$t> for DynIndex {
            fn index_mut(&mut self, index: $t) -> &mut Self::Output {
                #[cold]
                #[inline(never)]
                #[track_caller]
                fn assert_failed(index: DimLen, len: DimLen) -> ! {
                    panic!("index (is {index}) must be <= len (is {len})");
                }

                if index as DimLen >= self.len() {
                    assert_failed(index as DimLen, self.len())
                }

                match self {
                    Self::Stack(_, s) => &mut s[index as usize],
                    Self::Heap(h) => &mut h[index as usize],
                }
            }
        }
    };
}

dyn_index_index!(usize);
dyn_index_index!(DimLen);

/// Represents a dynamically-dimensioned container. For example, this may
/// be useful when reading an array from a file or user-input.
pub type DimDyn = Dim<DynIndex>;

impl DimDyn {
    /// Create a new [`DimDyn`] instance from a value. The value must be
    /// convertable into a `&[V]`, where `V` can be copied and cast
    /// into a [`UDim`].
    pub fn new_from<T, V>(index: T) -> Self
    where
        V: Copy,
        T: Deref<Target = [V]>,
        UDim: From<V>,
    {
        Self::new(match index.len() {
            stack if stack as DimLen <= MAX_STACK_DIMS as DimLen => {
                let mut data = [0; MAX_STACK_DIMS];
                for i in 0..stack {
                    data[i as usize] = UDim::from(index[i]);
                }

                DynIndex::Stack(stack as DimLen, data)
            }
            heap => {
                let mut data = Vec::with_capacity(heap as usize);

                for i in 0..heap {
                    data.push(UDim::from(index[i]));
                }

                DynIndex::Heap(data.into_boxed_slice())
            }
        })
    }
}

impl Dimension for DimDyn {
    type IndexScalar = UDim;
    type Index = DynIndex;

    fn zero() -> Self {
        Self::new(DynIndex::zero())
    }

    fn len(&self) -> DimLen {
        match self.get() {
            DynIndex::Stack(l, _) => *l,
            DynIndex::Heap(b) => b.len() as DimLen,
        }
    }

    fn size(&self) -> usize {
        match self.get() {
            DynIndex::Stack(l, h) => {
                (0..(*l as usize)).into_iter().fold(1, |acc, n| acc * h[n])
            }
            DynIndex::Heap(b) => b.iter().fold(1, |acc, n| acc * n),
        }
    }

    unsafe fn get_mut(&mut self) -> &mut Self::Index {
        &mut self.index
    }
}

impl std::fmt::Debug for DimDyn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('[');

        match self.get() {
            DynIndex::Stack(l, stack) => {
                for i in 0..*l {
                    s.push_str(&format!("{}", stack[i as usize]));
                    if i + 1 < *l {
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! repeat_for_dim_dyn {
        ($macro: tt) => {
            $macro!(0, 1, 2, 3, 4, 5, 8, 10, 12, 16);
        };
    }

    macro_rules! test_dim_dyn {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_dyn $dim >]() {
                        let mut data = Vec::with_capacity($dim);
                        for i in 0usize..$dim {
                            data.push(i + 1);
                        }

                        let dim = DimDyn::new_from(data);

                        assert_eq!(dim.len(), $dim);

                        for i in 0..$dim {
                            assert_eq!(dim[i], (i + 1) as UDim);
                        }
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_dyn_mut {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_dyn_mut $dim >]() {
                        let mut data = Vec::with_capacity($dim);
                        for i in 0usize..$dim {
                            data.push(i + 1);
                        }

                        let mut dim = DimDyn::new_from(data);

                        assert_eq!(dim.len(), $dim);

                        for i in 0..$dim {
                            assert_eq!(dim[i], (i + 1) as UDim);
                        }

                        unsafe {
                            for i in (0 as UDim)..$dim {
                                dim.get_mut()[i] = i + 2;
                            }
                        }

                        for i in 0..$dim {
                            assert_eq!(dim[i], (i + 2) as UDim);
                        }
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_dyn_size {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_dyn_size_ $dim >]() {
                        let mut data = Vec::with_capacity($dim);
                        let mut target = 1;
                        for i in 0..$dim {
                            data.push(i + 1);
                            target *= data[i];
                        }

                        let dim = DimDyn::new_from(data);

                        assert_eq!(dim.size(), target);
                    }
                }
            )*
        };
    }

    macro_rules! test_dim_dyn_str {
        ($($dim: literal),*) => {
            $(
                paste::paste! {
                    #[test]
                    pub fn [< test_dim_dyn_str_ $dim >]() {
                        let mut data = Vec::with_capacity($dim);
                        for i in 0usize..$dim {
                            data.push(i + 1);
                        }

                        let dim = DimDyn::new_from(data.clone());

                        assert_eq!(format!("{dim:?}"), format!("{data:?}"));
                    }
                }
            )*
        };
    }

    repeat_for_dim_dyn!(test_dim_dyn);
    repeat_for_dim_dyn!(test_dim_dyn_mut);
    repeat_for_dim_dyn!(test_dim_dyn_size);
    repeat_for_dim_dyn!(test_dim_dyn_str);
}
