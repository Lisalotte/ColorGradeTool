use std::fs::create_dir_all;
use std::path::PathBuf;
use std::str::FromStr;

use crate::app::configmanager;
use egui::Context;
use crate::app::presetmanager;
use super::ColorGradeApp;

pub fn buttons(app: &mut ColorGradeApp, ui: &mut egui::Ui, _ctx: &Context, clicked: &mut bool, _button_clicked: &mut i32) {

    // For all files in the config folder
    if let Ok(current_dir) = std::env::current_dir() {

        // Config dir
        let config_folder = "config/buttons";

        if !PathBuf::from_str(config_folder).unwrap().exists() {
            create_dir_all(config_folder).unwrap();
        }

        // Presets dir
        let mut presetdir = current_dir.join("presets");

        if !presetdir.is_dir() {
            ui.label("Preset directory could not be found.");
            return;
        }

        let paths = std::fs::read_dir(config_folder).unwrap();
 
        let mut counter = 0;
        let col_max = 2;
        let mut col = 1;

        ui.label(format!("Current: {}", app.preset_name));

        egui::Grid::new("new_grid").show(ui, |ui| {
            for path in paths {

                let buttonconfigdir = path.unwrap().path();
                let filename = buttonconfigdir.file_stem().unwrap().to_str().unwrap();

                // Filename should follow the pattern: button<#>.json
                if filename.starts_with("button") && filename.len() <= 7 {

                    // Project name                
                    let mut project_name = String::from("");
                    configmanager::get_projectname(&buttonconfigdir, &mut project_name);

                    // Preset name
                    let preset_name = configmanager::get_presetname(&buttonconfigdir);
                
                    let mut text = format!("{} - {}", project_name, preset_name);

                    // Path to preset
                    let presetpath = format!("{}/{}.json", presetdir.as_mut_os_string().to_str().unwrap(), preset_name);
           
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.horizontal(|ui| {
                            ui.set_max_width(400.0);
                            let pathbuf = PathBuf::from_str(presetpath.as_str()).unwrap();
                            if !pathbuf.exists() {
                                text = String::from("Preset couldn't be found");
                            }
                            let button = egui::Button::new(text).wrap(true);
                            if ui.add(button).clicked() {
                                presetmanager::load_preset(&mut app.color_grade, presetpath.clone());
                                app.loading = true;
                                *clicked = true;
                            }
                        });
                    });

                    ui.label(" ");

                    if col == col_max {
                        ui.end_row();
                        col = 1;
                    } else {
                        col += 1;
                    }

                    if counter > 9 {
                        break;
                    } else {
                        counter += 1;
                    }
                }
            }
        });
    }
}