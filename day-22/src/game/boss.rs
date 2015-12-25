use super::{Attack, Fighter};
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Boss {
    max_health: usize,
    health: usize,
    attack_power: usize,
    defense: usize,
}

pub const DEFAULT_BOSS: Boss = Boss {
    max_health: 58,
    health: 58,
    attack_power: 9,
    defense: 2,
};

impl Display for Boss {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Boss")
    }
}

impl Fighter for Boss {
    fn get_health(&self) -> usize {
        self.health
    }

    fn prepare_turn(&mut self, _: bool) {
        if cfg!(feature = "play") {
            println!("Boss: {} HP", self.health);
        }
    }

    fn take_hit(&mut self, attack: Attack) {
        self.health = attack.calculate_new_health::<Boss>(self);
    }

    fn get_attack(&self) -> Option<Attack> {
        Some(Attack::new(self.attack_power, 0))
    }

    fn get_defense(&self) -> usize {
        self.defense
    }

    fn prepare_for_fight(&mut self) {
        self.health = self.max_health
    }
}

impl Default for Boss {
    fn default() -> Self {
        DEFAULT_BOSS
    }
}
