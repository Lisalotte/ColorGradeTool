mod colorgrade;

#[derive(Default)]
pub struct ColorGradeApp {
    value: f32,
}

impl ColorGradeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ColorGradeApp {
    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {        
        
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.add(egui::Slider::new(&mut self.value, 0.0..=2.0).text("value"));
        });
    }
}