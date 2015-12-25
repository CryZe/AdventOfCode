use super::{Effect, EffectState, EffectStats};
use super::EffectState::*;

pub struct DrainEffect {
    remaining_duration: usize,
    damage: usize,
    health_regeneration: usize,
}

impl DrainEffect {
    pub fn new(duration: usize, damage: usize, health_regeneration: usize) -> Self {
        DrainEffect {
            remaining_duration: duration,
            damage: damage,
            health_regeneration: health_regeneration,
        }
    }
}

impl Effect for DrainEffect {
    fn cause_effect(&mut self) -> EffectState {
        if self.remaining_duration > 0 {
            self.remaining_duration = self.remaining_duration.saturating_sub(1);
            Active(EffectStats::new(self.remaining_duration,
                                    self.damage,
                                    0,
                                    self.health_regeneration,
                                    0))
        } else {
            Finished
        }
    }
}

pub struct ShieldEffect {
    remaining_duration: usize,
    defense: usize,
}

impl ShieldEffect {
    pub fn new(duration: usize, defense: usize) -> Self {
        ShieldEffect {
            remaining_duration: duration,
            defense: defense,
        }
    }
}

impl Effect for ShieldEffect {
    fn cause_effect(&mut self) -> EffectState {
        if self.remaining_duration > 0 {
            self.remaining_duration = self.remaining_duration.saturating_sub(1);
            Active(EffectStats::new(self.remaining_duration, 0, self.defense, 0, 0))
        } else {
            Finished
        }
    }
}

pub struct DamagingEffect {
    remaining_duration: usize,
    attack: usize,
}

impl DamagingEffect {
    pub fn new(duration: usize, attack: usize) -> Self {
        DamagingEffect {
            remaining_duration: duration,
            attack: attack,
        }
    }
}

impl Effect for DamagingEffect {
    fn cause_effect(&mut self) -> EffectState {
        if self.remaining_duration > 0 {
            self.remaining_duration = self.remaining_duration.saturating_sub(1);
            Active(EffectStats::new(self.remaining_duration, self.attack, 0, 0, 0))
        } else {
            Finished
        }
    }
}

pub struct HealingEffect {
    remaining_duration: usize,
    health_regeneration: usize,
}

impl HealingEffect {
    pub fn new(duration: usize, health_regeneration: usize) -> Self {
        HealingEffect {
            remaining_duration: duration,
            health_regeneration: health_regeneration,
        }
    }
}

impl Effect for HealingEffect {
    fn cause_effect(&mut self) -> EffectState {
        if self.remaining_duration > 0 {
            self.remaining_duration = self.remaining_duration.saturating_sub(1);
            Active(EffectStats::new(self.remaining_duration, 0, 0, self.health_regeneration, 0))
        } else {
            Finished
        }
    }
}

pub struct RechargingEffect {
    remaining_duration: usize,
    mana_regeneration: usize,
}

impl RechargingEffect {
    pub fn new(duration: usize, mana_regeneration: usize) -> Self {
        RechargingEffect {
            remaining_duration: duration,
            mana_regeneration: mana_regeneration,
        }
    }
}

impl Effect for RechargingEffect {
    fn cause_effect(&mut self) -> EffectState {
        if self.remaining_duration > 0 {
            self.remaining_duration = self.remaining_duration.saturating_sub(1);
            Active(EffectStats::new(self.remaining_duration, 0, 0, 0, self.mana_regeneration))
        } else {
            Finished
        }
    }
}
