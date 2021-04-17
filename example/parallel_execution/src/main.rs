use x_macro::Executor;

fn main() {
    let program = "HKey.press()\nEKey.wait(1s).press()\nLKey.wait(2s).press()\nLKey.wait(3s).press()\nOKey.wait(4ms).press()";
    Executor::execute(program);
}
