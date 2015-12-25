pub mod fighter; //This seems weird
mod equipment;
mod shop;
mod boss;
mod swordfighter;
pub mod spell;
pub mod effect;
pub mod items;
mod wizard;
pub mod spells;
pub mod effects;
pub mod wizardcontroller;
mod attack;
pub mod wizardcontrollers;

pub use self::fighter::*;
pub use self::equipment::*;
pub use self::shop::*;
pub use self::boss::*;
pub use self::swordfighter::*;
pub use self::spell::*;
pub use self::effect::*;
pub use self::wizard::*;
pub use self::wizardcontroller::*;
pub use self::attack::*;

pub mod prelude {
    pub use super::{Fighter, FighterExtensions};
    pub use super::Effect;
    pub use super::Spell;
    pub use super::WizardController;
}
