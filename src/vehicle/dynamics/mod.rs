#![allow(non_snake_case, dead_code)]
pub mod bicycle_full;
pub mod bicycle_simple;
pub mod discrete;

pub use bicycle_full::{BicycleFull, DynamicsFull};
pub use bicycle_simple::{BicycleSimple, DynamicsSimple};
pub use discrete::{DeltaT, Discrete};

use crate::vehicle::{HasTire, HasVehicleInfo, HasVehicleState};
use crate::S;
use nalgebra::{Matrix2x4, Matrix4, Vector4};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct StateSpace {
    pub A: Matrix4<S>,
    pub B: Vector4<S>,
    pub C: Matrix2x4<S>,
    pub D: S,
    pub dt: S,
}

impl StateSpace {
    pub fn new(A: Matrix4<S>, B: Vector4<S>, C: Matrix2x4<S>, D: S, dt: S) -> Self {
        Self { A, B, C, D, dt }
    }
    pub fn step(&self, x0: Vector4<S>, u: S) -> Vector4<S> {
        let A = self.A;
        let B = self.B;
        A * x0 + B * u
    }
}

#[derive(Debug)]
pub enum Model {
    Simple(BicycleSimple),
    Full(BicycleFull),
}

impl Model {
    pub fn new(info: &mut impl HasVehicleInfo) -> Self {
        Model::Full(BicycleFull::new(info))
    }
}

pub trait Dynamics {
    fn update_from(&mut self, info: &mut impl HasVehicleInfo);
    fn step(
        &mut self,
        rwa: S,
        acc_pct: S,
        brk_pct: S,
        tire: &mut impl HasTire,
        state: &mut impl HasVehicleState,
    );
}

// impl Default for Model {
//     fn default() -> Self {
//         Dynamics::Full(BicycleFull::new())
//     }
// }

impl Dynamics for Model {
    fn update_from(&mut self, _info: &mut impl HasVehicleInfo) {}
    fn step(
        &mut self,
        rwa: S,
        acc_pct: S,
        brk_pct: S,
        tire: &mut impl HasTire,
        state: &mut impl HasVehicleState,
    ) {
        match self {
            Model::Simple(model) => {
                // model.step(rwa, acc_pct, brk_pct);
                Dynamics::step(model, rwa, acc_pct, brk_pct, tire, state);
            }
            Model::Full(model) => {
                // model.step(rwa, acc_pct, brk_pct);
                Dynamics::step(model, rwa, acc_pct, brk_pct, tire, state);
            }
        }
    }
}

impl From<BicycleFull> for Model {
    fn from(bicycle: BicycleFull) -> Model {
        Model::Full(bicycle)
    }
}

impl From<BicycleSimple> for Model {
    fn from(bicycle: BicycleSimple) -> Model {
        Model::Simple(bicycle)
    }
}

// impl Dynamics for Option<Model> {
//     fn update_from(&mut self, info: &mut impl HasVehicleInfo) {
//         if let Some(model) = self {
//             model.update_from(info);
//         } else {
//             let mut model = Model::Full(BicycleFull::new(info));
//             model.update_from(info);
//             *self = Some(model);
//         }
//     }
//     fn step(
//         &mut self,
//         rwa: S,
//         acc_pct: S,
//         brk_pct: S,
//         tire: &mut impl HasTire,
//         state: &mut impl HasVehicleState,
//     ) {
//         if let Some(model) = self {
//             model.step(rwa, acc_pct, brk_pct, tire, state);
//         }
//     }
// }
