use super::Fighter;
use super::Equipment;

pub struct Player {
    max_health: usize,
    health: usize,
    equipment: Equipment,
}

impl Player {
    pub fn new(health: usize, equipment: Equipment) -> Self {
        Player {
            max_health: health,
            health: health,
            equipment: equipment,
        }
    }

    pub fn set_equipment(&mut self, equipment: Equipment) {
        self.equipment = equipment
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(100, Equipment::default())
    }
}

impl Fighter for Player {
    fn get_health(&self) -> usize {
        self.health
    }

    fn take_hit(&mut self, attack_power: usize) {
        self.health = self.health.saturating_sub(attack_power.saturating_sub(self.equipment
                                                                                 .get_defense()));
    }

    fn get_attack_power(&self) -> usize {
        self.equipment.get_attack_power()
    }

    fn get_defense(&self) -> usize {
        self.equipment.get_defense()
    }

    fn replenish_health(&mut self) {
        self.health = self.max_health
    }
}
