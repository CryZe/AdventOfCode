use super::game::prelude::*;
use super::game::wizardcontrollers::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use super::game::spells::get_default_spells;
use super::game::{Boss, Wizard, FightResult};

type Controller = RoundCountingController<ManaRecordingController<RecordedController>>;

impl Controller {
    fn from_instructions(instructions: Box<[usize]>) -> Self {
        RoundCountingController::new(ManaRecordingController::new(RecordedController::new(instructions)))
    }

    fn get_mana_usage(&self) -> usize {
        self.inner_controller.mana_usage
    }

    fn is_finished(&self) -> bool {
        self.inner_controller.inner_controller.instructions.len() >= self.rounds
    }
}

#[derive(Eq)]
struct Path {
    mana_usage: usize,
    spells: Vec<usize>,
}

impl Path {
    fn new(mana_usage: usize, spells: Vec<usize>) -> Self {
        Path {
            mana_usage: mana_usage,
            spells: spells,
        }
    }

    fn create_controller(&self) -> Controller {
        Controller::from_instructions(self.spells.clone().into_boxed_slice())
    }

    fn create_sub_paths(&self, spell_count: usize) -> Vec<Path> {
        let mut paths = Vec::new();
        for spell in 0..spell_count {
            let mut spells = self.spells.clone();
            spells.push(spell);
            let mut path = Path::new(0, spells);
            let (_, mana_usage) = simulate_fight(&mut path.create_controller());
            path.mana_usage = mana_usage;
            paths.push(path);
        }
        paths
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.mana_usage.eq(&other.mana_usage)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.mana_usage.partial_cmp(&self.mana_usage)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_usage.cmp(&self.mana_usage)
    }
}

enum Result {
    Win,
    Loss,
    InProgress,
}

use self::Result::*;

fn simulate_fight(controller: &mut Controller) -> (Result, usize) {
    let result = {
        let spells = get_default_spells();
        let mut player = Wizard::new(50, 500, spells, controller);
        let mut enemy = Boss::default();

        player.fight(&mut enemy)
    };
    let mana_usage = controller.get_mana_usage();
    let result = match result {
        FightResult::Win => Win,
        FightResult::Loss => {
            if controller.is_finished() {
                Loss
            } else {
                InProgress
            }
        }
    };
    (result, mana_usage)
}

pub fn find_optimized_fight() -> Option<RecordedController> {
    // Set up the Min Heap
    let initial_path = Path::new(0, Vec::new());
    let mut heap = BinaryHeap::new();
    heap.push(initial_path);
    let spell_count = get_default_spells().len();

    while let Some(path) = heap.pop() {
        let mut controller = path.create_controller();
        let (result, _) = simulate_fight(&mut controller);
        match result {
            Win => {
                return Some(controller.inner_controller.inner_controller);
            }
            Loss => {
                // Don't explore this path further
            }
            InProgress => {
                let paths = path.create_sub_paths(spell_count);
                heap.extend(paths);
            }
        }
    }

    None
}
