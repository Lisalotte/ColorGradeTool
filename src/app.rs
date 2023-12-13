mod remotecontrol;

use remotecontrol::GetRequest;
use crate::colorgrade;

pub struct ColorGradeApp<'a> {
    color_grade: colorgrade::ColorGrade <'a>,
}

impl<'a> ColorGradeApp<'a> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let request = GetRequest::init(); 
        remotecontrol::send_request(request.get_fullscreen);

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

        let components: [&colorgrade::ColorComponent; 3] = [fullscreen, scene, camera];

        let color_grade_obj = colorgrade::ColorGrade::new(
            components
        );
    
        Self {    
            color_grade: color_grade_obj
        }
        
    }

    pub fn setup() {

    }
}

impl<'a> eframe::App for ColorGradeApp<'a> {
    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {       
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Get values from UE").clicked() {
            }
            self.color_grade.create_sliderbox(ui);
        });
    }
}