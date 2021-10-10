use super::{Dynamics, StateSpace};
use crate::vehicle::{HasTire, HasVehicleInfo, HasVehicleState, PoseUpdate};

use crate::S;
use nalgebra::{matrix, vector, Matrix4, Matrix5, Vector4};

#[derive(Debug)]
pub struct BicycleSimple {
    model: StateSpace,
    x0: Vector4<S>,
}

impl BicycleSimple {
    pub fn new(info: &mut impl HasVehicleInfo) -> Self {
        let m = info.mass();
        let Iz = info.inertia_z();
        let lf = info.dist_to_front();
        let lr = info.dist_to_rear();
        let Vx = info.vel_lon();
        let Caf = info.stiffness_front();
        let Car = info.stiffness_rear();
        let dt = info.time_step();
        let model = get_bicycle_model(Vx, m, Iz, lf, lr, Caf, Car, dt);
        let x0 = Vector4::<S>::zeros();
        Self { model, x0 }
    }

    pub fn step(&mut self, rwa: S) {
        self.x0 = self.model.step(self.x0, rwa);
    }
}

impl Dynamics for BicycleSimple {
    fn update_from(&mut self, info: &mut impl HasVehicleInfo) {
        let m = info.mass();
        let Iz = info.inertia_z();
        let lf = info.dist_to_front();
        let lr = info.dist_to_rear();
        let Vx = info.vel_lon();
        let Caf = info.stiffness_front();
        let Car = info.stiffness_rear();
        let dt = info.time_step();
        self.model = get_bicycle_model(Vx, m, Iz, lf, lr, Caf, Car, dt);
        self.x0[1] = info.vel_lat();
        self.x0[3] = info.yaw_rate();
    }
    fn step(
        &mut self,
        rwa: S,
        _acc_pct: S,
        _brk_pct: S,
        _tire: &mut impl HasTire,
        _x0: &mut impl HasVehicleState,
    ) {
        self.step(rwa);
    }
}

pub fn get_bicycle_model(Vx: S, m: S, Iz: S, lf: S, lr: S, Caf: S, Car: S, dt: S) -> StateSpace {
    let mut A = matrix![0.0,             1.0,                   Vx,                0.0;
                        0.0, -(2.0*Caf+2.0*Car)/(m*Vx),        0.0, -Vx-(2.0*Caf*lf-2.0*Car*lr)/(m*Vx);
                        0.0,             0.0,                  0.0,                1.0;
                        0.0, -(2.0*lf*Caf-2.0*lr*Car)/(Iz*Vx), 0.0, -(2.0*lf*lf*Caf+2.0*lr*lr*Car)/(Iz*Vx)
    ];

    let mut B = vector![0.0, 2.0 * Caf / m, 0.0, 2.0 * lf * Caf / Iz];

    let C = matrix![1.0, 0.0, 0.0, 0.0;
                    0.0, 0.0, 1.0, 0.0];

    let D = 0.0;

    c2d(&mut A, &mut B, dt);

    StateSpace { A, B, C, D, dt }
}

fn c2d(A: &mut Matrix4<S>, B: &mut Vector4<S>, dt: S) {
    // Discretize with zoh
    let mut upper = A.insert_fixed_columns::<1>(A.shape().1, 0.);
    upper.set_column(A.shape().1, &B);

    let mut em = Matrix5::<S>::zeros();
    for i in 0..upper.shape().0 {
        em.set_row(i, &upper.row(i));
    }

    let ms = (em * dt).exp();

    let Ad = ms.slice((0, 0), (A.nrows(), A.ncols()));
    let Bd = ms.slice((0, A.nrows()), (B.nrows(), B.ncols()));

    for i in 0..A.nrows() {
        A.set_row(i, &Ad.row(i));
        B.set_row(i, &Bd.row(i));
    }
}

pub trait DynamicsSimple: HasVehicleInfo {
    fn bicycle_simple(&mut self, rwa: S);
}

impl<T> DynamicsSimple for T
where
    T: HasVehicleInfo,
{
    fn bicycle_simple(&mut self, rwa: S) {
        let m = self.mass();
        let Iz = self.inertia_z();
        let lf = self.dist_to_front();
        let lr = self.dist_to_rear();
        let Vx = self.vel_lon();
        let Caf = self.stiffness_front();
        let Car = self.stiffness_rear();
        let dt = self.time_step();

        let model = get_bicycle_model(Vx, m, Iz, lf, lr, Caf, Car, dt);
        let mut x0 = Vector4::<S>::zeros();
        x0[1] = self.vel_lat();
        x0[3] = self.yaw_rate();

        let x_new = model.step(x0, rwa);

        self.set_vel_lat(x_new[1]);
        self.set_yaw_rate(x_new[3]);
        self.update_pose(dt);
    }
}
