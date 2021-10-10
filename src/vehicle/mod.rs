#![allow(non_snake_case, dead_code)]
pub mod control;
pub mod dimension;
pub mod dynamics;
pub mod info;
pub mod state;
pub mod tire;

// use std::S::consts::PI;

pub use self::control::pid::{SpeedControl, PID};
pub use self::dimension::{Dimension, HasDimension};
pub use self::dynamics::{
    BicycleFull, DeltaT, Discrete, Dynamics, DynamicsFull, DynamicsSimple, Model,
};
pub use self::info::{HasVehicleInfo, VehicleInfo};
pub use self::state::{HasVehicleState, PoseUpdate, VehicleState};
pub use self::tire::{HasTire, Tire};

use crate::geom::{Angle, Point};
// use crate::logging::Log;
use crate::{Spatial, WithSpatial, S};

pub fn vehicle() -> Vehicle {
    let vehicle = Vehicle::new();
    vehicle
}

#[derive(Debug)]
pub struct Vehicle {
    vehicle_info: VehicleInfo,
    dynamics: Model,
    speed_controller: PID,
    pub t: S,
    // pub history: Log,
}

impl Vehicle {
    pub fn new() -> Self {
        let mut vehicle_info = VehicleInfo::new();
        let dynamics = BicycleFull::new(&mut vehicle_info).into();
        let t = 0.0;
        // let history = Log::new();
        let P = 100000.0;
        let I = 00.0;
        let D = 00.0;
        let speed_controller = PID::new(P, I, D, vehicle_info.velocity());
        Self {
            vehicle_info,
            speed_controller,
            dynamics,
            t,
            // history,
        }
    }

    fn step_time(&mut self) {
        let dt = self.time_step();
        self.t += dt;
    }

    fn update_history(&mut self) {
        let t = self.t;
        let x = self.position().x;
        let y = self.position().y;
        let heading = self.angle();
        let vel = self.velocity();
        let lat_vel = self.vel_lat();
        let yaw_rate = self.yaw_rate();
        // self.history.push(t, x, y, heading, vel, lat_vel, yaw_rate);
    }

    pub fn step_simple(&mut self, rwa: S) {
        self.bicycle_simple(rwa);
        self.step_time();
        self.update_history();
    }

    pub fn step_full(&mut self, rwa: S) {
        let vel = self.velocity();
        let Fx = SpeedControl::control(self, vel);
        self.bicycle_full(rwa, Fx / 2.0, Fx / 2.0);
        self.step_time();
        self.update_history();
    }
}

impl WithSpatial for Vehicle {
    fn get(&self) -> &Spatial {
        WithSpatial::get(&self.vehicle_info)
    }
    fn get_mut(&mut self) -> &mut Spatial {
        WithSpatial::get_mut(&mut self.vehicle_info)
    }
}

impl HasDimension for Vehicle {
    fn dimension(&mut self) -> &mut Dimension {
        HasDimension::dimension(&mut self.vehicle_info)
    }
}

impl HasVehicleState for Vehicle {
    fn state(&mut self) -> &mut VehicleState {
        HasVehicleState::state(&mut self.vehicle_info)
    }
}

impl HasTire for Vehicle {
    fn tire(&mut self) -> &mut Tire {
        HasTire::tire(&mut self.vehicle_info)
    }
}

impl Discrete for Vehicle {
    fn delta_t(&mut self) -> &mut DeltaT {
        Discrete::delta_t(&mut self.vehicle_info)
    }
}

impl SpeedControl for Vehicle {
    fn controller(&mut self) -> &mut PID {
        &mut self.speed_controller
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PI;
    use std::time::Instant;
    #[test]
    fn test_case() {
        let mut car = vehicle();
        let now = Instant::now();
        for _ in 0..10000 {
            car.step_full(5. * PI / 180.);
        }
        println!("Time = {} ms", now.elapsed().as_millis());
    }
    #[test]
    fn plot_tire_model() {
        // use crate::plot::plot;
        let car = vehicle();
        let mut slip = Vec::<S>::new();
        let mut fy = Vec::<S>::new();
        for i in -1000..1000 {
            let i = i as S * 0.001;
            // let y = car.tire_force_lat(i, 66243.0, 100.0, 1.0);
            // slip.push(i * 180.0 / PI);
            // fy.push(y);
        }
        // plot(&slip, &[&fy]);
    }
}
