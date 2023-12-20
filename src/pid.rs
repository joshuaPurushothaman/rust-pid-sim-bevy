use std::time::Duration;

use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,

    min_output: f64,
    max_output: f64,
    i_zone: f64,

    i_err: f64,
    last_error: f64,
}

impl PIDController {
    pub fn new(
        kp: f64,
        ki: f64,
        kd: f64,
        min_output: f64,
        max_output: f64,
        i_zone: f64,
    ) -> PIDController {
        Self {
            kp,
            ki,
            kd,
            min_output,
            max_output,
            i_zone,
            i_err: 0.0,
            last_error: 0.0,
        }
    }

    pub fn calculate(&mut self, dt: Duration, setpoint: f64, process_variable: f64) -> f64 {
        let error = setpoint - process_variable;

        let p_term = self.kp * error;

        let i_term = if error.abs() < self.i_zone {
            self.i_err += error * dt.as_secs_f64();
            self.i_err * self.ki
        } else {
            self.i_err = 0.0;
            0.0
        };

        let d_err = error - self.last_error;
        let mut d_term = self.kd * d_err / dt.as_secs_f64();
        if d_term.is_nan() {
            d_term = 0.;
        }
        self.last_error = error;

        let output = p_term + i_term + d_term;

        output.clamp(self.min_output, self.max_output)
    }
}
