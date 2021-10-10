use crate::S;
pub type DeltaT = S;

impl Discrete for DeltaT {
    fn delta_t(&mut self) -> &mut DeltaT {
        self
    }
}
pub trait Discrete {
    fn delta_t(&mut self) -> &mut DeltaT;
    fn time_step(&mut self) -> S {
        *self.delta_t()
    }
    fn set_time_step(&mut self, dt: S) {
        *self.delta_t() = dt;
    }
}
