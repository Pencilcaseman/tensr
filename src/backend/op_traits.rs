pub trait BinaryOp {}

pub trait ScalarKernel<T> {
    fn apply_scalar(lhs: T, rhs: T) -> T;
}
