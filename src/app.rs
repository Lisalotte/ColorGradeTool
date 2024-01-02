mod remotecontrol;
mod presetmanager;
mod configmanager;
mod window_utilities;
mod style;

use std::{thread, time, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use remotecontrol::GetRequest;
use crate::colorgrade::{self};
use rfd;

use self::remotecontrol::{update_everything, check_connection};

pub struct ButtonConfig {
    project_name: String,
    preset_name: String,
    ip_address: String,
    object_path: String,
    button_nr: i32,
}

impl ButtonConfig {
    fn new(project_name: String, preset_name: String, ip_address: String, object_path: String, button_nr: i32) -> Self {
        Self {            
            project_name: String::from(project_name.as_str()),
            preset_name: String::from(preset_name.as_str()),
            ip_address: String::from(ip_address.as_str()),
            object_path: String::from(object_path.as_str()),
            button_nr: button_nr.clone(),
        }
    }
}

pub struct ColorGradeApp {
    color_grade: colorgrade::ColorGrade,
    show_presetname_viewport: bool,
    show_path_viewport: bool,
    show_ip_viewport: bool,
    show_config_button_viewport: bool,
    show_config_viewport: bool,
    preset_name: String,
    project_name: String,
    object_path: String,
    ip_address: String,
    connection_ok: bool,
    button_nr: i32,
    config_name: String,

    button_config: ButtonConfig,
}

impl ColorGradeApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

        let color_grade_obj = colorgrade::ColorGrade::new(
            components
        );

        let mut object_path_init = String::from("");
        let mut ip_address_init = String::from("");
        let mut project_name_init = String::from("");
        
        // Read default config file
        configmanager::load_config(String::from("config/default.json"), &mut object_path_init, &mut ip_address_init, &mut project_name_init); 

        // Instantiate the color_grade struct
        Self {
            color_grade: color_grade_obj,
            show_presetname_viewport: false,
            show_path_viewport: false,
            show_ip_viewport: false,
            show_config_button_viewport: false,
            show_config_viewport: false,
            project_name: project_name_init,
            preset_name: String::from("preset"),
            object_path: object_path_init,
            ip_address: ip_address_init,
            connection_ok: false,
            button_nr: 0,
            config_name: String::from("name"),

            button_config: ButtonConfig::new(String::from("default"), String::from("preset"), String::from("0.0.0.0"), String::from(""), 0),
        }
    }

    pub fn init_button_config(&mut self) {
        self.button_config = ButtonConfig {  
            project_name: String::from(self.project_name.as_str()),
            preset_name: String::from(self.preset_name.as_str()),
            ip_address: String::from(self.ip_address.as_str()),
            object_path: String::from(self.object_path.as_str()),
            button_nr: self.button_nr.clone(),
        };  
    }
}

impl eframe::App for ColorGradeApp {

    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {       
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

        let mut pending_update = false;

        // Check connection
        if !self.connection_ok {
            let check_connection = check_connection(self.object_path.clone(), self.ip_address.clone());

            match check_connection {
                Ok(()) => self.connection_ok = true,
                Err(e) => { 
                    println!("Error: {}", e);
                },        
            };
        };
       
        // For every slider box, check if any values have changed.
        // If so, update all values to UE
        for component  in self.color_grade.components.iter_mut() {

            let iter_array = [&mut component.saturation, &mut component.contrast, &mut component.gamma, &mut component.gain];

            for color_value in iter_array {
                if let Some(r_old) = color_value.r_old {
                    if color_value.r != r_old {
                        // Update everything
                        color_value.r_old = Some(color_value.r);
                        pending_update = true;
                    }
                } else {
                    color_value.r_old = Some(0.0);
                }

                if let Some(g_old) = color_value.g_old {
                    if color_value.g != g_old {
                        // Update everything
                        color_value.g_old = Some(color_value.g);
                        pending_update = true;
                    }
                } else {
                    color_value.g_old = Some(0.0);
                }

                if let Some(b_old) = color_value.b_old {
                    if color_value.b != b_old {
                        // Update everything
                        color_value.b_old = Some(color_value.b);
                        pending_update = true;
                    }
                } else {
                    color_value.b_old = Some(0.0);
                }

                if let Some(a_old) = color_value.a_old {
                    if color_value.a != a_old {
                        // Update everything
                        color_value.a_old = Some(color_value.a);
                        pending_update = true;
                    }
                } else {
                    color_value.a_old = Some(0.0);
                }
            }
        }
        
        // Send all values to UE, if a slider value has changed
        if pending_update && self.connection_ok {
            let update_everything = remotecontrol::update_everything(&mut self.color_grade, self.object_path.clone(), self.ip_address.clone());
            
            match update_everything {
                Ok(()) => {
                    self.connection_ok = true;
                },
                Err(e) => {
                    self.connection_ok = false;
                    println!("Error: {}", e);
                }
            }
        }

        if self.connection_ok {

            //--- Top panel ---
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui: &mut egui::Ui| {
                ui.set_style(style::app_style());

                ui.horizontal(|ui| {     
                    if ui.button("Save Config").clicked() {
                        self.show_config_viewport = true;
                    }
                    if ui.button("Load Config").clicked() {
                        // Open file dialog
                        if let Ok(current_dir) = std::env::current_dir() {
                            if let Some(path) = rfd::FileDialog::new()
                            .set_directory(current_dir)
                            .pick_file() {
                                configmanager::load_config(path.display().to_string(), &mut self.object_path, &mut self.ip_address, &mut self.project_name);
                            }
                        }
                    }    
                });
                ui.vertical(|ui| {
                    ui.label(format!("UE Project: {}", self.project_name));
                    ui.horizontal(|ui| {
                        if ui.button("Set Target IP").clicked() {
                            self.show_ip_viewport = true;
                        }
                        ui.label(format!("IP: {}", self.ip_address));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Set Object Path").clicked() {
                            self.show_path_viewport = true;
                        }
                        ui.label(format!("Path: {}", self.object_path));
                    });  
                });
            });

            //--- Bottom panel ---
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.label("Configure Preset Buttons");

                let mut clicked = false;
                configmanager::configure_buttons(ui, &mut clicked, &mut self.show_config_button_viewport, &mut self.button_nr); 
                if clicked {
                    self.init_button_config();
                }
            });

            //--- Main panel ---
            egui::CentralPanel::default().show(ctx, |ui| {
                let request = GetRequest::init(); 

                    // Menu buttons
                ui.horizontal(|ui| {
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
                });
                
                self.color_grade.create_sliderbox(ui);
            });
        }
        else { //No connection with UE: show error message
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("Trying to establish a connection with UE..."));
            });
        }

        // New window for saving a preset
        if self.show_presetname_viewport {
            window_utilities::show_presetname_viewport(self, ctx);
        }
        
        // New window for setting the object path
        if self.show_path_viewport {
            window_utilities::show_path_viewport(self, ctx);
        }

        // New window for setting the ip address
        if self.show_ip_viewport {
            window_utilities::show_ip_viewport(self, ctx);
        }
        
        // New window for saving a config file
        if self.show_config_viewport {
            window_utilities::show_config_viewport(self, ctx);
        }

        // New window for configuring a preset button
        if self.show_config_button_viewport { 
            window_utilities::show_config_button_viewport(self, ctx);
        }
    }
}