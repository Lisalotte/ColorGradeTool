mod remotecontrol;
mod presetmanager;
mod configmanager;

use std::future::pending;

use remotecontrol::GetRequest;
use crate::colorgrade::{self, ColorComponent};
use rfd;

pub struct ColorGradeApp {
    color_grade: colorgrade::ColorGrade,
    show_presetname_viewport: bool,
    show_path_viewport: bool,
    show_ip_viewport: bool,
    preset_name: String,
    config_name: String,
    object_path: String,
    ip_address: String,
}

impl ColorGradeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };

        let fullscreen = colorgrade::ColorComponent::new(
            "fullscreen",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };

        let scene = colorgrade::ColorComponent::new(
            "scene",
            sat,
            con,
            gam,
            gain
        );

        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: Option::None, g_old: Option::None, b_old: Option::None, a_old: Option::None
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

        let mut object_path_init = String::from("");
        let mut ip_address_init = String::from("");
        
        configmanager::setup("default.json", &mut object_path_init, &mut ip_address_init);
    
        // Instantiate the color_grade struct
        Self {
            color_grade: color_grade_obj,
            show_presetname_viewport: false,
            show_path_viewport: false,
            show_ip_viewport: false,
            config_name: String::from("default"),
            preset_name: String::from("preset"),
            object_path: object_path_init,
            ip_address: ip_address_init,
        }
    }
}

impl eframe::App for ColorGradeApp {
    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {       
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        let mut pending_update = false;

        // For every slider box, check if any values have changed.
        // If so, update all values to UE
        for component  in self.color_grade.components.iter_mut() {

            let iter_array = [&mut component.saturation, &mut component.contrast, &mut component.gamma, &mut component.gain];

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
        
        // Send all values to UE, if a slider values has changed
        if (pending_update) {
            remotecontrol::update_everything(&mut self.color_grade, self.object_path.clone(), self.ip_address.clone()).unwrap();
        }

        //--- Main app ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let request = GetRequest::init(); 
            
            ui.horizontal(|ui| {
                // Menu buttons
                if ui.button("Save Preset").clicked() {
                    self.show_presetname_viewport = true;
                }
                if ui.button("Load Preset").clicked() { 
                    // Open file dialog
                    if let Ok(current_dir) = std::env::current_dir() {
                        if let Some(path) = rfd::FileDialog::new()
                        .set_directory(current_dir)
                        .pick_file() {
                            println!("Path: {}", path.display().to_string());
                            presetmanager::load_preset(&mut self.color_grade, path.display().to_string());
                        }
                    }
                }
                if ui.button("Set Object Path").clicked() {
                    self.show_path_viewport = true;
                }            
                if ui.button("Set Target IP").clicked() {
                    self.show_ip_viewport = true;
                }
            });
            self.color_grade.create_sliderbox(ui);
        });

        // New window for saving a preset
        if self.show_presetname_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("presetname_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Preset Name")
                    .with_inner_size([300.0, 200.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Save preset as:");
                        ui.text_edit_singleline(&mut self.preset_name);
                        if ui.button("Save").clicked() {
                            presetmanager::save_preset(&self.color_grade, &self.preset_name);
                            self.show_presetname_viewport = false;
                        }
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_presetname_viewport = false;
                    }
                },
            );
        }

        // New window for setting the object path
        if self.show_path_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("objectpath_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Object Path")
                    .with_inner_size([600.0, 200.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Object Path:");
                        ui.text_edit_singleline(&mut self.object_path);
                        if ui.button("Save").clicked() {
                            self.show_path_viewport = false;
                        }
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_path_viewport = false;
                    }
                },
            );
        }

        // New window for setting the ip address
        if self.show_ip_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("ip_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("IP Address")
                    .with_inner_size([600.0, 200.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("IP Address:");
                        ui.text_edit_singleline(&mut self.ip_address);
                        if ui.button("Save").clicked() {
                            self.show_ip_viewport = false;
                        }
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_ip_viewport = false;
                    }
                },
            );
        }
    }
}