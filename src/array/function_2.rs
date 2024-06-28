use crate::backend::op_traits;
use std::marker::PhantomData;

pub struct Function2RefRef<'a, Applicator, Op, Lhs, Rhs> {
    lhs: &'a Lhs,
    rhs: &'a Rhs,
    applicator_phantom: PhantomData<Applicator>,
    op_phantom: PhantomData<Op>,
}

impl<'a, Applicator, Op, Lhs, Rhs>
    Function2RefRef<'a, Applicator, Op, Lhs, Rhs>
{
    pub fn new(lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self {
            lhs,
            rhs,
            applicator_phantom: PhantomData,
            op_phantom: PhantomData,
        }
    }

    pub fn apply<Out>(&self, out: &mut Out)
    where
        Applicator: op_traits::Applicator2<Op, Lhs, Rhs, Out>,
    {
        Applicator::apply_contiguous(self.lhs, self.rhs, out);
    }
}
