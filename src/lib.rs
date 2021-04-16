pub mod parser;

use inputbot::{KeybdKey, MouseButton};
use parser::entities::actions::{input_action::InputAction, other_action::OtherAction, Action};

#[macro_use]
extern crate pest_derive;

pub trait Key {
    fn press(&self);
    fn release(&self);
}

impl Key for KeybdKey {
    fn press(&self) {
        KeybdKey::press(*self);
    }

    fn release(&self) {
        KeybdKey::release(*self);
    }
}

impl Key for MouseButton {
    fn press(&self) {
        MouseButton::press(*self);
    }

    fn release(&self) {
        MouseButton::release(*self);
    }
}

pub struct Instruction<T>
where
    T: Key,
{
    key: T,
    actions: Vec<Action>,
}

impl<T> Instruction<T>
where
    T: Key,
{
    pub fn execute(&self) {
        for action in &self.actions {
            match action {
                Action::Input(action) => match action {
                    InputAction::Press => {
                        self.key.press();
                    }
                    InputAction::Release => {
                        self.key.release();
                    }
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
