use inputbot::KeySequence;
use inputbot::MouseWheel;
use inputbot::MouseCursor;
use inputbot::MouseButton;
use inputbot::KeybdKey;
//use inputbot::KeySequence;

pub enum Attribute {
	WhilePressed,
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
	Horizontal,
	Vertical,
}

#[derive(Debug)]
pub enum MouseAction {
	Press(MouseButton),
	Release(MouseButton),

	Move(i32, i32),
	Scroll(Orientation, i32)
}

pub enum KeyboardAction {
	Press(KeybdKey),
	Release(KeybdKey),
	Send(KeySequence),
}

pub enum Action {
	Keyboard(KeyboardAction),
	Mouse(MouseAction),
	Attribute(Attribute),
}

impl Action {
	pub fn send(&self) {
		match self {
			Action::Keyboard(action) => {
				match action {
					KeyboardAction::Press(button) => button.press(),
					KeyboardAction::Release(button) => button.release(),
					KeyboardAction::Send(sequence) => sequence.send(),
				};
			},
			Action::Mouse(action) => {
				match action {
					MouseAction::Press(button) => button.press(),
					MouseAction::Release(button) => button.release(),
					MouseAction::Move(x, y) => {
						MouseCursor::move_abs(*x, -*y);
					},
					MouseAction::Scroll(orientation, delta) => {
						if let Orientation::Vertical = orientation {
							MouseWheel::scroll_ver(*delta);
						} else {
							MouseWheel::scroll_hor(*delta);
						}
					}
				}
			},
			_ => {}
		}
	}
}