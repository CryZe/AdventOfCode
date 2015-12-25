pub trait Fighter {
    fn get_health(&self) -> usize;
    fn take_hit(&mut self, attack_power: usize);
    fn get_attack_power(&self) -> usize;
    fn get_defense(&self) -> usize;
    fn replenish_health(&mut self);
}

#[derive(PartialEq)]
pub enum FightResult {
    Win,
    Loss,
    Unknown,
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
        self.replenish_health();
        other.replenish_health();

        if self.get_attack_power() <= other.get_defense() &&
           other.get_attack_power() <= self.get_defense() {
            return FightResult::Unknown;
        }

        loop {
            other.take_hit(self.get_attack_power());
            if !other.is_alive() {
                return FightResult::Win;
            }

            self.take_hit(other.get_attack_power());
            if !self.is_alive() {
                return FightResult::Loss;
            }
        }
    }
}
