use core::f32;
use std::f32::consts::FRAC_1_SQRT_2;

use bevy::{
    ecs::component::Component,
    log::{info, warn},
    math::ops::sqrt,
};

#[derive(Component)]
pub struct WaveField2D<const N: usize> {
    u: [[f32; N]; 2],
    space_step: f32,
    wave_speed: f32,
    damping: f32,
    side_len: usize,
}
impl<const N: usize> WaveField2D<N> {
    pub fn new(u: [f32; N], space_step: f32, wave_speed: f32, damping: f32) -> Self {
        let me = Self {
            u: [u, u],
            space_step,
            wave_speed,
            damping,
            side_len: sqrt(N as f32) as usize,
        };

        let r = me.calc_courant(1.0 / 60.0);
        if r > FRAC_1_SQRT_2 {
            panic!("Courant number has a value of {r} at 60 fps, must be below {FRAC_1_SQRT_2}");
        }

        info!("Courant number has a value of {r} at 60 fps");
        me
    }
    pub fn set_u(&mut self, u: [f32; N]) {
        self.u = [u, u];
    }

    pub fn get_xy(&self, x: f32, y: f32, min: f32, max: f32) -> f32 {
        let x = self.linearly_interpolate(x, min, max).round() as usize;
        let y = self.linearly_interpolate(y, min, max).round() as usize;

        self.get(0, x, y)
    }

    fn linearly_interpolate(&self, val: f32, out_min: f32, out_max: f32) -> f32 {
        ((val - out_min) * self.side_len as f32) / (out_max - out_min)
    }

    pub fn get(&self, t: usize, x: usize, y: usize) -> f32 {
        return self.u[t][x + (y * self.side_len)];
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
        let mut new_u = self.u[0];

        for i in 1..self.side_len - 1 {
            for j in 1..self.side_len - 1 {
                let discrete_time = 2.0 * self.get(0, i, j) - self.get(1, i, j);
                let discrete_space = self.get(0, i + 1, j)
                    + self.get(0, i - 1, j)
                    + self.get(0, i, j + 1)
                    + self.get(0, i, j - 1)
                    - (4.0 * self.get(0, i, j));

                new_u[i + (j * self.side_len)] =
                    (discrete_time + (r * r * (discrete_space))) * self.damping;
            }
        }

        // Set boundary to 0.0
        for edge in [0, self.side_len - 1] {
            for var in 0..self.side_len {
                new_u[edge + (var * self.side_len)] = 0.0;
                new_u[var + (edge * self.side_len)] = 0.0;
            }
        }

        self.u[1] = self.u[0];
        self.u[0] = new_u;
    }
}
