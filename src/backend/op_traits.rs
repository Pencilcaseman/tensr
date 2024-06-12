pub trait BinaryOp<Storage> {
    type Scalar;
    type StorageType;
    type SIMD;

    fn apply_contiguous(
        lhs: &Self::StorageType,
        rhs: &Self::StorageType,
        out: &mut Self::StorageType,
    );
}
