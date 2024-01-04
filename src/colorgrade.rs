use serde_json::json;
use serde_json::Value;

pub struct ColorValues {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    pub r_old: Option<f64>,
    pub g_old: Option<f64>,
    pub b_old: Option<f64>,
    pub a_old: Option<f64>,
}

impl ColorValues {
    pub fn create_sliders(&mut self, ui: &mut egui::Ui, name: &str) {

        ui.vertical(|ui| {            
            ui.label(name);
            //self.add_sliderbox(ui);
            ui.add(egui::Slider::new(&mut self.r, 0.0..=2.0).text("R"));
            ui.add(egui::Slider::new(&mut self.g, 0.0..=2.0).text("G"));
            ui.add(egui::Slider::new(&mut self.b, 0.0..=2.0).text("B"));
            ui.add(egui::Slider::new(&mut self.a, 0.0..=2.0).text("A"));
        });
    }

    pub fn to_json(&self) -> Value {
        let result = json!({
            "X": self.r,
            "Y": self.g,
            "Z": self.b,
            "W": self.a
        });

        return result;
    }
}

pub struct ColorComponent {
    pub name: String,
    pub saturation: ColorValues,
    pub contrast: ColorValues,
    pub gamma: ColorValues,
    pub gain: ColorValues,
}

impl ColorComponent {
    pub fn new(name_in: &str, sat: ColorValues, con: ColorValues, gam: ColorValues, gai: ColorValues) -> Self {
        Self {
            name: String::from(name_in),
            saturation: sat,
            contrast: con,
            gamma: gam,
            gain: gai
        }
    }
}

pub struct ColorGrade {
    pub components: [ColorComponent; 3]
}

impl ColorGrade {
    pub fn new(components_in: [ColorComponent; 3]) -> Self {
        Self {
            components: components_in
        }
    }

    fn capitalize(s: &str) -> String {
        if let Some(mut chars) = s.chars().next() {
            chars.make_ascii_uppercase();
            let mut result = chars.to_string();
            result.push_str(&s[1..]);
            return result;
        } else {
            return s.to_string() // Empty string
        }
    }

    pub fn create_sliderbox(&mut self, ui: &mut egui::Ui) {
        for comp in self.components.iter_mut() {
            
            ui.heading(Self::capitalize(&comp.name));
            ui.horizontal(|ui| {
                comp.saturation.create_sliders(ui, "Saturation");
                comp.contrast.create_sliders(ui, "Contrast");
                comp.gamma.create_sliders(ui, "Gamma");
                comp.gain.create_sliders(ui, "Gain");
            });
            
        }
    }
}