pub trait BinaryOp {}

pub trait Applicator2<Op, Lhs, Rhs, Out> {
    fn apply_contiguous(lhs: &Lhs, rhs: &Rhs, out: &mut Out);
}
