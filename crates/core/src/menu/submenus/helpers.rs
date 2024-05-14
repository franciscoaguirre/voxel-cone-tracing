macro_rules! cone_parameters_inputs {
    ( $self:expr, $ui:expr, $( $menu_name:literal: $cone_parameters:ident ),*$(,)? ) => {
        $(
            $ui.label($menu_name);
            $ui.horizontal(|ui| {
                ui.label("Aperture (degrees):");
                ui.add(
                    egui::Slider::new(&mut $self.output.$cone_parameters.cone_angle_in_degrees, 1.0..=90.0),
                );
                ui.label("Max distance:");
                ui.add(
                    egui::Slider::new(&mut $self.output.$cone_parameters.max_distance, 0.1..=1.0),
                );
            });
        )*
    };
}

pub(crate) use cone_parameters_inputs;
