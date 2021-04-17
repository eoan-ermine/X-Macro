use std::{thread, time::Duration};

use pest::iterators::Pair;

use crate::{
    parser::entities::{Parse, Rule},
    KeyExt,
};

use super::Invoke;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum OtherAction {
    Wait(WaitAction),
}

impl Invoke for OtherAction {
    fn invoke<T>(&self, key: &T)
    where
        T: KeyExt,
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

#[cfg(test)]
mod tests {
    use pest::Parser;
    use crate::parser::entities::{GrammarParser, Rule, Parse};
    use super::{OtherAction, WaitAction};

    use std::time::Duration;

    #[test]
    fn wait_invoke() {
        assert_eq!(
            WaitAction::parse(GrammarParser::parse(Rule::wait_invoke, "wait(500ms)").unwrap().next().unwrap()),
            WaitAction {
                duration: Duration::from_millis(500)
            }
        );
        assert_eq!(
            WaitAction::parse(GrammarParser::parse(Rule::wait_invoke, "wait(1.25s)").unwrap().next().unwrap()),
            WaitAction {
                duration: Duration::from_secs_f64(1.25_f64)
            }
        );
    }

    #[test]
    fn other_invoke() {
        assert_eq!(
            OtherAction::parse(GrammarParser::parse(Rule::other_invoke, "wait(100s)").unwrap().next().unwrap()),
            OtherAction::Wait(
                WaitAction {
                    duration: Duration::from_secs(100)
                }
            )
        )
    }
}
