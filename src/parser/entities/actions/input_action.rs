use crate::Key;

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
