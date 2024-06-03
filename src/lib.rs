#![feature(portable_simd)]

pub mod backend {
    pub mod traits;

    pub mod host {
        pub mod storage;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
