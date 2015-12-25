use super::items::DAGGER;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Item {
    pub cost: usize,
    pub attack_power: usize,
    pub defense: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Weapon(pub Item);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Armor(pub Item);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ring(pub Item);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Equipment {
    weapon: Weapon,
    armor: Option<Armor>,
    left_ring: Option<Ring>,
    right_ring: Option<Ring>,
}

impl Equipment {
    pub fn new(weapon: Weapon,
               armor: Option<Armor>,
               left_ring: Option<Ring>,
               right_ring: Option<Ring>)
               -> Self {
        Equipment {
            weapon: weapon,
            armor: armor,
            left_ring: left_ring,
            right_ring: right_ring,
        }
    }

    pub fn get_cost(&self) -> usize {
        self.weapon.0.cost + self.armor.map(|w| w.0.cost).unwrap_or(0) +
        self.left_ring.map(|w| w.0.cost).unwrap_or(0) +
        self.right_ring.map(|w| w.0.cost).unwrap_or(0)
    }

    pub fn get_attack_power(&self) -> usize {
        self.weapon.0.attack_power + self.armor.map(|w| w.0.attack_power).unwrap_or(0) +
        self.left_ring.map(|w| w.0.attack_power).unwrap_or(0) +
        self.right_ring.map(|w| w.0.attack_power).unwrap_or(0)
    }

    pub fn get_defense(&self) -> usize {
        self.weapon.0.defense + self.armor.map(|w| w.0.defense).unwrap_or(0) +
        self.left_ring.map(|w| w.0.defense).unwrap_or(0) +
        self.right_ring.map(|w| w.0.defense).unwrap_or(0)
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment::new(DAGGER, None, None, None)
    }
}
