use bevy::{
    ecs::resource::Resource,
    log::{info, warn},
    math::USizeVec2,
};
use core::f32;
use std::f32::consts::FRAC_1_SQRT_2;

#[derive(Resource)]
pub struct ScalarWaveField {
    u: [Vec<f32>; 2],
    resolution: USizeVec2,
    space_step: f32,
    wave_speed: f32,
    damping: f32,
}
impl ScalarWaveField {
    pub fn new(resolution: USizeVec2, space_step: f32, wave_speed: f32, damping: f32) -> Self {
        let num_cells = resolution.x * resolution.y;
        let me = Self {
            u: [vec![0.0; num_cells], vec![0.0; num_cells]],
            resolution,
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

    pub fn cell_real_space(&self, x: f32, y: f32, min: f32, max: f32) -> f32 {
        let x = self
            .linearly_interpolate(x, 0.0, self.resolution.x as f32, min, max)
            .round() as usize;
        let y = self
            .linearly_interpolate(y, 0.0, self.resolution.y as f32, min, max)
            .round() as usize;

        self.cell_grid_space(0, x, y)
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

    pub fn cell_grid_space(&self, t: usize, x: usize, y: usize) -> f32 {
        return self.u[t][x + (y * self.resolution.y)];
    }

    fn calc_courant(&self, dt: f32) -> f32 {
        self.wave_speed * dt / self.space_step
    }

    pub fn step(&mut self, dt: f32) -> Vec<f32> {
        let r = self.calc_courant(dt);
        if r > FRAC_1_SQRT_2 {
            warn!(
                "Courant number has a value of {r}, must be below {FRAC_1_SQRT_2} to prevent numeric explosion. This step has been skipped"
            );
        }
        let mut new_u = self.u[0].clone();

        for j in 1..self.resolution.y - 1 {
            for i in 1..self.resolution.x - 1 {
                let discrete_time =
                    2.0 * self.cell_grid_space(0, i, j) - self.cell_grid_space(1, i, j);
                let discrete_space = self.cell_grid_space(0, i + 1, j)
                    + self.cell_grid_space(0, i - 1, j)
                    + self.cell_grid_space(0, i, j + 1)
                    + self.cell_grid_space(0, i, j - 1)
                    - (4.0 * self.cell_grid_space(0, i, j));

                new_u[i + (j * self.resolution.x)] =
                    (discrete_time + (r * r * (discrete_space))) * self.damping;
            }
        }

        // Set boundary to 0.0
        for j in [0, self.resolution.y - 1] {
            for i in 0..self.resolution.x {
                new_u[j + (i * self.resolution.x)] = 0.0;
                new_u[i + (j * self.resolution.x)] = 0.0;
            }
        }

        self.u[1] = self.u[0].clone();
        self.u[0] = new_u.clone();
        new_u
    }
}
