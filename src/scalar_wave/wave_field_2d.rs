use core::f32;
use std::f32::consts::FRAC_1_SQRT_2;

use bevy::{
    ecs::component::Component,
    log::{info, warn},
    math::ops::sqrt,
};

#[derive(Component)]
pub struct WaveField2D {
    u: [Vec<f32>; 2],
    x_len: usize,
    y_len: usize,
    space_step: f32,
    wave_speed: f32,
    damping: f32,
}
impl WaveField2D {
    pub fn new(
        u: Vec<f32>,
        x_len: usize,
        y_len: usize,
        space_step: f32,
        wave_speed: f32,
        damping: f32,
    ) -> Self {
        let me = Self {
            u: [u.clone(), u],
            x_len,
            y_len,
            space_step,
            wave_speed,
            damping,
        };

        let r = me.calc_courant(1.0 / 60.0);
        if r > FRAC_1_SQRT_2 {
            panic!("Courant number has a value of {r} at 60 fps, must be below {FRAC_1_SQRT_2}");
        }

        info!("Courant number has a value of {r} at 60 fps");
        me
    }
    pub fn set_u(&mut self, u: Vec<f32>) {
        self.u = [u.clone(), u];
    }

    pub fn get_xy(&self, x: f32, y: f32, min: f32, max: f32) -> f32 {
        let x = self
            .linearly_interpolate(x, 0.0, self.x_len as f32, min, max)
            .round() as usize;
        let y = self
            .linearly_interpolate(y, 0.0, self.x_len as f32, min, max)
            .round() as usize;

        self.get(0, x, y)
    }

    fn linearly_interpolate(
        &self,
        val: f32,
        in_min: f32,
        in_max: f32,
        out_min: f32,
        out_max: f32,
    ) -> f32 {
        out_min + ((val - in_min) / (in_max - in_min)) * (out_max - out_min)
    }

    pub fn get(&self, t: usize, x: usize, y: usize) -> f32 {
        return self.u[t][x + (y * self.x_len)];
    }

    fn calc_courant(&self, dt: f32) -> f32 {
        self.wave_speed * dt / self.space_step
    }

    pub fn step(&mut self, dt: f32) {
        let r = self.calc_courant(dt);
        if r > FRAC_1_SQRT_2 {
            warn!(
                "Courant number has a value of {r}, must be below {FRAC_1_SQRT_2} to prevent numeric explosion. This step has been skipped"
            );
        }
        let mut new_u = self.u[0].clone();

        for j in 1..self.y_len - 1 {
            for i in 1..self.x_len - 1 {
                let discrete_time = 2.0 * self.get(0, i, j) - self.get(1, i, j);
                let discrete_space = self.get(0, i + 1, j)
                    + self.get(0, i - 1, j)
                    + self.get(0, i, j + 1)
                    + self.get(0, i, j - 1)
                    - (4.0 * self.get(0, i, j));

                new_u[i + (j * self.x_len)] =
                    (discrete_time + (r * r * (discrete_space))) * self.damping;
            }
        }

        // Set boundary to 0.0
        for j in [0, self.y_len - 1] {
            for i in 0..self.x_len {
                new_u[j + (i * self.x_len)] = 0.0;
                new_u[i + (j * self.x_len)] = 0.0;
            }
        }

        self.u[1] = self.u[0].clone();
        self.u[0] = new_u;
    }
}
