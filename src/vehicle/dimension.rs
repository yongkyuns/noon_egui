use crate::S;

#[derive(Debug)]
pub struct Dimension {
    pub m: S,
    pub Iz: S,
    pub lf: S,
    pub lr: S,
}

impl Dimension {
    pub fn new() -> Self {
        let m = 1600.0;
        let Iz = 1705.0;
        let lf = 1.4;
        let lr = 1.4;
        Self { m, Iz, lf, lr }
    }
}
pub trait HasDimension: Sized {
    fn dimension(&mut self) -> &mut Dimension;
    fn mass(&mut self) -> S {
        self.dimension().m
    }
    fn inertia_z(&mut self) -> S {
        self.dimension().Iz
    }
    fn dist_to_front(&mut self) -> S {
        self.dimension().lf
    }
    fn dist_to_rear(&mut self) -> S {
        self.dimension().lr
    }
}

impl HasDimension for Dimension {
    fn dimension(&mut self) -> &mut Dimension {
        self
    }
}
