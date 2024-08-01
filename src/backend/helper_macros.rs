#[macro_export]
macro_rules! repeat_binary_ops {
    ($macro: tt) => {
        $macro!(Add, add, +);
    };
}

#[macro_export]
macro_rules! repeat_array_function_types {
    ($macro: tt) => {
        $macro!('static, ArrayBase, ArrayBase);
    };
}
