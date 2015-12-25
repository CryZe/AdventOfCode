use super::Fighter;

pub struct Boss {
    max_health: usize,
    health: usize,
    attack_power: usize,
    defense: usize,
}

pub const DEFAULT_BOSS: Boss = Boss {
    max_health: 100,
    health: 100,
    attack_power: 8,
    defense: 2,
};

impl Fighter for Boss {
    fn get_health(&self) -> usize {
        self.health
    }

    fn take_hit(&mut self, attack_power: usize) {
        self.health = self.health.saturating_sub(attack_power.saturating_sub(self.defense));
    }

    fn get_attack_power(&self) -> usize {
        self.attack_power
    }

    fn get_defense(&self) -> usize {
        self.defense
    }

    fn replenish_health(&mut self) {
        self.health = self.max_health
    }
}

impl Default for Boss {
    fn default() -> Self {
        DEFAULT_BOSS
    }
}
