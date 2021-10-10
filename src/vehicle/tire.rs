use crate::{PI, S};
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Tire {
    pub Caf: S,
    pub Car: S,
    pub mu: S,
}

impl Tire {
    pub fn new() -> Self {
        let Caf = 66243.0;
        let Car = 66243.0;
        let mu = 1.0;
        Self { Caf, Car, mu }
    }
}

impl HasTire for Tire {
    fn tire(&mut self) -> &mut Tire {
        self
    }
}
pub trait HasTire {
    fn tire(&mut self) -> &mut Tire;
    fn stiffness_front(&mut self) -> S {
        self.tire().Caf
    }
    fn stiffness_rear(&mut self) -> S {
        self.tire().Car
    }
    fn coeff_friction(&mut self) -> S {
        self.tire().mu
    }
    fn tire_force_front(&mut self, alpha: S, Fz: S) -> S {
        let Ca = self.tire().Caf;
        let mu = self.tire().mu;
        magic_formula(alpha, Ca, Fz, mu)
    }
    fn tire_force_rear(&mut self, alpha: S, Fz: S) -> S {
        let Ca = self.tire().Car;
        let mu = self.tire().mu;
        magic_formula(alpha, Ca, Fz, mu)
    }
}

pub fn magic_formula(alpha: S, Ca: S, Fz: S, mu: S) -> S {
    let mut alpha = alpha.sin().asin();
    alpha = 180.0 / PI * alpha;

    // println!("alpha = {}", alpha);

    let a0 = 1.4; // Shape factor [-]
    let a1 = 0.0; // Load dependency of lateral friction (*1000) [1/kN]
    let a2 = 800.0; // Lateral friction level (*1000) [-]
    let a3 = 2.736 * Ca * PI / 180.0; // Maximum cornering stiffness [N/deg]
    let a4 = 7.0; // Load at maximum cornering stiffness [kN]
    let a5 = 0.0; // Camber sensitivity of cornering stiffness
    let a6 = 0.0; // Load dependency of curvature factor
    let a7 = -1.0; // Curvature factor level
    let a8 = 0.0; // Camber sensitivity of horizontal shift
    let a9 = 0.0; // Load dependency of horizontal shift
    let a10 = 0.0; // Horizontal shift level
    let a11 = 0.0; // Combined load and camber sensitivity of vertical shift
    let a12 = 0.0; // Load dependency of vertical shift
    let a13 = 0.0; // Vertical shift level

    let Fz = Fz / 1000.0; // Conversion from [N] to [kN]
    let camber = 0.0 as S; // Camber angle

    let C = a0; // Shape factor
    let mu0 = a1 * Fz + a2; // Lateral friction coefficient nominal
    let mu = mu * 1000.0; // Lateral friction coefficient operational
    let D = mu0 * Fz;
    // println!("Fz = {}, mu0 = {}, D = {}", Fz, mu0, D);
    let BCD = a3 * (2.0 * (Fz / a4).atan()).sin() * (1.0 - a5 * camber.abs()); // Cornering stiffness
    let E = a6 * Fz * a7; // Curvature factor
    let B = BCD / (C * D); // Stiffness factor
                           // println!("BCD = {}, B = {}", BCD, B);
    let Sh = a8 * camber + a9 * Fz + a10; // Horizontal shift
    let Sv = a11 * Fz * camber + a12 * Fz + a13; // Vertical shift
    let alphaEq = mu0 / mu * (alpha + Sh); // Equivalent slip angle
                                           // Reference Characteristics
                                           // println!("alphaEq = {}", alphaEq);
    let fy = D * (C * (B * alphaEq - E * (B * alphaEq - (B * alphaEq).atan())).atan()).sin();
    // Lateral force
    -mu / mu0 * (fy + Sv)
}
