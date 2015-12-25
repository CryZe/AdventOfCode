use std::fmt::Display;
use super::{NamedEffect, Wizard, EffectStats};

pub enum SpellEffect {
    Immediate(EffectStats),
    TimeBased(NamedEffect),
}

pub struct NamedSpell(String, Box<Spell>);

pub trait Spell : Display {
    fn get_cost(&self) -> usize;
    fn can_cast(&self, wizard: &Wizard) -> bool;
    fn cast_spell(&self) -> SpellEffect;
}
