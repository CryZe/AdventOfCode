use super::{NamedEffect, EffectState, Wizard, SpellEffect};
use super::SpellEffect::*;
use super::effects::*;
use super::prelude::*;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct MagicMissile;

impl Display for MagicMissile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Magic Missile")
    }
}

impl Spell for MagicMissile {
    fn get_cost(&self) -> usize {
        53
    }

    fn can_cast(&self, wizard: &Wizard) -> bool {
        wizard.get_mana() >= self.get_cost()
    }

    fn cast_spell(&self) -> SpellEffect {
        let mut effect = DamagingEffect::new(1, 4);
        if let EffectState::Active(stats) = effect.cause_effect() {
            Immediate(stats)
        } else {
            unreachable!();
        }
    }
}

pub struct Drain;

impl Display for Drain {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Drain")
    }
}

impl Spell for Drain {
    fn get_cost(&self) -> usize {
        73
    }

    fn can_cast(&self, wizard: &Wizard) -> bool {
        wizard.get_mana() >= self.get_cost()
    }

    fn cast_spell(&self) -> SpellEffect {
        let mut effect = DrainEffect::new(1, 2, 2);
        if let EffectState::Active(stats) = effect.cause_effect() {
            Immediate(stats)
        } else {
            unreachable!();
        }
    }
}

pub struct Shield;

impl Display for Shield {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Shield")
    }
}

impl Spell for Shield {
    fn get_cost(&self) -> usize {
        113
    }

    fn can_cast(&self, wizard: &Wizard) -> bool {
        wizard.get_mana() >= self.get_cost() && !wizard.is_effect_active("Shield")
    }

    fn cast_spell(&self) -> SpellEffect {
        let effect = ShieldEffect::new(6, 7);
        TimeBased(NamedEffect("Shield".to_owned(), Box::new(effect)))
    }
}

pub struct Poison;

impl Display for Poison {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Poison")
    }
}

impl Spell for Poison {
    fn get_cost(&self) -> usize {
        173
    }

    fn can_cast(&self, wizard: &Wizard) -> bool {
        wizard.get_mana() >= self.get_cost() && !wizard.is_effect_active("Poison")
    }

    fn cast_spell(&self) -> SpellEffect {
        let effect = DamagingEffect::new(6, 3);
        TimeBased(NamedEffect("Poison".to_owned(), Box::new(effect)))
    }
}

pub struct Recharge;

impl Display for Recharge {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Recharge")
    }
}

impl Spell for Recharge {
    fn get_cost(&self) -> usize {
        229
    }

    fn can_cast(&self, wizard: &Wizard) -> bool {
        wizard.get_mana() >= self.get_cost() && !wizard.is_effect_active("Recharge")
    }


    fn cast_spell(&self) -> SpellEffect {
        let effect = RechargingEffect::new(5, 101);
        TimeBased(NamedEffect("Recharge".to_owned(), Box::new(effect)))
    }
}

pub fn get_default_spells() -> Box<[Box<Spell>]> {
    Box::new([Box::new(MagicMissile),
              Box::new(Drain),
              Box::new(Shield),
              Box::new(Poison),
              Box::new(Recharge)])
}
