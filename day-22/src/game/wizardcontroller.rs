use super::Wizard;
use super::prelude::*;

pub trait WizardController {
    fn choose_spell<'a>(&mut self, wizard: &'a Wizard) -> Option<&'a Spell>;
    fn reset(&mut self) {}
}
