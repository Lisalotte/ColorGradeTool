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
        
        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };

        let fullscreen = &mut colorgrade::ColorComponent::new(
            "fullscreen",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };

        let scene = &mut colorgrade::ColorComponent::new(
            "scene",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0
        };

        let camera = &mut colorgrade::ColorComponent::new(
            "camera",
            sat,
            con,
            gam,
            gain
        );

        let components: [&mut colorgrade::ColorComponent; 3] = [fullscreen, scene, camera];

        let mut color_grade = colorgrade::ColorGrade::new(
            components,
            "UpdateEverything",
            "path"
        );

        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.add(egui::Slider::new(&mut self.value, 0.0..=2.0).text("value"));

            color_grade.create_sliders(ui);
        });
    }
}