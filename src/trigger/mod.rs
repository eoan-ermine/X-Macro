use inputbot::{KeybdKey, MouseButton};
use crate::parser::MyFromStr;

#[derive(Debug)]
pub enum Event {
	Keyboard(KeybdKey),
	Mouse(MouseButton),
}

#[derive(Debug)]
pub struct Trigger {
	event: Event,
}

impl Trigger {
	pub fn new(event: Event) -> Trigger {
		Trigger {
			event
		}

	}
	pub fn event(&self) -> &Event {
		&self.event
	}

	pub fn parse(trigger: &str) -> Trigger {
		if trigger.ends_with("Key") {
			Trigger::new(Event::Keyboard(KeybdKey::from_str(trigger).unwrap()))
		} else if trigger.ends_with("Button") {
			Trigger::new(Event::Mouse(MouseButton::from_str(trigger).unwrap()))
		} else {
			panic!("Invalid trigger");
		}
	}
}