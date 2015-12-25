use super::{Attack, EffectState, EffectStats, NamedEffect};
use std::cmp::min;
use std::collections::HashMap;
use super::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};

use super::SpellEffect::*;

pub struct Wizard<'a> {
    health: usize,
    max_health: usize,
    mana: usize,
    standard_mana: usize,
    current_effects: HashMap<String, Box<Effect>>,
    spells: Box<[Box<Spell>]>,
    current_attack_power: usize,
    current_defense: usize,
    controller: RefCell<&'a mut WizardController>,
    giving_up: bool,
}

impl<'a> Wizard<'a> {
    pub fn new(health: usize,
               mana: usize,
               spells: Box<[Box<Spell>]>,
               controller: &'a mut WizardController)
               -> Self {
        Wizard {
            health: health,
            max_health: health,
            mana: mana,
            standard_mana: mana,
            current_effects: HashMap::new(),
            spells: spells,
            current_attack_power: 0,
            current_defense: 0,
            controller: RefCell::new(controller),
            giving_up: false,
        }
    }

    pub fn is_effect_active(&self, effect_name: &str) -> bool {
        self.current_effects.contains_key(effect_name)
    }

    pub fn get_mana(&self) -> usize {
        self.mana
    }

    pub fn get_spells<'b>(&'b self) -> &'b [Box<Spell>] {
        &self.spells
    }

    fn apply_stats(&mut self, stats: EffectStats) {
        self.health = min(self.max_health, self.health + stats.health_regeneration);
        self.mana = self.mana + stats.mana_regeneration;
        self.current_attack_power += stats.attack_power;
        self.current_defense += stats.defense;
    }
}

impl<'a> Display for Wizard<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Wizard")
    }
}

impl<'a> Fighter for Wizard<'a> {
    fn get_health(&self) -> usize {
        self.health
    }

    fn prepare_turn(&mut self, own_turn: bool) {
        if cfg!(feature = "play") {
            println!("Wizard: {} HP {} Mana", self.health, self.mana);

            for (effect, _) in self.current_effects.iter() {
                println!("{} is active", effect);
            }
        }

        let mut deletions = Vec::new();
        let mut current_stats = Vec::new();

        self.current_attack_power = 0;
        self.current_defense = 0;

        for (name, effect) in self.current_effects.iter_mut() {
            let state = effect.cause_effect();
            let mut staged_for_deletion = true;
            if let EffectState::Active(stats) = state {
                staged_for_deletion = stats.remaining_duration == 0;
                current_stats.push(stats);
            }
            if staged_for_deletion {
                deletions.push(name.clone());
            }
        }

        for name in deletions.iter() {
            self.current_effects.remove(name);
        }


        if own_turn {
            let mut spell_effect = None;

            let mut giving_up = false;
            let mut mana_cost = 0;
            {
                let possible_spell = self.controller.borrow_mut().choose_spell(self);
                if let Some(spell) = possible_spell {
                    debug_assert!(spell.can_cast(self));
                    if cfg!(feature = "play") {
                        println!("Wizard casts {}", spell);
                    }

                    mana_cost = spell.get_cost();
                    spell_effect = Some(spell.cast_spell());
                } else {
                    giving_up = true;
                }
            }
            self.mana -= mana_cost;
            self.giving_up = giving_up;

            match spell_effect {
                Some(Immediate(stats)) => {
                    current_stats.push(stats);
                }
                Some(TimeBased(NamedEffect(name, effect))) => {
                    self.current_effects.insert(name, effect);
                }
                _ => {}
            }
        }

        for stats in current_stats {
            self.apply_stats(stats);
        }
    }

    fn take_hit(&mut self, attack: Attack) {
        self.health = attack.calculate_new_health::<Wizard>(self);
    }

    fn get_attack(&self) -> Option<Attack> {
        if self.current_attack_power > 0 {
            Some(Attack::new(0, self.current_attack_power))
        } else {
            None
        }
    }

    fn get_passive_attack(&self) -> Option<Attack> {
        self.get_attack()
    }

    fn get_defense(&self) -> usize {
        self.current_defense
    }

    fn prepare_for_fight(&mut self) {
        self.giving_up = false;
        self.health = self.max_health;
        self.mana = self.standard_mana;
        self.controller.borrow_mut().reset();
    }

    fn is_giving_up(&self) -> bool {
        self.giving_up
    }
}
