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

macro_rules! test_alloc_uninitialized {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            let n = 1000;
            let mut s = storage::HostStorage::<$type>::new_uninitialized(n);
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

test_all_fundamental!(test_alloc);
test_all_fundamental!(test_alloc_uninitialized);
test_all_fundamental!(test_as_shared);
