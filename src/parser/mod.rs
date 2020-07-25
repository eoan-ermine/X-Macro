use inputbot::KeybdKey::{self, *};
use inputbot::MouseButton::{self, *};

use crate::action::{Orientation, MouseAction, KeyboardAction, Action, Attribute};

pub trait MyFromStr: Sized {
	fn from_str(s: &str) -> Option<Self>;
}

impl MyFromStr for KeybdKey {
	fn from_str(s: &str) -> Option<KeybdKey> {
		match s {
			"BackspaceKey" => Some(BackspaceKey),
			"TabKey" => Some(TabKey),  
			"EnterKey" => Some(EnterKey),
			"EscapeKey" =>  Some(EscapeKey),  
			"SpaceKey" => Some(SpaceKey),  
			"HomeKey" => Some(HomeKey),  
			"LeftKey" => Some(LeftKey),  
			"UpKey" => Some(UpKey),  
			"RightKey" => Some(RightKey),  
			"DownKey" => Some(DownKey),  
			"InsertKey" => Some(InsertKey),  
			"DeleteKey" => Some(DeleteKey),  
			"Numrow0Key" => Some(Numrow0Key),  
			"Numrow1Key" => Some(Numrow1Key),  
			"Numrow2Key" => Some(Numrow2Key),  
			"Numrow3Key" => Some(Numrow3Key),  
			"Numrow4Key" => Some(Numrow4Key),  
			"Numrow5Key" => Some(Numrow5Key),  
			"Numrow6Key" => Some(Numrow6Key),  
			"Numrow7Key" => Some(Numrow7Key),  
			"Numrow8Key" => Some(Numrow8Key),  
			"Numrow9Key" => Some(Numrow9Key),  
			"AKey" => Some(AKey),  
			"BKey" => Some(BKey),  
			"CKey" => Some(CKey),  
			"DKey" => Some(DKey),  
			"EKey" => Some(EKey),  
			"FKey" => Some(FKey),  
			"GKey" => Some(GKey),  
			"HKey" => Some(HKey),  
			"IKey" => Some(IKey),  
			"JKey" => Some(JKey),  
			"KKey" => Some(KKey),  
			"LKey" => Some(LKey),  
			"MKey" => Some(MKey),  
			"NKey" => Some(NKey),  
			"OKey" => Some(OKey),  
			"PKey" => Some(PKey),  
			"QKey" => Some(QKey),  
			"RKey" => Some(RKey),  
			"SKey" => Some(SKey),  
			"TKey" => Some(TKey),  
			"UKey" => Some(UKey),  
			"VKey" => Some(VKey),  
			"WKey" => Some(WKey),  
			"XKey" => Some(XKey),  
			"YKey" => Some(YKey),  
			"ZKey" => Some(ZKey),  
			"Numpad0Key" => Some(Numpad0Key),  
			"Numpad1Key" => Some(Numpad1Key),  
			"Numpad2Key" => Some(Numpad2Key),  
			"Numpad3Key" => Some(Numpad3Key),  
			"Numpad4Key" => Some(Numpad4Key),  
			"Numpad5Key" => Some(Numpad5Key),  
			"Numpad6Key" => Some(Numpad6Key),  
			"Numpad7Key" => Some(Numpad7Key),  
			"Numpad8Key" => Some(Numpad8Key),  
			"Numpad9Key" => Some(Numpad9Key),  
			"F1Key" => Some(F1Key),  
			"F2Key" => Some(F2Key),  
			"F3Key" => Some(F3Key),  
			"F4Key" => Some(F4Key),  
			"F5Key" => Some(F5Key),  
			"F6Key" => Some(F6Key),  
			"F7Key" => Some(F7Key),  
			"F8Key" => Some(F8Key),  
			"F9Key" => Some(F9Key),  
			"F10Key" => Some(F10Key),  
			"F11Key" => Some(F11Key),  
			"F12Key" => Some(F12Key),  
			"NumLockKey" => Some(NumLockKey),  
			"ScrollLockKey" => Some(ScrollLockKey),  
			"CapsLockKey" => Some(CapsLockKey),  
			"LShiftKey" => Some(LShiftKey),  
			"RShiftKey" => Some(RShiftKey),  
			"LControlKey" => Some(LControlKey),  
			"RControlKey" => Some(RControlKey), 
			_ => None,
		}
	}
}

impl MyFromStr for MouseButton {
	fn from_str(s: &str) -> Option<MouseButton> {
		match s {
			"LeftButton" => Some(LeftButton),
			"MiddleButton" => Some(MiddleButton),
			"RightButton" => Some(RightButton),
			"X1Button" => Some(X1Button),
			"X2Button" => Some(X2Button),
			_ => None,
		}
	}
}

impl MyFromStr for Orientation {
	fn from_str(s: &str) -> Option<Orientation> {
		match s {
			"Vertical" => Some(Orientation::Vertical),
			"Horizontal" => Some(Orientation::Horizontal),
			_ => None
		}
	}
}

impl MyFromStr for Attribute {
	fn from_str(s: &str) -> Option<Attribute> {
		match s {
			"WhilePressed" => Some(Attribute::WhilePressed),
			_ => None
		}
	}

}

peg::parser!{
	pub grammar action_parser() for str {
		pub rule number() -> i32 
			= n:$("-"?['0'..='9']+) { n.parse().unwrap() }
		pub rule character() -> char
			= n:$(['a'..='z' | 'A'..='Z']) { n.parse().unwrap() }

		pub rule key() -> KeybdKey 
			= n:$((character() / number())+)? {? match n {
				Some(n) => { 
					if n.ends_with("Key") {
						Ok(KeybdKey::from_str(n).unwrap())
					} else {
						Err("invalid")
					}
				},
				None => Err("invalid"),
			}}
		pub rule button() -> MouseButton
			= n:$((character() / number())+)? {? match n {
				Some(n) => { 
					if n.ends_with("Button") {
						Ok(MouseButton::from_str(n).unwrap())
					} else {
						Err("invalid")
					}
				},
				None => Err("invalid"),
			}}
		pub rule orientation() -> Orientation
			= n:$("Vertical"/"Horizontal") { Orientation::from_str(n).unwrap() }
		pub rule attribute() -> Action
			= "Attribute::" n:$("WhilePressed") { Action::Attribute(Attribute::from_str(n).unwrap()) }

		pub rule keyboard_press() -> Action
			= "Keyboard::Press(" k:key() ")" { Action::Keyboard(KeyboardAction::Press(k)) }
		pub rule keyboard_release() -> Action
			= "Keyboard::Release(" k:key() ")" { Action::Keyboard(KeyboardAction::Release(k)) }
		pub rule keyboard_send() -> Action
			= "Keyboard::Send(" seq:(character()+) ")" { Action::Keyboard(KeyboardAction::Send(inputbot::KeySequence(Box::leak(seq.into_iter().collect::<String>().into_boxed_str())))) }

		pub rule mouse_press() -> Action
			= "Mouse::Press(" btn:button() ")" { Action::Mouse(MouseAction::Press(btn))}
		pub rule mouse_release() -> Action
			= "Mouse::Release(" btn:button() ")" { Action::Mouse(MouseAction::Release(btn))}
		pub rule mouse_scroll() -> Action
			= "Mouse::Scroll(" orientation:orientation() "," delta:number() ")" { Action::Mouse(MouseAction::Scroll(orientation, delta ))}
		pub rule mouse_move() -> Action
			= "Mouse::Move(" x:number() "," y:number() ")" { Action::Mouse(MouseAction::Move(x, y))}

		pub rule keyboard_action() -> Action
			= action:(keyboard_press() / keyboard_release() / keyboard_send()) { action }
		pub rule mouse_action() -> Action
			= action:(mouse_press() / mouse_release() / mouse_scroll() / mouse_move()) { action }

		pub rule action() -> Action
			= action:(keyboard_action() / mouse_action() / attribute()) { action }
	}
}

#[cfg(test)]
mod tests {
	use super::action_parser::*;

	#[test]
	fn test_parse_number() {
		fn test_parse_positive_numbers() {
			assert_eq!(number("5").unwrap(), 5);
			assert_eq!(number("510").unwrap(), 510);
		}

		fn test_parse_negative_numbers() {
			assert_eq!(number("-5").unwrap(), -5);
			assert_eq!(number("-510").unwrap(), -510);
		}

		test_parse_positive_numbers();
		test_parse_negative_numbers();
	}

	#[test]
	fn test_parse_character() {
		fn test_parse_big_character() {
			assert_eq!(character("A").unwrap(), 'A');
			assert_eq!(character("B").unwrap(), 'B');
		}

		fn test_parse_small_character() {
			assert_eq!(character("a").unwrap(), 'a');
			assert_eq!(character("b").unwrap(), 'b');
		}

		test_parse_big_character();
		test_parse_small_character();
	}


	#[test]
	fn test_parse_key() {
		use inputbot::KeybdKey::*;

		assert_eq!(key("BackspaceKey").unwrap(), BackspaceKey);
		assert_eq!(key("Numrow0Key").unwrap(), Numrow0Key);
		assert_eq!(key("AKey").unwrap(), AKey);
		assert_eq!(key("Numpad0Key").unwrap(), Numpad0Key);
		assert_eq!(key("F1Key").unwrap(), F1Key);
		assert_eq!(key("LControlKey").unwrap(), LControlKey);
	}

	#[test]
	#[should_panic]
	fn test_parse_key_panic() {
		key("MyKey").unwrap();
		key("OtherKey").unwrap();
		key("Unicorn").unwrap();
	}

	#[test]
	fn test_parse_button() {
		use inputbot::MouseButton::*;

		assert_eq!(button("LeftButton").unwrap(), LeftButton);
		assert_eq!(button("MiddleButton").unwrap(), MiddleButton);
		assert_eq!(button("RightButton").unwrap(), RightButton);
		assert_eq!(button("X1Button").unwrap(), X1Button);
		assert_eq!(button("X2Button").unwrap(), X2Button);
	}

	#[test]
	#[should_panic]
	fn test_parse_button_panic() {
		button("NotAButton").unwrap();
		button("Unicorn").unwrap();
	}

	#[test]
	fn test_parse_orientation() {
		use crate::parser::Orientation::*;

		assert_eq!(orientation("Vertical").unwrap(), Vertical);
		assert_eq!(orientation("Horizontal").unwrap(), Horizontal);
	}

	#[test]
	#[should_panic]
	fn test_parse_orientation_panic() {
		orientation("NotAnOrientation").unwrap();
		orientation("Unicorn").unwrap();
	}
	#[test]
	fn test_keyboard_press() {
		keyboard_press("Keyboard::Press(BackspaceKey)").unwrap();
		keyboard_press("Keyboard::Press(Numrow0Key)").unwrap();
		keyboard_press("Keyboard::Press(AKey)").unwrap();
		keyboard_press("Keyboard::Press(Numpad0Key)").unwrap();
		keyboard_press("Keyboard::Press(F1Key)").unwrap();
		keyboard_press("Keyboard::Press(LControlKey)").unwrap();
	}

	#[test]
	#[should_panic]
	fn test_keyboard_press_panic() {
		keyboard_press("Keyboard::Press(NotAKey)").unwrap();
		keyboard_press("Mouse::Press(Numrow0Key)").unwrap();
		keyboard_press("Keyboard::Release(AKey)").unwrap();
		keyboard_press("Mouse::Release(Numpad0Key)").unwrap();
		keyboard_press("Unicorn::Press(F1Key)").unwrap();
	}
	#[test]
	fn test_keyboard_release() {
		keyboard_release("Keyboard::Release(BackspaceKey)").unwrap();
		keyboard_release("Keyboard::Release(Numrow0Key)").unwrap();
		keyboard_release("Keyboard::Release(AKey)").unwrap();
		keyboard_release("Keyboard::Release(Numpad0Key)").unwrap();
		keyboard_release("Keyboard::Release(F1Key)").unwrap();
		keyboard_release("Keyboard::Release(LControlKey)").unwrap();
	}

	#[test]
	#[should_panic]
	fn test_keyboard_release_panic() {
		keyboard_release("Keyboard::Press(NotAKey)").unwrap();
		keyboard_release("Mouse::Press(Numrow0Key)").unwrap();
		keyboard_release("Keyboard::Press(AKey)").unwrap();
		keyboard_release("Mouse::Release(Numpad0Key)").unwrap();
		keyboard_release("Unicorn::Press(F1Key)").unwrap();
	}

	#[test]
	fn test_mouse_press() {
		mouse_press("Mouse::Press(LeftButton)").unwrap();
		mouse_press("Mouse::Press(MiddleButton)").unwrap();
		mouse_press("Mouse::Press(RightButton)").unwrap();
		mouse_press("Mouse::Press(X1Button)").unwrap();
		mouse_press("Mouse::Press(X2Button)").unwrap();
	}

	#[test]
	#[should_panic]
	fn test_mouse_press_panic() {
		mouse_press("Mouse::Release(LeftButton)").unwrap();
		mouse_press("Mouse::Press(NotAButton)").unwrap();
		mouse_press("Keyboard::Press(RightButton)").unwrap();
		mouse_press("Keyboard::Release(X1Button)").unwrap();
		mouse_press("Mouse::Unicorn(X2Button)").unwrap();
	}


	#[test]
	fn test_mouse_release() {
		mouse_release("Mouse::Release(LeftButton)").unwrap();
		mouse_release("Mouse::Release(MiddleButton)").unwrap();
		mouse_release("Mouse::Release(RightButton)").unwrap();
		mouse_release("Mouse::Release(X1Button)").unwrap();
		mouse_release("Mouse::Release(X2Button)").unwrap();
	}

	#[test]
	fn test_mouse_scroll() {
		mouse_scroll("Mouse::Scroll(Horizontal,5)").unwrap();
		mouse_scroll("Mouse::Scroll(Vertical,5)").unwrap();
		mouse_scroll("Mouse::Scroll(Horizontal,-5)").unwrap();
		mouse_scroll("Mouse::Scroll(Vertical,-5)").unwrap();

		mouse_scroll("Mouse::Scroll(Vertical,-505)").unwrap();
		mouse_scroll("Mouse::Scroll(Vertical,505)").unwrap();
	}

	#[test]
	fn test_mouse_move() {
		mouse_move("Mouse::Move(10,5)").unwrap();
		mouse_move("Mouse::Move(10,5)").unwrap();

		mouse_move("Mouse::Move(865,524)").unwrap();
		mouse_move("Mouse::Move(865,524)").unwrap();
		mouse_move("Mouse::Move(-865,-524)").unwrap();
		mouse_move("Mouse::Move(-865,-524)").unwrap();
	}

	#[test]
	fn test_mouse_action() {
		mouse_press("Mouse::Press(LeftButton)").unwrap();
		mouse_release("Mouse::Release(LeftButton)").unwrap();
		mouse_scroll("Mouse::Scroll(Horizontal,5)").unwrap();
		mouse_move("Mouse::Move(10,5)").unwrap();
	}

	#[test]
	fn test_keyboard_action() {
		keyboard_press("Keyboard::Press(AKey)").unwrap();
		keyboard_release("Keyboard::Release(AKey)").unwrap();
	}
}
