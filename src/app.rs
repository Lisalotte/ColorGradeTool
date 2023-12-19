mod remotecontrol;

use std::future::pending;

use remotecontrol::GetRequest;
use crate::colorgrade::{self, ColorComponent};

use self::remotecontrol::update_everything;

pub struct ColorGradeApp {
    color_grade: colorgrade::ColorGrade,
}

impl ColorGradeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };

        let fullscreen = colorgrade::ColorComponent::new(
            "fullscreen",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };

        let scene = colorgrade::ColorComponent::new(
            "scene",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, sliderbox: Option::None, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };

        let camera = colorgrade::ColorComponent::new(
            "camera",
            sat,
            con,
            gam,
            gain
        );

        let components: [colorgrade::ColorComponent; 3] = [fullscreen, scene, camera];

        let mut color_grade_obj = colorgrade::ColorGrade::new(
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

        let mut pending_update = false;

        // for every slider box
        for component  in self.color_grade.components.iter_mut() {
            // let prefix = match i {
            //     0 => "fullscreen",
            //     1 => "scene",
            //     2 => "camera",
            //     _ => unreachable!(),
            // };

            let mut iter_array = [&mut component.saturation, &mut component.contrast, &mut component.gamma, &mut component.gain];

            for color_value in iter_array {
                if let Some(r_old) = color_value.r_old {
                    if (color_value.r != r_old) {
                        // Update everything
                        color_value.r_old = Some(color_value.r);
                        pending_update = true;
                    }
                } else {
                    color_value.r_old = Some(0.0);
                }

                if let Some(g_old) = color_value.g_old {
                    if (color_value.g != g_old) {
                        // Update everything
                        color_value.g_old = Some(color_value.g);
                        pending_update = true;
                    }
                } else {
                    color_value.g_old = Some(0.0);
                }

                if let Some(b_old) = color_value.b_old {
                    if (color_value.b != b_old) {
                        // Update everything
                        color_value.b_old = Some(color_value.b);
                        pending_update = true;
                    }
                } else {
                    color_value.b_old = Some(0.0);
                }

                if let Some(a_old) = color_value.a_old {
                    if (color_value.a != a_old) {
                        // Update everything
                        color_value.a_old = Some(color_value.a);
                        pending_update = true;
                    }
                } else {
                    color_value.a_old = Some(0.0);
                }
            }
        }
        
        if (pending_update) {
            remotecontrol::update_everything(&mut self.color_grade).unwrap();
        }
        // check if value has changed

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