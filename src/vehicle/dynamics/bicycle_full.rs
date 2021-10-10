#![allow(unused)]
use nalgebra::Vector6;

use super::Dynamics;
use crate::pt;
use crate::vehicle::{HasTire, HasVehicleInfo, HasVehicleState};
use crate::S;

#[derive(Debug)]
pub struct BicycleFull {
    state: Vector6<S>,
    m: S,
    Iz: S,
    lf: S,
    lr: S,
    e_prev: S,
    e_cum: S,
    dt: S,
}

impl BicycleFull {
    pub fn new(info: &mut impl HasVehicleInfo) -> Self {
        let state = Vector6::<S>::zeros();
        // state[3] = Vx;
        let m = info.mass();
        let Iz = info.inertia_z();
        let lf = info.dist_to_front();
        let lr = info.dist_to_rear();
        let e_prev = 0.0;
        let e_cum = 0.0;
        let dt = info.time_step();
        Self {
            state,
            m,
            Iz,
            lf,
            lr,
            e_prev,
            e_cum,
            dt,
        }
    }
    pub fn step(
        &mut self,
        rwa: S,
        acc_pct: S,
        brk_pct: S,
        tire: &mut impl HasTire,
        state: &mut impl HasVehicleState,
    ) {
        let g = 9.81;

        let m = self.m;
        let Iz = self.Iz;
        let lf = self.lf;
        let lr = self.lr;
        let dt = self.dt;

        let x = self.state[0];
        let y = self.state[1];
        let psi = self.state[2];
        let v = self.state[3];
        let beta = self.state[4];
        let dpsi = self.state[5];

        // let vy = v * beta.sin();
        let vx = v * beta.cos();

        let Fzf = m * (lr / (lf + lr)) * g;
        let Fzr = m * (lf / (lf + lr)) * g;

        let max_force_acc = 100.0e3; // Maximum engine force [N]
        let max_force_brk = 300.0e3; // Maximum break force [N]
        let split_ratio = 0.5; // 0 = RWD, 1 = FWD, 0.5 = AWD
        let acc = acc_pct.min(100.0) / 100.0; // Normalized desired acceleration
        let brk = brk_pct.min(100.0) / 100.0; // Normalized desired deceleration
                                              // let Fxf = ((acc * max_force_acc) - (brk * max_force_brk)) * split_ratio; // [N]
                                              // let Fxr = ((acc * max_force_acc) - (brk * max_force_brk)) * (1.0 - split_ratio); // [N]

        let P = 3000.0;
        let I = 600.0;
        let D = 600.0;
        // let e = self.Vx - vx;
        // let e_prev = self.e_prev;
        // let e_d = e - e_prev;
        // self.e_cum += e * dt;
        // let e_cum = self.e_cum;
        // let Fxf = P * e + I * e_cum + D * e_d;
        let Fxf = 0.0;
        let Fxr = Fxf;
        // self.e_prev = e;

        // println!("Fzf = {},Fzr = {},Fxf = {},Fxr = {}", Fzf, Fzr, Fxf, Fxr);

        // println!(
        //     "v = {} m/s, beta = {} deg, dpsi = {} deg/s",
        //     v,
        //     beta * 180.0 / PI,
        //     dpsi * 180.0 / PI
        // );

        // println!(
        //     "beta = {} deg, dpsi = {} deg/s",
        //     beta * 180.0 / PI,
        //     dpsi * 180.0 / PI
        // );

        let alpha_f = (v * beta.sin() + lf * dpsi).atan2(v * beta.cos()) - rwa;
        let alpha_r = (v * beta.sin() - lr * dpsi).atan2(v * beta.cos());
        // println!(
        //     "t = {}, alpha_f = {}, alpha_r = {}",
        //     self.t,
        //     alpha_f * 180.0 / PI,
        //     alpha_r * 180.0 / PI
        // );

        let Fyf = tire.tire_force_front(alpha_f, Fzf);
        let Fyr = tire.tire_force_rear(alpha_r, Fzr);
        // let Fyf = 2.0 * self.tire_force_lat(alpha_f, self.Caf / 2.0, Fzf / 2.0, mu);
        // let Fyr = 2.0 * self.tire_force_lat(alpha_r, self.Car / 2.0, Fzr / 2.0, mu);
        // let Fyf = alpha_f * -66243.0 * 2.0;
        // let Fyr = alpha_r * -66243.0 * 2.0;
        // println!("Fyf = {}, Fyr = {}", Fyf, Fyr);
        // println!("");

        let dx = v * (beta + psi).cos();
        let dy = v * (beta + psi).sin();
        let dv = (Fxf * (beta - rwa).cos()
            + Fxr * beta.cos()
            + Fyf * (beta - rwa).sin()
            + Fyr * beta.sin())
            / m;
        let dbeta = ((-Fxf * (beta - rwa).sin()) - Fxr * (beta.sin())
            + Fyf * (beta - rwa).cos()
            + Fyr * (beta.cos())
            - m * v * dpsi)
            / (m * v);
        let ddpsi = (Fxf * lf * rwa.sin() + Fyf * lf * rwa.cos() - Fyr * lr) / Iz;

        self.state[0] = x + (dx * dt);
        self.state[1] = y + (dy * dt);
        self.state[2] = psi + (dpsi * dt);
        self.state[3] = v + (dv * dt);
        self.state[4] = beta + (dbeta * dt);
        self.state[5] = dpsi + (ddpsi * dt);

        state.move_by_mut(pt(dx * dt, dy * dt));
        state.rotate_by_mut(dpsi * dt);
        // self.step_time();
        // self.history.push(
        //     self.t,
        //     self.position.x,
        //     self.position.y,
        //     self.angle,
        //     self.state[3],                       // velocity
        //     self.state[3] * self.state[4].sin(), // lateral velocity
        //     self.state[5],                       // yaw-rate
        // );
    }
}

impl Dynamics for BicycleFull {
    fn update_from(&mut self, info: &mut impl HasVehicleInfo) {
        self.m = info.mass();
        self.Iz = info.inertia_z();
        self.lf = info.dist_to_front();
        self.lr = info.dist_to_rear();
    }
    fn step(
        &mut self,
        rwa: S,
        acc_pct: S,
        brk_pct: S,
        tire: &mut impl HasTire,
        state: &mut impl HasVehicleState,
    ) {
        self.step(rwa, acc_pct, brk_pct, tire, state);
    }
}

pub trait DynamicsFull: HasVehicleInfo {
    fn bicycle_full(&mut self, rwa: S, Fxf: S, Fxr: S);
}

impl<T> DynamicsFull for T
where
    T: HasVehicleInfo,
{
    fn bicycle_full(&mut self, rwa: S, acc_pct: S, brk_pct: S) {
        let g = 9.81;

        let m = self.mass();
        let Iz = self.inertia_z();
        let lf = self.dist_to_front();
        let lr = self.dist_to_rear();
        let dt = self.time_step();

        let x = self.position().x;
        let y = self.position().y;
        let psi = self.heading();
        let beta = self.side_slip();
        let dpsi = self.yaw_rate();

        // let vy = v * beta.sin();
        // let vx = self.vel_lon();
        // let vy = self.vel_lat();
        let v = self.velocity();

        let Fzf = m * (lr / (lf + lr)) * g;
        let Fzr = m * (lf / (lf + lr)) * g;

        // let max_force_acc = 100.0e3; // Maximum engine force [N]
        // let max_force_brk = 300.0e3; // Maximum break force [N]
        // let split_ratio = 0.5; // 0 = RWD, 1 = FWD, 0.5 = AWD
        // let acc = acc_pct.min(100.0) / 100.0; // Normalized desired acceleration
        // let brk = brk_pct.min(100.0) / 100.0; // Normalized desired deceleration
        // let Fxf = ((acc * max_force_acc) - (brk * max_force_brk)) * split_ratio; // [N]
        // let Fxr = ((acc * max_force_acc) - (brk * max_force_brk)) * (1.0 - split_ratio); // [N]

        // let P = 3000.0;
        // let I = 600.0;
        // let D = 600.0;
        // let e = self.Vx - vx;
        // let e_prev = self.e_prev;
        // let e_d = e - e_prev;
        // self.e_cum += e * dt;
        // let e_cum = self.e_cum;
        // let Fxf = P * e + I * e_cum + D * e_d;
        let Fxf = acc_pct;
        let Fxr = brk_pct;
        // self.e_prev = e;

        // println!("Fzf = {},Fzr = {},Fxf = {},Fxr = {}", Fzf, Fzr, Fxf, Fxr);

        // println!(
        //     "v = {} m/s, beta = {} deg, dpsi = {} deg/s",
        //     v,
        //     beta * 180.0 / PI,
        //     dpsi * 180.0 / PI
        // );

        // println!(
        //     "beta = {} deg, dpsi = {} deg/s",
        //     beta * 180.0 / PI,
        //     dpsi * 180.0 / PI
        // );

        let alpha_f = (v * beta.sin() + lf * dpsi).atan2(v * beta.cos()) - rwa;
        let alpha_r = (v * beta.sin() - lr * dpsi).atan2(v * beta.cos());
        // println!(
        //     "t = {}, alpha_f = {}, alpha_r = {}",
        //     self.t,
        //     alpha_f * 180.0 / PI,
        //     alpha_r * 180.0 / PI
        // );

        // let Fyf = self.tire_force_front(alpha_f, Fzf);
        // let Fyr = self.tire_force_rear(alpha_r, Fzr);
        let Fyf = -alpha_f * self.stiffness_front() * 2.0;
        let Fyr = -alpha_r * self.stiffness_rear() * 2.0;
        // println!("Fyf = {}, Fyr = {}", Fyf, Fyr);
        // println!("");

        let dx = v * (beta + psi).cos();
        let dy = v * (beta + psi).sin();
        let dv = (Fxf * (beta - rwa).cos()
            + Fxr * beta.cos()
            + Fyf * (beta - rwa).sin()
            + Fyr * beta.sin())
            / m;
        let dbeta = ((-Fxf * (beta - rwa).sin()) - Fxr * (beta.sin())
            + Fyf * (beta - rwa).cos()
            + Fyr * (beta.cos())
            - m * v * dpsi)
            / (m * v);
        let ddpsi = (Fxf * lf * rwa.sin() + Fyf * lf * rwa.cos() - Fyr * lr) / Iz;

        // self.state[0] = x + (dx * dt);
        // self.state[1] = y + (dy * dt);
        // self.state[2] = psi + (dpsi * dt);
        // self.state[3] = v + (dv * dt);
        // self.state[4] = beta + (dbeta * dt);
        // self.state[5] = dpsi + (ddpsi * dt);

        self.move_to_mut(pt(x + (dx * dt), y + (dy * dt)));
        self.set_heading(psi + (dpsi * dt));
        self.set_vel_lat((v + (dv * dt)) * (beta + (dbeta * dt)).sin());
        self.set_vel_lon((v + (dv * dt)) * (beta + (dbeta * dt)).cos());
        self.set_yaw_rate(dpsi + (ddpsi * dt));

        // self.update_pose(dt);
    }
}
