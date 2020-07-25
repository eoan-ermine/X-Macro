use crate::action::Action;
use crate::parser::action_parser::action;

pub struct Executor { }

impl Executor {
	pub fn new() -> Executor {
		Executor {}
	}
	pub fn execute(code: &str) {
		for line in code.lines() {
			let action = action(line).unwrap();
			action.send();
		}
	}
	pub fn parse(code: &str) -> Vec<Action> {
		code.lines().map(|x| action(x).unwrap()).collect()
	}
}