use rayon::prelude::*;
use std::hint::black_box;
use tensr::backend::host::storage;

macro_rules! test_all {
    ($macro_name:ident, $($type:ty),+) => {
        $(
            paste::paste! {
                $macro_name!($type, [<$macro_name _ $type>]);
            }
        )+
    };
}

macro_rules! test_all_fundamental {
    ($macro_name:ident) => {
        test_all!($macro_name, i16, i32, i64, u16, u32, u64, f32, f64);
    };
}

macro_rules! test_alloc {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 1000;
            let s = storage::HostStorage::<$type>::new(n);
            assert_eq!(s.length, n);

            // Assert alignment is correct
            assert_eq!((s.ptr.0.as_ptr() as usize) % storage::MEM_ALIGN, 0);

            for i in 0..s.length {
                type Type = $type;
                assert_eq!(s[i], { Type::default() });
            }
        }
    };
}

macro_rules! test_alloc_uninit {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 1000;
            let mut s = unsafe { storage::HostStorage::<$type>::new_uninit(n) };
            assert_eq!(s.length, n);

            // Assert alignment is correct
            assert_eq!((s.ptr.0.as_ptr() as usize) % storage::MEM_ALIGN, 0);

            // Check we can write to this data without segfaulting
            for i in 0..s.length {
                type Type = $type;
                s[i] = Type::default();
                assert_eq!(s[i], Type::default());
            }
        }
    };
}

macro_rules! test_as_shared {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 1000;
            let s = storage::HostStorage::<$type>::new(n);
            let shared = s.as_shared();

            assert_eq!(shared.length, n);

            // Assert alignment is correct (correct, since it references the same pointer)
            assert_eq!(
                (shared.ptr.0.as_ptr() as usize) % storage::MEM_ALIGN,
                0
            );

            // Check all values are valid and correct
            for i in 0..shared.length {
                type Type = $type;
                assert_eq!(shared[i], { Type::default() });
            }
        }

        paste::paste! {
            #[test]
            fn [<$name _mut>]() {
                let n = 1000;
                let mut s = storage::HostStorage::<$type>::new(n);
                let mut shared = s.as_shared();

                assert_eq!(shared.length, n);

                // Assert alignment is correct (correct, since it references the same pointer)
                assert_eq!(
                    (shared.ptr.0.as_ptr() as usize) % storage::MEM_ALIGN,
                    0
                );

                // Set values in shared and then verify in s
                for i in 0..shared.length {
                    type Type = $type;
                    shared[i] = Type::MAX;
                }

                for i in 0..shared.length {
                    type Type = $type;
                    assert_eq!(s[i], Type::MAX);
                }
            }
        }
    };
}

macro_rules! test_take_as_vec {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 1000;
            let mut v = Vec::new();

            // Drop s to check the memory is not freed
            {
                let mut s = storage::HostStorage::<$type>::new(n);
                v = s.take_as_vec();
                drop(s);
            }

            assert_eq!(v.len(), n);

            // Check all values are valid and correct
            for i in 0..v.len() {
                type Type = $type;
                assert_eq!(v[i], { Type::default() });
            }
        }
    };
}

macro_rules! test_simd_par_iter {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n_simd = 1000;
            let n = n_simd * storage::SIMD_WIDTH;
            let s = storage::HostStorage::<$type>::new(n);

            type Type = $type;

            (0..n).into_par_iter().zip(s.simd_par_iter()).for_each(
                |(idx, packet)| {
                    for i in 0..storage::SIMD_WIDTH {
                        assert_eq!(packet[i], Type::default());
                    }
                },
            );
        }
    };
}

macro_rules! test_slice_par_iter {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n_simd = 1000;
            let n = n_simd * storage::SIMD_WIDTH;
            let s = storage::HostStorage::<$type>::new(n);

            type Type = $type;

            (0..n)
                .into_par_iter()
                .zip(s.slice_par_iter(storage::SIMD_WIDTH))
                .for_each(|(idx, slice)| {
                    for i in 0..storage::SIMD_WIDTH {
                        assert_eq!(slice[i], Type::default());
                    }
                });
        }
    };
}

macro_rules! test_slice_mut_par_iter {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n_simd = 1000;
            let n = n_simd * storage::SIMD_WIDTH;
            let mut s = unsafe { storage::HostStorage::<$type>::new_uninit(n); }

            type Type = $type;

            (0..n)
                .into_par_iter()
                .zip(s.slice_mut_par_iter(storage::SIMD_WIDTH))
                .for_each(|(idx, slice)| {
                    for i in 0..storage::SIMD_WIDTH {
                        slice[i] = Type::default();
                        assert_eq!(slice[i], Type::default());
                    }
                });
        }
    };
}

macro_rules! test_drop {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 8196;

            // Create a LOT of these and see if the system runs out of memory...
            for _ in 0..10_000 {
                let s = black_box(storage::HostStorage::<$type>::new(n));
                drop(s);
            }
        }
    };
}

test_all_fundamental!(test_alloc);
test_all_fundamental!(test_alloc_uninit);
test_all_fundamental!(test_as_shared);
test_all_fundamental!(test_take_as_vec);
test_all_fundamental!(test_simd_par_iter);
test_all_fundamental!(test_drop);
