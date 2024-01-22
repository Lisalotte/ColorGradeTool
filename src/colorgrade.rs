use serde_json::json;
use serde_json::Value;

pub struct ColorValues {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    pub r_old: f64,
    pub g_old: f64,
    pub b_old: f64,
    pub a_old: f64,
}

impl ColorValues {
    pub fn create_sliders(&mut self, ui: &mut egui::Ui, name: &str, step: f64, minval: f64, maxval: f64) {

        ui.vertical(|ui| {            
            ui.label(name);
            //self.add_sliderbox(ui);

            ui.horizontal(|ui| {
                ui.label("R");
                if ui.button("-").clicked() {
                    if self.r >= minval {
                        self.r -= step;
                    }
                }
                ui.add(egui::Slider::new(&mut self.r, minval..=maxval));
                if ui.button("+").clicked() {
                    if self.r <= maxval {
                        self.r += step;
                    }
                }
                ui.label(" ");
            });

            ui.horizontal(|ui| {
                ui.label("G");
                if ui.button("-").clicked() {
                    if self.g >= minval {
                        self.g -= step;
                    }
                }            
                ui.add(egui::Slider::new(&mut self.g, minval..=maxval));
                if ui.button("+").clicked() {
                    if self.g <= maxval {
                        self.g += step;
                    }
                }
                ui.label(" ");
            });

            ui.horizontal(|ui| {
                ui.label("B");
                if ui.button("-").clicked() {
                    if self.b >= minval {
                        self.b -= step;
                    }
                }
                ui.add(egui::Slider::new(&mut self.b, minval..=maxval));
                if ui.button("+").clicked() {
                    if self.b <= maxval {
                        self.b += step;
                    }
                }
                ui.label(" ");
            });

            ui.horizontal(|ui| {
                ui.label("A");
                if ui.button("-").clicked() {
                    if self.a >= minval {
                        self.a -= step;
                    }
                }
                ui.add(egui::Slider::new(&mut self.a, minval..=maxval));
                if ui.button("+").clicked() {
                    if self.a <= maxval {
                        self.a += step;
                    }
                }
            });
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
    pub offset: ColorValues,
}

impl ColorComponent {
    pub fn new(name_in: &str, sat: ColorValues, con: ColorValues, gam: ColorValues, gai: ColorValues, off: ColorValues) -> Self {
        Self {
            name: String::from(name_in),
            saturation: sat,
            contrast: con,
            gamma: gam,
            gain: gai,
            offset: off,
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
            
            let step = 0.05;
            let minval = 0.0 + step;
            let maxval = 2.0 - step;

            ui.heading(Self::capitalize(&comp.name));
            ui.horizontal(|ui| {
                comp.saturation.create_sliders(ui, "Saturation", step, minval, maxval);
                comp.contrast.create_sliders(ui, "Contrast", step, minval, maxval);
                comp.gamma.create_sliders(ui, "Gamma", step, minval, maxval);
                comp.gain.create_sliders(ui, "Gain", step, minval, maxval);
                comp.offset.create_sliders(ui, "Offset", 0.01, -1.0, 1.0);
            });
            
        }
    }
}