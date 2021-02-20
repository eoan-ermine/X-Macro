use druid::widget::{Flex, TextBox, Button};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

use crate::bind::Bind;
use crate::trigger::Trigger;
use crate::executor::Executor;

use std::thread;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Data, Lens)]
struct State {
	trigger: String,
	actions: String,
	is_listener_spawned: bool,
}

pub fn initialize() {
	let data = State{
		trigger: "".into(),
		actions: "".into(),
		is_listener_spawned: false,
	};

    let main_window = WindowDesc::new(ui_builder)
    	.title("X-Macro")
    	.window_size((500.0, 500.0));

    AppLauncher::with_window(main_window)
        .launch(data).expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<State> {
	let trigger_ch = TextBox::new()
		.with_placeholder("Input trigger key or button")
		.fix_width(500.0)
		.lens(State::trigger);

	let actions_ch = TextBox::multiline()
		.with_placeholder("Write macro")
		.fix_width(500.0)
		.lens(State::actions);

	let button = Button::new("Bind")
		.on_click(|_ctx, data: &mut State, _env| {
			let trigger = data.trigger.clone();
			let mut contents = data.actions.clone();

			let trigger = Trigger::parse(&trigger.trim());
			let actions = Executor::parse(&contents);

			let bind = Bind::new(trigger, actions);
			bind.start();

			if !data.is_listener_spawned {
				thread::spawn(|| {
					inputbot::handle_input_events();
				});
				data.is_listener_spawned = true;
			}
		});

	Flex::column()
		.with_child(trigger_ch).with_spacer(8.0)
		.with_child(actions_ch).with_spacer(8.0)
		.with_child(button).with_spacer(8.0).center()
}