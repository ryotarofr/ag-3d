use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub alive: bool,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            alive: true,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        self.current -= damage;
        if self.current <= 0.0 {
            self.current = 0.0;
            self.alive = false;
        }
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100.0)
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, _app: &mut App) {
    }
}