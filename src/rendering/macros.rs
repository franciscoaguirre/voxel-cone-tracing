#[macro_export]
macro_rules! toggle_boolean {
    ($key:ident, $fn_name:ident) => {
        pub fn $fn_name(event: &glfw::WindowEvent, value: &mut bool) {
            match *event {
                glfw::WindowEvent::Key(Key::$key, _, Action::Press, _) => {
                    *value = !*value;
                }
                _ => {}
            }
        }
    };
}
