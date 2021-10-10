// #[cfg(test)]
// mod tests {

//     use nalgebra::vector;

//     use super::*;
//     // use crate::matrix::macro_def;
//     use crate::{get_bicycle_model, StateSpace};
//     use osqp::{CscMatrix, Problem, Settings};
//     #[test]
//     fn mpc_bicycle() {
//         use std::time::Instant;
//         let now = Instant::now();

//         let Vx = 25.0; // [m/s]
//         let m = 1600.0; // [kg]
//         let Iz = 1705.0; // [kg*m^2]
//         let lf = 1.4; // [m]
//         let lr = 1.4; // [m]
//         let Caf = 66243.0; // [N/rad]
//         let Car = 66243.0; // [N/rad]
//         let dt = 0.1; // [sec]

//         let StateSpace {
//             A,
//             B,
//             C: _,
//             D: _,
//             dt: _,
//         } = get_bicycle_model(Vx, m, Iz, lf, lr, Caf, Car, dt);

//         println!(
//             "Finished discretizing plant. Elapsed = {} micros",
//             now.elapsed().as_micros()
//         );
//         let now = Instant::now();

//         let Ad = A;
//         let Bd = B;

//         const NX: usize = 4; // Number of states
//         const NU: usize = 1; // Number of inputs

//         let u0 = 0.0;
//         let umin = vector![(-30_f64 - u0).to_radians()];
//         let umax = vector![(30_f64 - u0).to_radians()];
//         let xmin = vector![-10_f64, -10., -1., -1.]; // lateral position, lateral velocity, yaw, yaw-rate
//         let xmax = vector![10_f64, 10., 1., 1.];

//         // Objective function
//         let Q = diag!(1_f64, 1., 1., 1.);
//         let QN = Q;
//         let R = eye!(1);

//         // Initial and reference states
//         let x0 = zeros!(NX);
//         let xr = vector!(1_f64, 0., 0., 0.);

//         // Prediction horizon
//         const N: usize = 10;

//         // Cast MPC problem to a QP: x = (x(0),x(1),...,x(N),u(0),...,u(N-1))
//         // - quadratic objective
//         let P = block_diag!(kron!(eye!(N), Q), QN, kron!(eye!(N), R));
//         let q = vstack!(
//             kron!(ones!(N, 1), -dot!(Q, xr)),
//             -dot!(QN, xr),
//             zeros!({ N * NU }, 1)
//         )
//         .transpose();

//         // - linear dynamics
//         let Ax = kron!(eye!(N + 1), -eye!(NX)) + kron!(eye!({ N + 1 }, -1), Ad);
//         let Bu = kron!(vstack!(zeros!(1, N), eye!(N)), Bd);

//         let Aeq = hstack!(Ax, Bu);
//         let leq = hstack!(-x0, zeros!(1, { N * NX }));
//         let ueq = leq;

//         // - input and state constraints
//         let Aineq = eye!((N + 1) * NX + N * NU);
//         let lineq = vstack!(kron!(ones!({ N + 1 }, 1), xmin), kron!(ones!(N, 1), umin)).transpose();
//         let uineq = vstack!(kron!(ones!({ N + 1 }, 1), xmax), kron!(ones!(N, 1), umax)).transpose();

//         // - OSQP constraints
//         let A = vstack!(Aeq, Aineq);
//         let l = hstack!(leq, lineq);
//         let u = hstack!(ueq, uineq);

//         let P = &P.transpose().data.0;
//         let A = &A.transpose().data.0;
//         let q = &q.transpose().data.0[0];
//         let l = &l.transpose().data.0[0];
//         let u = &u.transpose().data.0[0];

//         println!(
//             "Finished computing matrices. Elapsed = {} micros",
//             now.elapsed().as_micros()
//         );
//         let now = Instant::now();

//         // Extract the upper triangular elements of `P`
//         let P = CscMatrix::from(P).into_upper_tri();

//         // Disable verbose output
//         let settings = Settings::default();

//         // Create an OSQP problem
//         let mut prob = Problem::new(P, q, A, l, u, &settings).expect("failed to setup problem");

//         println!(
//             "Finished setting up problem. Elapsed = {} micros",
//             now.elapsed().as_micros()
//         );
//         let now = Instant::now();

//         // Solve problem
//         let result = prob.solve();

//         println!(
//             "Finished solving problem. Elapsed = {} micros",
//             now.elapsed().as_micros()
//         );

//         // Print the solution
//         println!("{:?}", result.x().expect("failed to solve problem"));
//     }
// }
