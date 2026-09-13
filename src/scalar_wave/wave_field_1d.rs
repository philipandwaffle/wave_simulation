use bevy::{
    ecs::component::Component,
    log::{info, warn},
    math::ops::sqrt,
};

#[derive(Component)]
pub struct WaveField1D<const N: usize> {
    u: [[f32; N]; 2],
    space_step: f32,
    wave_speed: f32,
    damping: f32,
}
impl<const N: usize> WaveField1D<N> {
    pub fn new(u: [f32; N], space_step: f32, wave_speed: f32, damping: f32) -> Self {
        let me = Self {
            u: [u, u],
            space_step,
            wave_speed,
            damping,
        };

        let r = me.calc_courant(1.0 / 60.0);
        if r > 1.0 {
            panic!("Courant number has a value of {r} at 60 fps");
        }

        info!("Courant number has a value of {r} at 60 fps");
        me
    }

    pub fn get_x(&self, x: f32, min: f32, max: f32) -> f32 {
        let x = ((x - min) * N as f32) / (max - min);
        return self.u[0][x.round() as usize];
    }

    pub fn get(&self, x: usize) -> f32 {
        return self.u[0][x];
    }

    fn calc_courant(&self, dt: f32) -> f32 {
        self.wave_speed * dt / self.space_step
    }

    pub fn step(&mut self, dt: f32) {
        let r = self.calc_courant(dt);
        if r > 1.0 {
            warn!(
                "Courant number has a value of {r}, must be below 1.0 to prevent numeric explosion. This step has been skipped"
            );
        }
        let mut new_u = self.u[0];

        for i in 1..N - 1 {
            new_u[i] = (2.0 * self.u[0][i] - self.u[1][i]
                + r * r * (self.u[0][i + 1] - 2.0 * self.u[0][i] + self.u[0][i - 1]))
                * self.damping;
        }

        self.u[1] = self.u[0];
        self.u[0] = new_u;
    }
}
