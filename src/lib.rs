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
#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::Instruction;
    use crate::parser::entities::{
        actions::{
            input_action::InputAction,
            other_action::{OtherAction, WaitAction},
            Action,
        },
        keys::Key,
        GrammarParser, Parse, Rule,
    };
    use pest::Parser;

    use inputbot::{MouseButton, KeybdKey};
    use std::time::Duration;

    #[test]
    fn key_simple_instruction() {
        assert_eq!(
            Instruction::parse(
                GrammarParser::parse(Rule::instruction, "SpaceKey.press()")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Instruction {
                key: Key::Keyboard(KeybdKey::SpaceKey),
                actions: vec![Action::Input(InputAction::Press)],
                r#await: false
            }
        );
    }

    #[test]
    fn key_complex_instruction() {
        assert_eq!(
            Instruction::parse(
                GrammarParser::parse(
                    Rule::instruction,
                    "OtherKey(256).press().wait(250ms).release()"
                )
                .unwrap()
                .next()
                .unwrap()
            ),
            Instruction {
                key: Key::Keyboard(KeybdKey::OtherKey(256)),
                actions: vec![
                    Action::Input(InputAction::Press),
                    Action::Other(OtherAction::Wait(WaitAction {
                        duration: Duration::from_millis(250)
                    })),
                    Action::Input(InputAction::Release),
                ],
                r#await: false
            }
        );
    }

    #[test]
    fn button_simple_instruction() {
        assert_eq!(
            Instruction::parse(
                GrammarParser::parse(Rule::instruction, "RightButton.press()")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Instruction {
                key: Key::Mouse(MouseButton::RightButton),
                actions: vec![Action::Input(InputAction::Press)],
                r#await: false
            }
        );
    }

    #[test]
    fn button_complex_instruction() {
        assert_eq!(
            Instruction::parse(
                GrammarParser::parse(
                    Rule::instruction,
                    "OtherButton(256).press().wait(5s).release()"
                )
                .unwrap()
                .next()
                .unwrap()
            ),
            Instruction {
                key: Key::Mouse(MouseButton::OtherButton(256)),
                actions: vec![
                    Action::Input(InputAction::Press),
                    Action::Other(OtherAction::Wait(WaitAction {
                        duration: Duration::from_secs_f64(5_f64)
                    })),
                    Action::Input(InputAction::Release),
                ],
                r#await: false
            }
        );
    }

    #[test]
    fn await_instruction() {
        assert_eq!(
            Instruction::parse(
                GrammarParser::parse(
                    Rule::instruction,
                    "OtherButton(256).press().wait(5s).release().await"
                )
                .unwrap()
                .next()
                .unwrap()
            ),
            Instruction {
                key: Key::Mouse(MouseButton::OtherButton(256)),
                actions: vec![
                    Action::Input(InputAction::Press),
                    Action::Other(OtherAction::Wait(WaitAction {
                        duration: Duration::from_secs_f64(5_f64)
                    })),
                    Action::Input(InputAction::Release),
                ],
                r#await: true
            }
        );
    }
}
