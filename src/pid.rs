//TODO: Trait for controllers? BLEHHhh

use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct PIDController {
    kp: f32,
    ki: f32,
    kd: f32,

    min_output: f32,
    max_output: f32,
    i_zone: f32,

    i_err: f32,
    last_error: f32,
}

impl PIDController {
    pub fn new(
        kp: f32,
        ki: f32,
        kd: f32,
        min_output: f32,
        max_output: f32,
        i_zone: f32,
    ) -> PIDController {
        PIDController {
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
    pub fn calculate(&mut self, dt: Duration, setpoint: f32, process_variable: f32) -> f32 {
        let error = setpoint - process_variable;

        let p_term = self.kp * error;

        let mut i_term = 0.0;

        if error.abs() < self.i_zone {
            self.i_err += error * dt.as_secs_f32();
            i_term = self.i_err * self.ki;
        } else {
            self.i_err = 0.0;
        }

        //  calculate d_err
        let d_err = error - self.last_error;
        let d_term = self.kd * d_err / dt.as_secs_f32();
        self.last_error = error;

        // calculate output
        let output = p_term + i_term + d_term;

        // clamp between min and max
        if output < self.min_output {
            return self.min_output;
        } else if output > self.max_output {
            return self.max_output;
        } else {
            return output;
        }
    }
}
