use super::{Attack, Fighter};
use super::Equipment;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct SwordFighter {
    max_health: usize,
    health: usize,
    equipment: Equipment,
}

impl SwordFighter {
    pub fn new(health: usize, equipment: Equipment) -> Self {
        SwordFighter {
            max_health: health,
            health: health,
            equipment: equipment,
        }
    }

    pub fn set_equipment(&mut self, equipment: Equipment) {
        self.equipment = equipment
    }
}

impl Default for SwordFighter {
    fn default() -> Self {
        SwordFighter::new(100, Equipment::default())
    }
}

impl Display for SwordFighter {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Sword Fighter")
    }
}

impl Fighter for SwordFighter {
    fn get_health(&self) -> usize {
        self.health
    }

    fn prepare_turn(&mut self, _: bool) {}

    fn take_hit(&mut self, attack: Attack) {
        self.health = attack.calculate_new_health::<SwordFighter>(self);
    }

    fn get_attack(&self) -> Option<Attack> {
        Some(Attack::new(self.equipment.get_attack_power(), 0))
    }

    fn get_defense(&self) -> usize {
        self.equipment.get_defense()
    }

    fn prepare_for_fight(&mut self) {
        self.health = self.max_health
    }
}
