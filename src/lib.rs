pub mod parser;

use inputbot::{KeybdKey, MouseButton};
use parser::entities::{
    actions::{input_action::InputAction, other_action::OtherAction, Action},
    keys::Key,
    Parse, Rule,
};
use pest::iterators::Pair;

#[macro_use]
extern crate pest_derive;

pub trait KeyExt {
    fn press(&self);
    fn release(&self);
}

impl KeyExt for KeybdKey {
    fn press(&self) {
        KeybdKey::press(*self);
    }

    fn release(&self) {
        KeybdKey::release(*self);
    }
}

impl KeyExt for MouseButton {
    fn press(&self) {
        MouseButton::press(*self);
    }

    fn release(&self) {
        MouseButton::release(*self);
    }
}

pub struct Instruction {
    key: parser::entities::keys::Key,
    actions: Vec<Action>,
    r#await: bool,
}

impl Instruction {
    pub fn execute(&self) {
        for action in &self.actions {
            match action {
                Action::Input(action) => match action {
                    InputAction::Press => match self.key {
                        Key::Keyboard(e) => e.press(),
                        Key::Mouse(e) => e.press(),
                    },
                    InputAction::Release => match self.key {
                        Key::Keyboard(e) => e.release(),
                        Key::Mouse(e) => e.release(),
                    },
                },
                Action::Other(action) => match action {
                    OtherAction::Wait(e) => {
                        std::thread::sleep(e.duration);
                    }
                },
            }
        }
    }
}

impl Parse for Instruction {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();

        let key_rule = inner.next().unwrap();

        let mut actions = Vec::new();

        let invokes = inner.next().unwrap().into_inner();
        for invoke in invokes {
            match invoke.as_rule() {
                Rule::invoke => actions.push(Action::parse(invoke)),
                _ => unreachable!(),
            }
        }

        Self {
            key: Key::parse(key_rule),
            actions,
            r#await: inner.next().is_some(),
        }
    }
}
