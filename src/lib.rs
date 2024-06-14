#![feature(portable_simd)]

pub mod backend {
    pub mod op_traits;
    pub mod traits;

    pub mod host {
        pub mod host_backend;
        pub mod host_function;
        pub mod host_kernels;
        pub mod host_storage;
    }
}
