#[macro_export]
macro_rules! repeat_binary_ops {
    ($macro: tt) => {
        $macro!(Add, add, +);
        $macro!(Sub, sub, -);
    };
}

// #[macro_export]
// macro_rules! array_binary_ops {
//     () => {
//         [Add, add, +]
//     }
// }
//
// #[macro_export]
// macro_rules! binop_permutations {
//     () => {
//         [(Own, ArrayBase), (Own, ArrayBase)],
//         [(Own, ArrayBase), (Ref, ArrayBase)],
//         [(Ref, ArrayBase), (Own, ArrayBase)],
//         [(Ref, ArrayBase), (Ref, ArrayBase)],
//
//         [(Own, TensrFn2), (Own, ArrayBase)],
//         [(Own, TensrFn2), (Ref, ArrayBase)],
//         [(Ref, TensrFn2), (Own, ArrayBase)],
//         [(Ref, TensrFn2), (Ref, ArrayBase)],
//
//         [(Own, ArrayBase), (Own, TensrFn2)],
//         [(Own, ArrayBase), (Ref, TensrFn2)],
//         [(Ref, ArrayBase), (Own, TensrFn2)],
//         [(Ref, ArrayBase), (Ref, TensrFn2)],
//
//         [(Own, TensrFn2), (Own, TensrFn2)],
//         [(Own, TensrFn2), (Ref, TensrFn2)],
//         [(Ref, TensrFn2), (Own, TensrFn2)],
//         [(Ref, TensrFn2), (Ref, TensrFn2)],
//     };
// }
