pub struct EffectStats {
    pub remaining_duration: usize,
    pub attack_power: usize,
    pub defense: usize,
    pub health_regeneration: usize,
    pub mana_regeneration: usize,
}

impl EffectStats {
    pub fn new(remaining_duration: usize,
               attack_power: usize,
               defense: usize,
               health_regeneration: usize,
               mana_regeneration: usize)
               -> Self {
        EffectStats {
            remaining_duration: remaining_duration,
            attack_power: attack_power,
            defense: defense,
            health_regeneration: health_regeneration,
            mana_regeneration: mana_regeneration,
        }
    }
}

pub enum EffectState {
    Active(EffectStats),
    Finished,
}

pub struct NamedEffect(pub String, pub Box<Effect>);

pub trait Effect {
    fn cause_effect(&mut self) -> EffectState;
}
