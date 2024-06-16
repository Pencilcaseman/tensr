pub trait Applicator2<Op, Lhs, Rhs, Out, T> {
    fn apply_contiguous(lhs: &Lhs, rhs: &Rhs, out: &mut Out);
}
