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

#[macro_export]
macro_rules! handle_increments {
    (
        $name:literal,
        $up:ident,
        $down:ident,
        $fn_name:ident,
        $type:ty,
        $increment:literal,
        $min_value:expr,
        $max_value:expr
    ) => {
        pub fn $fn_name(event: &glfw::WindowEvent, value: &mut $type) {
            match *event {
                glfw::WindowEvent::Key(Key::$up, _, Action::Press, _) => {
                    *value = (*value + $increment).min($max_value);
                    println!("{} is: {}", $name, *value);
                }
                glfw::WindowEvent::Key(Key::$down, _, Action::Press, _) => {
                    if *value != 0 as $type {
                        *value = *value - $increment;
                    }
                    println!("{} is: {}", $name, *value);
                }
                _ => {}
            }
        }
    };
}
