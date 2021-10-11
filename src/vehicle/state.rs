// use crate::geom::{point, Angle, HasOrientation, HasPosition, Point};
use crate::S;
use crate::{pt, Spatial, Visual, WithSpatial, WithVisual};

#[derive(Debug, Clone, Copy)]
pub struct VehicleState {
    Vx: S,
    Vy: S,
    Fxf: S,
    Fxr: S,
    RWA: S,
    dPSI: S,
    spatial: Spatial,
    // visual: Visual,
}

impl VehicleState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for VehicleState {
    fn default() -> Self {
        Self {
            Vx: 30.0,
            Vy: 0.0,
            RWA: 0.0,
            Fxf: 0.0,
            Fxr: 0.0,
            dPSI: 0.0,
            spatial: Spatial::default(),
            // visual: Visual::default(),
        }
    }
}

impl WithSpatial for VehicleState {
    fn get(&self) -> &Spatial {
        &self.spatial
    }
    fn get_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }
}

// impl WithVisual for VehicleState {
//     fn get(&self) -> &Visual {
//         &self.visual
//     }
//     fn get_mut(&mut self) -> &mut Visual {
//         &mut self.visual
//     }
// }

pub trait PoseUpdate {
    fn update_pose(&mut self, dt: S);
}

impl<T> PoseUpdate for T
where
    T: WithSpatial + HasVehicleState,
{
    fn update_pose(&mut self, dt: S) {
        let Vx = self.vel_lon();
        let Vy = self.vel_lat();
        let dpsi = self.yaw_rate();
        let psi = self.heading();

        let dx = (Vx * psi.cos() - Vy * psi.sin()) * dt;
        let dy = (Vx * psi.sin() + Vy * psi.cos()) * dt;

        self.rotate_by_mut(dpsi * dt);
        self.move_by_mut(pt(dx, dy));
    }
}

pub trait HasVehicleState: WithSpatial {
    fn state(&mut self) -> &mut VehicleState;
    fn vel_lon(&mut self) -> S {
        self.state().Vx
    }
    fn set_vel_lon(&mut self, lon_vel: S) {
        self.state().Vx = lon_vel;
    }
    fn vel_lat(&mut self) -> S {
        self.state().Vy
    }
    fn set_vel_lat(&mut self, lat_vel: S) {
        self.state().Vy = lat_vel;
    }
    fn force_lon_front(&mut self) -> S {
        self.state().Fxf
    }
    fn force_lon_rear(&mut self) -> S {
        self.state().Fxr
    }
    fn road_wheel_angle_rad(&mut self) -> S {
        self.state().RWA
    }
    fn yaw_rate(&mut self) -> S {
        self.state().dPSI
    }
    fn set_yaw_rate(&mut self, yaw_rate: S) {
        self.state().dPSI = yaw_rate;
    }
    fn heading(&mut self) -> S {
        self.state().angle()
    }
    fn set_heading(&mut self, angle: S) {
        self.state().set_angle(angle);
    }
    fn side_slip(&mut self) -> S {
        (self.vel_lat() / self.vel_lon()).atan()
    }
    fn velocity(&mut self) -> S {
        (self.vel_lat().powi(2) + self.vel_lon().powi(2)).sqrt()
    }
}

// impl<T> HasPosition for T where T: HasVehicleState {}

impl HasVehicleState for VehicleState {
    fn state(&mut self) -> &mut VehicleState {
        self
    }
}
