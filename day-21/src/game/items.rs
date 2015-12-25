use super::{Item, Weapon, Armor, Ring};

pub const DAGGER: Weapon = Weapon(Item {
    cost: 8,
    attack_power: 4,
    defense: 0,
});
pub const SHORTSWORD: Weapon = Weapon(Item {
    cost: 10,
    attack_power: 5,
    defense: 0,
});
pub const WARHAMMER: Weapon = Weapon(Item {
    cost: 25,
    attack_power: 6,
    defense: 0,
});
pub const LONGSWORD: Weapon = Weapon(Item {
    cost: 40,
    attack_power: 7,
    defense: 0,
});
pub const GREATAXE: Weapon = Weapon(Item {
    cost: 74,
    attack_power: 8,
    defense: 0,
});

pub const LEATHER: Armor = Armor(Item {
    cost: 13,
    attack_power: 0,
    defense: 1,
});
pub const CHAINMAIL: Armor = Armor(Item {
    cost: 31,
    attack_power: 0,
    defense: 2,
});
pub const SPLINTMAIL: Armor = Armor(Item {
    cost: 53,
    attack_power: 0,
    defense: 3,
});
pub const BANDEDMAIL: Armor = Armor(Item {
    cost: 75,
    attack_power: 0,
    defense: 4,
});
pub const PLATEMAIL: Armor = Armor(Item {
    cost: 102,
    attack_power: 0,
    defense: 5,
});

pub const DAMAGE1: Ring = Ring(Item {
    cost: 25,
    attack_power: 1,
    defense: 0,
});
pub const DAMAGE2: Ring = Ring(Item {
    cost: 50,
    attack_power: 2,
    defense: 0,
});
pub const DAMAGE3: Ring = Ring(Item {
    cost: 100,
    attack_power: 3,
    defense: 0,
});
pub const DEFENSE1: Ring = Ring(Item {
    cost: 20,
    attack_power: 0,
    defense: 1,
});
pub const DEFENSE2: Ring = Ring(Item {
    cost: 40,
    attack_power: 0,
    defense: 2,
});
pub const DEFENSE3: Ring = Ring(Item {
    cost: 80,
    attack_power: 0,
    defense: 3,
});
