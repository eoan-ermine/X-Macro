use inputbot::{KeybdKey, MouseButton};
use pest::iterators::Pair;

use crate::KeyExt;

use super::{Parse, Rule};

pub mod keybd_key;
pub mod mouse_button;

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Keyboard(KeybdKey),
    Mouse(MouseButton),
}

impl KeyExt for Key {
    fn press(&self) {
        match self {
            Self::Keyboard(e) => e.press(),
            Self::Mouse(e) => e.press()
        }
    }

    fn release(&self) {
        match self {
            Self::Keyboard(e) => e.release(),
            Self::Mouse(e) => e.release()
        }
    }
}

impl Parse for Key {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::keybd_key => Self::Keyboard(KeybdKey::parse(pair)),
            Rule::mouse_button => Self::Mouse(MouseButton::parse(pair)),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::entities::{GrammarParser, Parse, Rule};
    use pest::Parser;

    use super::Key;
    use inputbot::{KeybdKey, MouseButton};

    #[test]
    fn key() {
        assert_eq!(
            Key::parse(
                GrammarParser::parse(Rule::key, "AKey")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Key::Keyboard(KeybdKey::AKey)
        );

        assert_eq!(
            Key::parse(
                GrammarParser::parse(Rule::key, "OtherKey(256)")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Key::Keyboard(KeybdKey::OtherKey(256))
        );

        assert_eq!(
            Key::parse(
                GrammarParser::parse(Rule::key, "MiddleButton")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Key::Mouse(MouseButton::MiddleButton)
        );

        assert_eq!(
            Key::parse(
                GrammarParser::parse(Rule::key, "OtherButton(256)")
                    .unwrap()
                    .next()
                    .unwrap()
            ),
            Key::Mouse(MouseButton::OtherButton(256))
        );
    }
}
