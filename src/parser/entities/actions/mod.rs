use pest::iterators::Pair;

use crate::{InputAction, KeyExt, OtherAction};

use super::{Parse, Rule};

pub mod input_action;
pub mod other_action;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Input(InputAction),
    Other(OtherAction),
}

impl Parse for Action {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::input_invoke => {
                Action::Input(InputAction::parse(pair.into_inner().next().unwrap()))
            }
            Rule::other_invoke => Action::Other(OtherAction::parse(pair)),
            _ => unreachable!(),
        }
    }
}

pub trait Invoke {
    fn invoke<T>(&self, key: &T)
    where
        T: KeyExt;
}

impl Invoke for Action {
    fn invoke<T>(&self, key: &T)
    where
        T: KeyExt,
    {
        match self {
            Action::Input(e) => e.invoke(key),
            Action::Other(e) => e.invoke(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::entities::actions::{
        input_action::InputAction,
        other_action::{OtherAction, WaitAction},
        Action,
    };
    use crate::parser::entities::{GrammarParser, Parse, Rule};
    use pest::Parser;

    use std::time::Duration;

    #[test]
    fn input_invoke() {
        assert_eq!(
            Action::parse(
                GrammarParser::parse(Rule::invoke, "press()")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Action::Input(InputAction::Press)
        );
        assert_eq!(
            Action::parse(
                GrammarParser::parse(Rule::invoke, "release()")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Action::Input(InputAction::Release)
        );
    }

    #[test]
    fn other_invoke() {
        assert_eq!(
            Action::parse(
                GrammarParser::parse(Rule::invoke, "wait(250ms)")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Action::Other(OtherAction::Wait(WaitAction {
                duration: Duration::from_millis(250)
            }))
        );
    }
}
