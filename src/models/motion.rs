use crate::helper_functions::cumtrapz;

use super::units::{AccelUnit, DispUnit, VelUnit};
use ndarray::Array1;
/// A struct representing time series motion data including acceleration,
/// velocity, and displacement records, along with associated metadata.
#[derive(Debug, Clone)]
pub struct Motion {
    /// Acceleration values (e.g., in g or m/s²).
    pub accelerations: Array1<f64>,
    /// Velocity values (e.g., in m/s).
    pub velocities: Array1<f64>,
    /// Displacement values (e.g., in cm or m).
    pub displacements: Array1<f64>,
    /// Time values corresponding to each sample.
    pub times: Array1<f64>,
    /// Constant time step (Δt) between each sample in seconds.
    pub time_step: f64,
    /// Unit of acceleration (e.g., "g", "m/s²").
    pub acc_unit: AccelUnit,
    /// Unit of velocity (e.g., "m/s").
    pub vel_unit: VelUnit,
    /// Unit of displacement (e.g., "cm", "m").
    pub disp_unit: DispUnit,
}

impl Motion {
    /// Processes the acceleration data to compute velocity and displacement using cumulative trapezoidal integration.
    pub fn from_acceleration(accelerations: Vec<f64>, time_step: f64, acc_unit: AccelUnit) -> Self {
        if time_step == 0.0 {
            panic!("Time step is zero");
        }
        let mut acc = Array1::from_vec(accelerations);
        // Convert acceleration to g
        let factor = acc_unit.to_g_factor();
        acc.mapv_inplace(|a| a * factor);

        // Integrate acceleration to get velocity in g·s, then convert to cm/s
        let mut vel = cumtrapz(&acc, time_step);
        vel.mapv_inplace(|v| v * 981.0); // 1 g = 981 cm/s²

        // Integrate velocity to get displacement in cm
        let disp = cumtrapz(&vel, time_step);

        // Generate time array
        let n = acc.len();
        let times = Array1::from_iter((0..n).map(|i| i as f64 * time_step));

        Motion {
            accelerations: acc,
            velocities: vel,
            displacements: disp,
            times,
            time_step,
            acc_unit,
            vel_unit: VelUnit::Cmps,
            disp_unit: DispUnit::Cm,
        }
    }
    /// Processes the velocity data to compute acceleration and displacement using finite differences and integration.
    pub fn from_velocity(velocities: Vec<f64>, time_step: f64, vel_unit: VelUnit) -> Self {
        if time_step == 0.0 {
            panic!("Time step is zero");
        }
        if velocities.is_empty() {
            panic!("No velocity data");
        }

        let mut vel = Array1::from_vec(velocities);
        // Convert velocity to cm/s
        let factor = vel_unit.to_cmps_factor();
        vel.mapv_inplace(|v| v * factor);

        let n = vel.len();
        let times = Array1::from_iter((0..n).map(|i| i as f64 * time_step));

        // Compute acceleration using finite differences
        let mut acc = Array1::zeros(n);
        for i in 1..n {
            acc[i] = (vel[i] - vel[i - 1]) / time_step;
        }
        // Use the initial velocity as an estimate for the first acceleration value
        acc[0] = vel[0] / time_step;

        // Convert acceleration to g
        acc.mapv_inplace(|a| a / 981.0);

        // Integrate velocity to get displacement in cm
        let disp = cumtrapz(&vel, time_step);

        Motion {
            accelerations: acc,
            velocities: vel,
            displacements: disp,
            times,
            time_step,
            acc_unit: AccelUnit::Cmps2, // Use a pseudo-unit for metadata tracking if needed
            vel_unit,
            disp_unit: DispUnit::Cm,
        }
    }
    /// Processes the displacement data to compute velocity and acceleration using finite differences.
    pub fn from_displacement(displacements: Vec<f64>, time_step: f64, disp_unit: DispUnit) -> Self {
        if time_step == 0.0 {
            panic!("Time step is zero");
        }
        if displacements.is_empty() {
            panic!("No displacement data");
        }

        let mut disp = Array1::from_vec(displacements);
        // Convert displacement to cm
        let factor = disp_unit.to_cm_factor();
        disp.mapv_inplace(|d| d * factor);

        let n = disp.len();
        let times = Array1::from_iter((0..n).map(|i| i as f64 * time_step));

        // Compute velocity using finite differences
        let mut vel = Array1::zeros(n);
        for i in 1..n {
            vel[i] = (disp[i] - disp[i - 1]) / time_step;
        }
        vel[0] = disp[0] / time_step; // Estimate initial velocity

        // Compute acceleration using finite differences
        let mut acc = Array1::zeros(n);
        for i in 1..n {
            acc[i] = (vel[i] - vel[i - 1]) / time_step;
        }
        acc[0] = vel[0] / time_step; // Estimate initial acceleration

        // Convert acceleration to g
        acc.mapv_inplace(|a| a / 981.0);

        Motion {
            accelerations: acc,
            velocities: vel,
            displacements: disp,
            times,
            time_step,
            acc_unit: AccelUnit::G,
            vel_unit: VelUnit::Cmps,
            disp_unit,
        }
    }
}
