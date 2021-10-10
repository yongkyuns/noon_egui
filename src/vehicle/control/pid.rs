#![allow(non_snake_case)]
use crate::vehicle::Discrete;
use crate::S;

#[derive(Debug)]
pub struct PID {
    desired: S,
    err_cum: S,
    err_prev: S,
    P: S,
    I: S,
    D: S,
}

impl PID {
    pub fn new(P: S, I: S, D: S, desired: S) -> Self {
        Self {
            P,
            I,
            D,
            desired,
            err_cum: 0.0,
            err_prev: 0.0,
        }
    }
    pub fn control(&mut self, measured: S, dt: S) -> S {
        let P = self.P;
        let I = self.I;
        let D = self.D;
        let err = self.desired - measured;
        let err_prev = self.err_prev;

        let control_input = P * err + I * (self.err_cum) + D * (err - err_prev) / dt;

        self.err_prev = err;
        self.err_cum += err;

        control_input
    }
}

pub trait HasPID: Discrete {
    fn controller(&mut self) -> &mut PID;
    fn control(&mut self, measured: S) -> S {
        let dt = self.time_step();
        let controller = self.controller();
        controller.control(measured, dt)
        // let P = controller.P;
        // let I = controller.I;
        // let D = controller.D;
        // let err = controller.desired - measured;
        // let err_prev = controller.err_prev;

        // let control_input = P * err + I * (controller.err_cum) + D * (err - err_prev) / dt;

        // controller.err_prev = err;
        // controller.err_cum += err;

        // control_input
    }
}

pub trait SpeedControl: Discrete {
    fn controller(&mut self) -> &mut PID;
    fn control(&mut self, measured: S) -> S {
        let dt = self.time_step();
        let controller = self.controller();
        controller.control(measured, dt)
    }
}
