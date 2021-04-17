use pest::iterators::Pair;

use crate::{InputAction, KeyExt, OtherAction};

use super::{Parse, Rule};

pub mod input_action;
pub mod other_action;

pub enum Action {
    Input(InputAction),
    Other(OtherAction),
}

impl Parse for Action {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::input_action => Action::Input(InputAction::parse(pair)),
            Rule::other_action => Action::Other(OtherAction::parse(pair)),
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
