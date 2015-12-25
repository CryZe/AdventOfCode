pub mod fighter; //This seems weird
mod equipment;
mod shop;
mod boss;
mod player;
pub mod items;

pub use self::fighter::*;
pub use self::equipment::*;
pub use self::shop::*;
pub use self::boss::*;
pub use self::player::*;

pub mod prelude {
    pub use super::{Fighter, FighterExtensions};
}
