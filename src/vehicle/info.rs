use super::dimension::{Dimension, HasDimension};
use super::dynamics::{DeltaT, Discrete};
use super::state::{HasVehicleState, VehicleState};
use super::tire::{HasTire, Tire};

use crate::geom::{Angle, Point};
use crate::{Spatial, WithSpatial};

#[derive(Debug)]
pub struct VehicleInfo {
    pub dimension: Dimension,
    pub state: VehicleState,
    pub tire: Tire,
    pub dt: DeltaT,
}

impl VehicleInfo {
    pub fn new() -> Self {
        let dimension = Dimension::new();
        let state = VehicleState::new();
        let tire = Tire::new();
        let dt = 0.001;
        Self {
            dimension,
            state,
            tire,
            dt,
        }
    }
}

impl WithSpatial for VehicleInfo {
    fn get(&self) -> &Spatial {
        WithSpatial::get(&self.state)
    }
    fn get_mut(&mut self) -> &mut Spatial {
        WithSpatial::get_mut(&mut self.state)
    }
}

impl HasDimension for VehicleInfo {
    fn dimension(&mut self) -> &mut Dimension {
        HasDimension::dimension(&mut self.dimension)
    }
}

impl HasVehicleState for VehicleInfo {
    fn state(&mut self) -> &mut VehicleState {
        HasVehicleState::state(&mut self.state)
    }
}

impl HasTire for VehicleInfo {
    fn tire(&mut self) -> &mut Tire {
        HasTire::tire(&mut self.tire)
    }
}

impl Discrete for VehicleInfo {
    fn delta_t(&mut self) -> &mut DeltaT {
        Discrete::delta_t(&mut self.dt)
    }
}

pub trait HasVehicleInfo: HasDimension + HasVehicleState + HasTire + Discrete {}
impl<T> HasVehicleInfo for T where T: HasDimension + HasVehicleState + HasTire + Discrete {}
