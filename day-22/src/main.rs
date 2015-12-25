pub mod game;
pub mod fightoptimizing;

use game::prelude::*;
use game::{Boss, Wizard};
use game::wizardcontrollers::*;
use game::spells::*;
use game::FightResult::*;

#[cfg(not(feature = "play"))]
use fightoptimizing::find_optimized_fight;

#[cfg(feature = "play")]
fn main() {
    let mut controller = ManaRecordingController::new(StdInController);
    let spells = get_default_spells();

    let result = {
        let mut player = Wizard::new(50, 500, spells, &mut controller);
        let mut enemy = Boss::default();

        player.fight(&mut enemy)
    };

    match result {
        Win => println!("Won with {} mana used :)", controller.mana_usage),
        Loss => println!("Lost :("),
    }
}

#[cfg(not(feature = "play"))]
fn main() {
    let mut controller = ManaRecordingController::new(find_optimized_fight()
                                                          .expect("Couldn't find a winning fight"));
    let spells = get_default_spells();

    for spell in controller.inner_controller.instructions.iter() {
        println!("{}", spells[*spell]);
    }

    let result = {
        let mut player = Wizard::new(50, 500, spells, &mut controller);
        let mut enemy = Boss::default();

        player.fight(&mut enemy)
    };

    match result {
        Win => println!("Won with {} mana used :)", controller.mana_usage),
        Loss => println!("Lost :("),
    }
}
