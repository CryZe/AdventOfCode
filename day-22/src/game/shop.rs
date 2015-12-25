use super::{Weapon, Armor, Ring};
use super::items::*;

pub struct Shop {
    pub weapons: Box<[Weapon]>,
    pub armors: Box<[Armor]>,
    pub rings: Box<[Ring]>,
}

impl Default for Shop {
    fn default() -> Self {
        Shop {
            weapons: Box::new([DAGGER, SHORTSWORD, WARHAMMER, LONGSWORD, GREATAXE]),
            armors: Box::new([LEATHER, CHAINMAIL, SPLINTMAIL, BANDEDMAIL, PLATEMAIL]),
            rings: Box::new([DAMAGE1, DAMAGE2, DAMAGE3, DEFENSE1, DEFENSE2, DEFENSE3]),
        }
    }
}
