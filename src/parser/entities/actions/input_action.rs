use pest::iterators::Pair;

use crate::{
    parser::entities::{Parse, Rule},
    KeyExt,
};

use super::Invoke;

#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    Press,
    Release,
}

impl Invoke for InputAction {
    fn invoke<T>(&self, key: &T)
    where
        T: KeyExt,
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

#[cfg(test)]
mod tests {
    use super::InputAction;
    use crate::parser::entities::{GrammarParser, Parse, Rule};
    use pest::Parser;

    #[test]
    fn input_action() {
        assert_eq!(
            InputAction::parse(
                GrammarParser::parse(Rule::input_action, "press")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            InputAction::Press
        );
        assert_eq!(
            InputAction::parse(
                GrammarParser::parse(Rule::input_action, "release")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            InputAction::Release
        );
    }

    #[test]
    fn input_invoke() {
        GrammarParser::parse(Rule::input_action, "press()").unwrap();
        GrammarParser::parse(Rule::input_invoke, "release()").unwrap();
    }
}
