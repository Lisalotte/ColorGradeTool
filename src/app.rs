mod remotecontrol;
mod presetmanager;
mod configmanager;
mod window_utilities;
mod style;
mod simple_mode;

use std::{ffi::OsStr, path::PathBuf};

use egui::{RichText, Color32, Ui};
use remotecontrol::GetRequest;
use crate::colorgrade::{self};
use rfd;

use self::remotecontrol::{check_connection, check_object_path};

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
    show_popup: bool,
    preset_name: String,
    project_name: String,
    object_path: String,
    ip_address: String,
    connection_ok: bool,
    path_ok: bool,
    button_nr: i32,
    config_name: String,

    pending_update: bool,
    simple_mode: bool,
    preset_edited: bool,
    loading: bool,

    button_config: ButtonConfig,
}

impl ColorGradeApp {

    // Initialize with default values
    fn new_colorgrade() -> (colorgrade::ColorValues, colorgrade::ColorValues, colorgrade::ColorValues, colorgrade::ColorValues) {
        let sat = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: 1.0, g_old: 1.0, b_old: 1.0, a_old: 1.0
        };
        let con = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: 1.0, g_old: 1.0, b_old: 1.0, a_old: 1.0
        };
        let gam = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: 1.0, g_old: 1.0, b_old: 1.0, a_old: 1.0
        };
        let gain = colorgrade::ColorValues{
            r: 1.0, g: 1.0, b: 1.0, a: 1.0, r_old: 1.0, g_old: 1.0, b_old: 1.0, a_old: 1.0
        };

        return (sat, con, gam, gain);
    }

    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {

        let (sat, con, gam, gain) = Self::new_colorgrade();

        let fullscreen = colorgrade::ColorComponent::new(
            "fullscreen",
            sat,
            con,
            gam,
            gain
        );

        let (sat, con, gam, gain) = Self::new_colorgrade();

        let scene = colorgrade::ColorComponent::new(
            "scene",
            sat,
            con,
            gam,
            gain
        );

        let (sat, con, gam, gain) = Self::new_colorgrade();

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
        let mut preset_name_init = String::from("preset");

        // Read default config file
        configmanager::load_config(String::from("config/default.json"), &mut preset_name_init, &mut object_path_init, &mut ip_address_init, &mut project_name_init); 

        // Check connection with UE
        let check_connection = check_connection(object_path_init.clone(), ip_address_init.clone());
        let mut connection_ok = false;

        match check_connection {
            Ok(()) => connection_ok = true,
            Err(e) => { 
                println!("Error: {}", e);
            },        
        };

        // Check path to object in UE
        let mut path_ok = false;
        if connection_ok { 
            let check_path = check_object_path(object_path_init.clone(), ip_address_init.clone());
            match check_path {
                Ok(()) => path_ok = true,
                Err(e) => { 
                    println!("Error: {}", e);
                },        
            };
        }

        // Instantiate the color_grade struct
        Self {
            color_grade: color_grade_obj,
            show_presetname_viewport: false,
            show_path_viewport: false,
            show_ip_viewport: false,
            show_config_button_viewport: false,
            show_config_viewport: false,
            show_popup: false,
            project_name: project_name_init,
            preset_name: preset_name_init,
            object_path: object_path_init,
            ip_address: ip_address_init,
            connection_ok: connection_ok,
            path_ok: path_ok,
            button_nr: 0,
            config_name: String::from("name"),

            pending_update: true,
            simple_mode: true,
            preset_edited: false,
            loading: true,

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

    pub fn update_presetname_from_path(&mut self, path: PathBuf) {
        if path.exists() {
            self.preset_name = String::from(path.file_stem().unwrap().to_str().unwrap());
        }
    }
}

impl eframe::App for ColorGradeApp {

    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {       
       
        // For every slider box, check if any values have changed.
        // If so, update all values to UE
        for component  in self.color_grade.components.iter_mut() {

            let iter_array = [&mut component.saturation, &mut component.contrast, &mut component.gamma, &mut component.gain];

            for color_value in iter_array {
                if color_value.r != color_value.r_old {
                    // Update everything
                    color_value.r_old = color_value.r;
                    self.pending_update = true;
                }
                if color_value.g != color_value.g_old {
                    // Update everything
                    color_value.g_old = color_value.g;
                    self.pending_update = true;
                }
                if color_value.b != color_value.b_old {
                    // Update everything
                    color_value.b_old = color_value.b;
                    self.pending_update = true;
                }
                if color_value.a != color_value.a_old {
                    // Update everything
                    color_value.a_old = color_value.a;
                    self.pending_update = true;
                }
            }
        }

        if !self.simple_mode {
            //--- Top panel ---
            egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(80.0)
            .frame(style::panel_frame(ctx))
            .show(ctx, |ui: &mut egui::Ui| {

                ui.horizontal(|ui| {   
                    ui.set_style(style::bigger_buttons(ctx));

                    if ui.button("Save Config").clicked() {
                        self.show_config_viewport = true;
                    }
                    if ui.button("Load Config").clicked() {
                        // Open file dialog
                        if let Ok(current_dir) = std::env::current_dir() {
                            if let Some(path) = rfd::FileDialog::new()
                            .set_directory(current_dir)
                            .pick_file() {
                                configmanager::load_config(path.display().to_string(), &mut self.preset_name, &mut self.object_path, &mut self.ip_address, &mut self.project_name);
                                presetmanager::load_preset(&mut self.color_grade, format!("presets/{}.json", self.preset_name));
                                self.loading = true;
                            }
                        }
                    }    
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Switch to simple mode").clicked() {
                            self.simple_mode = true;
                        }
                    });

                });
                ui.set_style(style::app_style(ctx));

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Select UE Project").clicked() {
                            // Open file dialog
                            if let Ok(current_dir) = std::env::current_dir() {
                                if let Some(path) = rfd::FileDialog::new()
                                .set_directory(current_dir)
                                .pick_file() {
                                    if path.extension() == Some(OsStr::new("uproject")) {
                                        self.project_name = String::from(path.file_stem().unwrap().to_str().unwrap());
                                    }
                                }
                            }
                        }
                        ui.label(format!("UE Project: {}", self.project_name));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Set Target IP").clicked() {
                            self.show_ip_viewport = true;
                        }
                        if !self.connection_ok {
                            let ip_warning = RichText::new(format!("IP: {} (connection failed)", self.ip_address))
                                .color(Color32::RED);
                            ui.label(ip_warning);

                            // Reconnect
                            if ui.button("Reconnect").clicked() { // Try to reconnect to UE
                                let check_connection = check_connection(self.object_path.clone(), self.ip_address.clone());

                                match check_connection {
                                    Ok(()) => self.connection_ok = true,
                                    Err(e) => { 
                                        println!("Error: {}", e);
                                    },        
                                };

                                if self.connection_ok { 
                                    let check_path = check_object_path(self.object_path.clone(), self.ip_address.clone());
                                    match check_path {
                                        Ok(()) => { 
                                            self.path_ok = true;
                                            self.pending_update = true;
                                        },
                                        Err(_e) => { 
                                            self.path_ok = false
                                        },        
                                    };
                                }
                            }
                            //
                        } else {
                            ui.label(format!("IP: {}", self.ip_address));
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Set Object Path").clicked() {
                            self.show_path_viewport = true;
                        }
                        if self.path_ok {
                            ui.label(format!("Path: {}", self.object_path));
                        }
                        else { 
                            // Reconnect
                            ui.label(RichText::new(format!("Path: {}", self.object_path)).color(Color32::RED));
                            if ui.button("Reconnect").clicked() {
                                let check_path = check_object_path(self.object_path.clone(), self.ip_address.clone());
                                match check_path {
                                    Ok(()) => { 
                                        self.path_ok = true;
                                        self.pending_update = true;
                                    },
                                    Err(_e) => { 
                                        self.path_ok = false;
                                    },        
                                };
                            }
                            //
                        }
                    });  
                });
                
                ui.allocate_space(ui.available_size()); // Fill in extra space with emptiness
            });

            //--- Bottom panel ---
            egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(120.0)
            .frame(style::panel_frame(ctx))
            .show(ctx, |ui| {                
                ui.set_style(style::configure_buttons_style(ctx));

                ui.heading("Configure Preset Buttons");                

                let mut clicked = false;
                configmanager::configure_buttons(ui, ctx, &mut clicked, &mut self.show_config_button_viewport, &mut self.show_popup, &mut self.button_nr); 
                if clicked {
                    self.init_button_config();
                }

                ui.allocate_space(ui.available_size()); // Fill in extra space with emptiness
            });

            //--- Main panel ---
            egui::CentralPanel::default()
            .frame(style::panel_frame(ctx))
            .show(ctx, |ui| {
                let _request = GetRequest::init(); 
                ui.set_style(style::bigger_buttons(ctx));

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
                                presetmanager::load_preset(&mut self.color_grade, path.display().to_string());
                                self.update_presetname_from_path(path);
                                self.preset_edited = false;
                                self.loading = true;
                            }
                        }
                    }
                    Ui::add_space(ui, 10.0);
                    if !self.preset_edited {
                        ui.label(format!("Current preset: {}", self.preset_name));
                    } else {
                        ui.label(format!("Current preset: {} (edited)", self.preset_name));
                    }
                });

                ui.set_style(style::app_style(ctx));
                
                self.color_grade.create_sliderbox(ui);

                ui.allocate_space(ui.available_size()); // Fill in extra space with emptiness
            });
        }
        else { // Simple mode
            egui::CentralPanel::default()
            .frame(style::panel_frame(ctx))
            .show(ctx, |ui| {
                ui.set_style(style::bigger_buttons(ctx));

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Configuration mode").clicked() {
                            self.simple_mode = false;
                        }
                    });
                });

                let mut clicked = false;
                let mut button_clicked = 0;

                ui.set_style(style::simplemode_buttons(ctx));
                simple_mode::buttons(self, ui, ctx, &mut clicked, &mut button_clicked);
                
                if clicked {
                    self.preset_edited = false;
                }
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

        if self.show_popup {
            window_utilities::show_popup(self, ctx, self.button_config.button_nr);
        }

        // Send all values to UE, if a slider value has changed
        if self.pending_update && self.connection_ok && self.path_ok {
            
            // Check if loading a preset initiated the update
            if !self.loading {
                self.preset_edited = true;
            } else {
                self.loading = false;
            }

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

            self.pending_update = false;
        }
    }
}