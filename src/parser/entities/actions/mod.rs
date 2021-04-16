use crate::{InputAction, Key, OtherAction};

pub mod input_action;
pub mod other_action;

pub enum Action {
    Input(InputAction),
    Other(OtherAction),
}

pub trait Invoke {
    fn invoke<T>(&self, key: &T)
    where
        T: Key;
}

impl Invoke for Action {
    fn invoke<T>(&self, key: &T)
    where
        T: Key {
        match self {
            Action::Input(e) => e.invoke(key),
            Action::Other(e) => e.invoke(key),
        }
    }
}