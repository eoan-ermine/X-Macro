use std::{thread, time::Duration};

use pest::iterators::Pair;

use crate::{
    parser::entities::{Parse, Rule},
    Key,
};

use super::Invoke;

pub struct WaitAction {
    pub duration: Duration,
}

impl Invoke for WaitAction {
    fn invoke<T>(&self, _: &T) {
        thread::sleep(self.duration);
    }
}

impl Parse for WaitAction {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();

        let dur = inner.next().unwrap().as_str().parse::<f64>().unwrap();
        let unit = inner.next().unwrap();

        let duration = match unit.as_rule() {
            Rule::ms => Duration::from_millis(dur as u64),
            Rule::s => Duration::from_secs_f64(dur),
            _ => unreachable!(),
        };

        Self { duration }
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

impl Parse for OtherAction {
    fn parse(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::wait_invoke => OtherAction::Wait(WaitAction::parse(inner)),
            _ => unreachable!(),
        }
    }
}
