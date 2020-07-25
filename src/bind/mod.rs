
use crate::action::{self, Action};
use crate::trigger::{Event, Trigger};
use std::{thread::sleep, time::Duration};

pub struct Bind {
	trigger: Trigger,
	actions: Vec<Action>,
}

impl Bind {
	pub fn new(trigger: Trigger, actions: Vec<Action>) -> &'static mut Bind {
		Box::leak(Box::new(
			Bind {
				trigger,
				actions,
			}
		))
	}
	pub fn start(&'static self) {
		match self.trigger.event() {
			Event::Keyboard(key) => {
				key.bind(move || {
					if let Action::Attribute(attr) = &self.actions[0] {
						if let action::Attribute::WhilePressed = attr {
							while key.is_pressed() {
								for action in &self.actions[1..] {
									action.send();
            						sleep(Duration::from_millis(50));
								}
							}
						}
					} else {
						for action in &self.actions {
							action.send();
						}
					}
				});
			},
			Event::Mouse(button) => {
				button.bind(move || {
					if let Action::Attribute(attr) = &self.actions[0] {
						if let action::Attribute::WhilePressed = attr {
							while button.is_pressed() {
								for action in &self.actions[1..] {
									action.send();
            						sleep(Duration::from_millis(50));
								}
							}
						}
					} else {
						for action in &self.actions {
							action.send();
						}
					}
				});
			}
		}
	}
}
