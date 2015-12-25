use super::Attack;
use std::fmt::Display;

pub trait Fighter: Display {
    fn get_health(&self) -> usize;
    fn take_hit(&mut self, attack: Attack);
    fn get_attack(&self) -> Option<Attack>;
    fn get_defense(&self) -> usize;
    fn prepare_for_fight(&mut self);
    fn prepare_turn(&mut self, own_turn: bool);
    fn get_passive_attack(&self) -> Option<Attack> {
        None
    }
    fn is_giving_up(&self) -> bool {
        false
    }
}

#[derive(PartialEq)]
pub enum FightResult {
    Win,
    Loss,
}

pub trait FighterExtensions {
    fn is_alive(&self) -> bool;
    fn fight<F: Fighter>(&mut self, other: &mut F) -> FightResult;
}

impl<T> FighterExtensions for T where T: Fighter
{
    fn is_alive(&self) -> bool {
        return self.get_health() > 0;
    }

    fn fight<F: Fighter>(&mut self, other: &mut F) -> FightResult {
        self.prepare_for_fight();
        other.prepare_for_fight();

        loop {
            if cfg!(feature = "play") {
                println!("");
                println!("-- {} turn --", self);
            }

            if cfg!(feature = "hard") {
                if cfg!(feature = "play") {
                    print!("Hard Mode: ");
                }
                self.take_hit(Attack::new(0, 1));

                if !self.is_alive() {
                    return FightResult::Loss;
                }
            }

            self.prepare_turn(true);
            other.prepare_turn(false);

            if let Some(passive_attack) = other.get_passive_attack() {
                if cfg!(feature = "play") {
                    print!("Passive Damage to {}: ", self);
                }
                self.take_hit(passive_attack);
                if !self.is_alive() {
                    return FightResult::Loss;
                }
            }

            if self.is_giving_up() {
                if cfg!(feature = "play") {
                    println!("{} gave up", self);
                }
                return FightResult::Loss;
            }

            if let Some(attack) = self.get_attack() {
                if cfg!(feature = "play") {
                    print!("Active Damage to {}: ", other);
                }
                other.take_hit(attack);
            }

            if !other.is_alive() {
                return FightResult::Win;
            }

            if cfg!(feature = "play") {
                println!("");
                println!("-- {} turn --", other);
            }

            self.prepare_turn(false);
            other.prepare_turn(true);

            if let Some(passive_attack) = self.get_passive_attack() {
                if cfg!(feature = "play") {
                    print!("Passive Damage to {}: ", other);
                }
                other.take_hit(passive_attack);
                if !other.is_alive() {
                    return FightResult::Win;
                }
            }

            if other.is_giving_up() {
                if cfg!(feature = "play") {
                    println!("{} gave up", other);
                }
                return FightResult::Win;
            }

            if let Some(attack) = other.get_attack() {
                if cfg!(feature = "play") {
                    print!("Active Damage to {}: ", self);
                }
                self.take_hit(attack);
            }

            if !self.is_alive() {
                return FightResult::Loss;
            }
        }
    }
}
