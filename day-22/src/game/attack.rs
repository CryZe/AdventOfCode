use std::cmp::max;
use super::prelude::*;

pub struct Attack {
    physical_power: usize,
    magic_power: usize,
}

impl Attack {
    pub fn new(physical_power: usize, magic_power: usize) -> Self {
        Attack {
            physical_power: physical_power,
            magic_power: magic_power,
        }
    }

    pub fn calculate_new_health<'a, F: Fighter>(self, fighter: &Fighter) -> usize {
        let magic_damage = self.magic_power;
        let physical_damage = self.physical_power.saturating_sub(fighter.get_defense());
        let damage = max(1, magic_damage + physical_damage);
        if cfg!(feature = "play") {
            println!("-{} HP", damage);
        }
        fighter.get_health().saturating_sub(damage)
    }
}
