use inputbot::{KeybdKey, MouseButton};
use pest::iterators::Pair;

use super::{Parse, Rule};

pub mod keybd_key;
pub mod mouse_button;

pub enum Key {
    Keyboard(KeybdKey),
    Mouse(MouseButton),
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
