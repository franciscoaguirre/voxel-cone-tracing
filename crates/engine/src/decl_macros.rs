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

#[macro_export]
macro_rules! pause_systems_with_number_keys {
    ($systems:expr, $event:expr, $($num:literal),*) => {
        paste::paste! {
            $(
                match $event {
                    egui_glfw_gl::glfw::WindowEvent::Key(egui_glfw_gl::glfw::Key::[<Num $num>], _, egui_glfw_gl::glfw::Action::Press, _) => {
                        if $systems[$num].is_paused() {
                            $systems[$num].unpause();
                            println!("System number {} unpaused.", $num);
                        } else {
                            $systems[$num].pause();
                            println!("System number {} paused.", $num);
                        }
                    }
                    _ => {}
                }
            )*
        }
    };
}
