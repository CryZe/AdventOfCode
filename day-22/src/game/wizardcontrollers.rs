use super::Wizard;
use super::prelude::*;
use std::io::stdin;

pub struct StdInController;

impl WizardController for StdInController {
    fn choose_spell<'a>(&mut self, wizard: &'a Wizard) -> Option<&'a Spell> {
        let selection = wizard.get_spells()
                              .into_iter()
                              .filter(|s| s.can_cast(wizard))
                              .enumerate()
                              .map(|(i, s)| format!("[{}] {}", i, s))
                              .collect::<Vec<_>>()
                              .join(" ");
        println!("{}", selection);
        let mut line = String::new();
        let _ = stdin().read_line(&mut line);
        let index = line.lines().nth(0).and_then(|l| l.parse::<usize>().ok());
        index.and_then(|i| {
            wizard.get_spells()
                  .into_iter()
                  .filter(|s| s.can_cast(wizard))
                  .nth(i)
                  .map(|s| s.as_ref())
        })
    }
}

pub struct ManaRecordingController<T: WizardController> {
    pub mana_usage: usize,
    pub inner_controller: T,
}

impl<T: WizardController> ManaRecordingController<T> {
    pub fn new(inner_controller: T) -> Self {
        ManaRecordingController {
            mana_usage: 0,
            inner_controller: inner_controller,
        }
    }
}

impl<T: WizardController> WizardController for ManaRecordingController<T> {
    fn choose_spell<'a>(&mut self, wizard: &'a Wizard) -> Option<&'a Spell> {
        let spell = self.inner_controller.choose_spell(wizard);
        if let Some(spell) = spell {
            self.mana_usage += spell.get_cost();
        }
        spell
    }

    fn reset(&mut self) {
        self.mana_usage = 0;
        self.inner_controller.reset();
    }
}

pub struct RecordedController {
    pub instructions: Box<[usize]>,
    current_index: usize,
}

impl RecordedController {
    pub fn new(instructions: Box<[usize]>) -> Self {
        RecordedController {
            instructions: instructions,
            current_index: 0,
        }
    }
}

impl WizardController for RecordedController {
    fn choose_spell<'a>(&mut self, wizard: &'a Wizard) -> Option<&'a Spell> {
        let i = self.instructions.iter().cloned().nth(self.current_index);
        self.current_index += 1;
        i.and_then(|i| {
            wizard.get_spells()
                  .iter()
                  .nth(i)
                  .map(|s| s.as_ref())
                  .and_then(|s| {
                      if s.can_cast(wizard) {
                          Some(s)
                      } else {
                          None
                      }
                  })
        })
    }

    fn reset(&mut self) {
        self.current_index = 0;
    }
}

pub struct RoundCountingController<T: WizardController> {
    pub rounds: usize,
    pub inner_controller: T,
}

impl<T: WizardController> RoundCountingController<T> {
    pub fn new(inner_controller: T) -> Self {
        RoundCountingController {
            rounds: 0,
            inner_controller: inner_controller,
        }
    }
}

impl<T: WizardController> WizardController for RoundCountingController<T> {
    fn choose_spell<'a>(&mut self, wizard: &'a Wizard) -> Option<&'a Spell> {
        self.rounds += 1;
        self.inner_controller.choose_spell(wizard)
    }

    fn reset(&mut self) {
        self.rounds = 0;
        self.inner_controller.reset();
    }
}
