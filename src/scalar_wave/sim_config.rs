use std::f32::consts::PI;

use bevy::{
    ecs::resource::Resource,
    math::{Vec2, ops::cos},
};

#[derive(Resource)]
pub struct SimConfig {
    paused: bool,
    pub space_step: f32,
    pub wave_speed: f32,
    pub damping: f32,
    elapsed: f32,
    cur_duration: f32,
    simulations: Vec<Simulation>,
}
impl SimConfig {
    pub fn new(space_step: f32, wave_speed: f32, damping: f32) -> Self {
        Self {
            paused: true,
            space_step,
            wave_speed,
            damping,
            elapsed: 0.0,
            cur_duration: 0.0,
            simulations: vec![],
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub fn tick(&mut self, dt: f32) {
        if !self.paused {
            self.elapsed += dt;
        }
    }

    pub fn with_simulations(mut self, simulations: Vec<Simulation>) -> Self {
        self.simulations = simulations;
        self
    }

    pub fn should_change_sim(&self) -> bool {
        let cur_num_sims = self.simulations.len();

        if cur_num_sims == 0 {
            false
        } else {
            self.elapsed >= self.cur_duration
        }
    }

    pub fn next_sim(&mut self) -> Option<Simulation> {
        self.elapsed = 0.0;
        self.simulations.pop().and_then(|sim| {
            self.cur_duration = sim.duration;
            Some(sim)
        })
    }
}

#[derive(Clone)]
pub struct Simulation {
    title: String,
    xy_points: Vec<Vec2>,
    initial_u: Vec<f32>,
    duration: f32,
}
impl Simulation {
    pub fn new(title: &str, xy_points: Vec<Vec2>, initial_u: Vec<f32>, duration: f32) -> Self {
        Self {
            title: title.to_string(),
            xy_points,
            initial_u,
            duration,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn initial_u(&self) -> Vec<f32> {
        self.initial_u.clone()
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    fn cosine_map(x: f32, max_x: f32, max_y: f32) -> f32 {
        max_x * (1.0 + cos((PI * x) / max_y))
    }

    pub fn with_cosine_dot(mut self, max_magnitude: f32, centre: Vec2, radius: f32) -> Self {
        for (index, cur_pos) in self.xy_points.iter().enumerate() {
            let dist = (cur_pos - centre).length();
            if dist <= radius {
                self.initial_u[index] = Self::cosine_map(dist, max_magnitude, radius);
            }
        }
        self
    }

    pub fn with_cosine_ring(
        mut self,
        max_magnitude: f32,
        centre: Vec2,
        width: f32,
        radius: f32,
    ) -> Self {
        for (index, cur_pos) in self.xy_points.iter().enumerate() {
            let dist = (cur_pos - centre).length();
            let diff = (radius - dist).abs();
            if diff < width {
                self.initial_u[index] = Self::cosine_map(diff, max_magnitude, width);
            }
        }
        self
    }

    pub fn with_cosine_line(
        mut self,
        max_magnitude: f32,
        dist: f32,
        width: f32,
        x_axis: bool,
    ) -> Self {
        for (index, cur_pos) in self.xy_points.iter().enumerate() {
            if x_axis {
                let x_diff = (cur_pos.x - dist).abs();
                if x_diff < width {
                    self.initial_u[index] = Self::cosine_map(x_diff, max_magnitude, width);
                }
            } else {
                let y_diff = (cur_pos.y - dist).abs();
                if y_diff < width {
                    self.initial_u[index] = Self::cosine_map(y_diff, max_magnitude, width);
                }
            }
        }

        self
    }
}
