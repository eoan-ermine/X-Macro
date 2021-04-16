use pest::iterators::Pair;

use crate::{
    parser::entities::{Parse, Rule},
    Key,
};

use super::Invoke;

pub enum InputAction {
    Press,
    Release,
}

impl Invoke for InputAction {
    fn invoke<T>(&self, key: &T)
    where
        T: Key,
    {
        match self {
            InputAction::Press => key.press(),
            InputAction::Release => key.release(),
        }
    }
}

impl Parse for InputAction {
    fn parse(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::press => InputAction::Press,
            Rule::release => InputAction::Release,
            _ => unreachable!(),
        }
    }
}
