pub trait Integrable {
    type Output;

    fn integrate_from(&self, left_point: &Self) -> Self::Output;
}
