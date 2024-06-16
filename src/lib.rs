#![feature(portable_simd)]
#![feature(specialization)]

pub mod backend {
    pub mod op_traits;
    pub mod traits;

    pub mod host {
        pub mod host_applicator;
        pub mod host_backend;
        pub mod host_kernels;
        pub mod host_storage;
    }
}

pub mod dimension {
    pub mod shape;
    pub mod stride;
}

pub mod array {
    pub mod array;
    pub mod array_traits;
}
