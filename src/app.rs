mod remotecontrol;

use remotecontrol::GetRequest;
use crate::colorgrade;

pub struct ColorGradeApp {
    color_grade: colorgrade::ColorGrade,
}

impl ColorGradeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };

        let fullscreen = colorgrade::ColorComponent::new(
            "fullscreen",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };

        let scene = colorgrade::ColorComponent::new(
            "scene",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None
        };

        let camera = colorgrade::ColorComponent::new(
            "camera",
            sat,
            con,
            gam,
            gain
        );

        let components: [colorgrade::ColorComponent; 3] = [fullscreen, scene, camera];

        let color_grade_obj = colorgrade::ColorGrade::new(
            components
        );
    
        // Instantiate the color_grade struct
        Self {
            color_grade: color_grade_obj
        }
        
    }

    pub fn setup() {

    }
}

impl eframe::App for ColorGradeApp {
    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {       
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        egui::CentralPanel::default().show(ctx, |ui| {
            let request = GetRequest::init(); 
            if ui.button("Get values from UE").clicked() {
                remotecontrol::send_request(request.get_fullscreen).unwrap();
            }
            if ui.button("Update to UE").clicked() {
                remotecontrol::update_everything(&mut self.color_grade).unwrap();
            }
            self.color_grade.create_sliderbox(ui);
        });
    }
}