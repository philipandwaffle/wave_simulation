use crate::consts::N_2D;
use bevy::ecs::resource::Resource;

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

pub struct Simulation {
    pub title: String,
    pub initial_u: [f32; N_2D],
    duration: f32,
}
impl Simulation {
    pub fn new(title: &str, initial_u: [f32; N_2D], duration: f32) -> Self {
        Self {
            title: title.to_string(),
            initial_u,
            duration,
        }
    }
}
