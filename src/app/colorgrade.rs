pub struct ColorValues {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub struct ColorComponent<'b> {
    pub name: &'b str,
    pub saturation: ColorValues,
    pub contrast: ColorValues,
    pub gamma: ColorValues,
    pub gain: ColorValues,
}

pub struct ColorGrade<'a> {
    pub components: [&'a mut ColorComponent<'a>; 3],
    pub function_name: String,    
    pub level_path: String,
}

impl<'a> ColorGrade<'a> {
    pub fn new(components_in: [&'a mut ColorComponent<'a>; 3], function_name_in: &str, level_path_in: &str) -> Self {
        Self {
            components: components_in,
            function_name: String::from(function_name_in),
            level_path: String::from(level_path_in)
        }
    }

    fn update_everything() {

    }

    pub fn create_sliderbox(&mut self, ui: &mut egui::Ui) {
        for comp in self.components.iter_mut() {
            ui.label(comp.name);
            ui.horizontal(|ui| {
                comp.saturation.create_sliders(ui);
                comp.contrast.create_sliders(ui);
                comp.gamma.create_sliders(ui);
                comp.gain.create_sliders(ui);
            });
        }
    }
}

impl<'b> ColorComponent<'b> {
    pub fn new(name_in: &'b str, sat: ColorValues, con: ColorValues, gam: ColorValues, gai: ColorValues) -> Self {
        Self {
            name: name_in,
            saturation: sat,
            contrast: con,
            gamma: gam,
            gain: gai
        }
    }
}

impl ColorValues {
    pub fn create_sliders(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add(egui::Slider::new(&mut self.r, 0.0..=2.0).text("R"));
            ui.add(egui::Slider::new(&mut self.g, 0.0..=2.0).text("G"));
            ui.add(egui::Slider::new(&mut self.b, 0.0..=2.0).text("B"));
            ui.add(egui::Slider::new(&mut self.a, 0.0..=2.0).text("A"));
        });
    }
}