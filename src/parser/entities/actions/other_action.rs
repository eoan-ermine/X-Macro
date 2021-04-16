use std::{thread, time::Duration};

use crate::Key;

use super::Invoke;

pub struct WaitAction {
    pub duration: Duration,
}

impl Invoke for WaitAction {
    fn invoke<T>(&self, _: &T) {
        thread::sleep(self.duration);
    }
}

pub enum OtherAction {
    Wait(WaitAction),
}

impl Invoke for OtherAction {
    fn invoke<T>(&self, key: &T)
    where
        T: Key,
    {
        match self {
            OtherAction::Wait(e) => e.invoke(key),
        }
    }
}
