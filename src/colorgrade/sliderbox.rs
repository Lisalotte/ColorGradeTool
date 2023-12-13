
pub struct SliderBox {
    pub r: egui::Response,
    pub g: egui::Response,
    pub b: egui::Response,
    pub a: egui::Response
}

impl SliderBox {
    pub fn new(mut r: f64, mut g: f64, mut b: f64, mut a: f64, ui: &mut egui::Ui) -> Self {
        Self {            
            r: ui.add(egui::Slider::new(&mut r, 0.0..=2.0).text("R")),
            g: ui.add(egui::Slider::new(&mut g, 0.0..=2.0).text("G")),
            b: ui.add(egui::Slider::new(&mut b, 0.0..=2.0).text("B")),
            a: ui.add(egui::Slider::new(&mut a, 0.0..=2.0).text("A")),
        }
    }
}