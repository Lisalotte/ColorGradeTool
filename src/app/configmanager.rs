use egui::{RichText, Color32, Context};
use serde_json::json;
use serde_json::Value;
use std::path::PathBuf;

pub fn get_projectname(config_path: &PathBuf, project_name: &mut String) { 
    if config_path.exists() {           
        let json_string = std::fs::read_to_string(config_path).expect("Failed to read file");

        let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

        if let Some(project_name_value) = json_object["project_name"].as_str() {
            *project_name = project_name_value.to_string();
        } else {
            println!("Error: 'project_name' field not found or not a string in the config file.");
        }
    }
}

pub fn get_presetname(config_path: &PathBuf) -> String {
    if config_path.exists() {        
        let json_string = std::fs::read_to_string(config_path).expect("Failed to read file");

        let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

        if let Some(preset_name_value) = json_object["preset"].as_str() {
            return preset_name_value.to_string();
        } else {
            println!("Error: 'preset' field not found or not a string in the config file.");
        }
    }
    return String::from("");
}

pub fn save_config(config_folder: &str, config_file: &str, object_path: &str, preset_name: &str, ip_address: &str, project_name: &str) {
    
    let json_object = json!({
        "project_name" : project_name,
        "preset" : preset_name,
        "path" : object_path,
        "ip" : ip_address
    });

    let json_string = serde_json::to_string_pretty(&json_object).expect("Failed to serialize to JSON");

    if let Ok(current_dir) = std::env::current_dir() {
        // Construct the path to the folder in the current working directory
        let folder_path = current_dir.join(config_folder);

        let folder_path_string = folder_path.to_string_lossy().to_string();

        if !folder_path.exists() {
            // Create the folder if it doesn't exist
            std::fs::create_dir_all(folder_path).unwrap();
        }

        // Write the JSON string to a file
        std::fs::write(format!("{folder_path_string}/{config_file}"), json_string).expect("Failed to write to file");
    }
}

pub fn load_config(path: String, preset_name: &mut String, object_path: &mut String, ip_address: &mut String, project_name: &mut String) {
    // load config file
    if let Ok(current_dir) = std::env::current_dir() {
        let config_path = current_dir.join(path);

        if config_path.exists() {            
            let json_string = std::fs::read_to_string(config_path).expect("Failed to read file");

            let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

            if let Some(object_preset_value) = json_object["preset"].as_str() {
                *preset_name = object_preset_value.to_string();
            } else {
                println!("Error: 'preset' field not found or not a string in the config file.");
            }

            if let Some(object_path_value) = json_object["path"].as_str() {
                *object_path = object_path_value.to_string();
            } else {
                println!("Error: 'path' field not found or not a string in the config file.");
            }

            if let Some(ip_address_value) = json_object["ip"].as_str() {
                *ip_address = ip_address_value.to_string();
            } else {
                println!("Error: 'ip' field not found or not a string in the config file.");
            }

            if let Some(project_name_value) = json_object["project_name"].as_str() {
                *project_name = project_name_value.to_string();
            } else {
                println!("Error: 'project_name' field not found or not a string in the config file.");
            }
        }
        else {
            println!("Cannot find config file.")
        }
    }
}

pub fn configure_buttons(ui: &mut egui::Ui, _ctx: &Context, clicked: &mut bool, show_config_viewport: &mut bool, show_popup: &mut bool, button_clicked: &mut i32) {

    // For all files in the config folder
    if let Ok(current_dir) = std::env::current_dir() {
        let config_folder = "config/buttons";
        
        let config_folder_path = current_dir.join(config_folder);

        if !config_folder_path.is_dir() {
            // Create the folder if it doesn't exist
            std::fs::create_dir_all(config_folder_path).unwrap();
        }

        let paths = std::fs::read_dir(config_folder).unwrap();

        let mut counter = 0;
        let col_max = 2;
        let mut col = 1;

        egui::Grid::new("new_grid").show(ui, |ui| {
            for path in paths {
                // Create a button, with a maximum of 10 buttons
                let mut project_name = String::from("");

                let actualpath = path.unwrap().path();

                get_projectname(&actualpath, &mut project_name);
            
                // let text = RichText::new(
                //     format!("{} - {}", project_name, get_presetname(&actualpath)))
                //     .color(Color32::BLACK)
                //     .background_color(Color32::LIGHT_GRAY);
                let text = format!("{} - {}", project_name, get_presetname(&actualpath));

                ui.horizontal(|ui| {
                    ui.label(text)
                });

                if ui.button( "Overwrite").clicked() {
                    *button_clicked = counter;
                    *clicked = true;
                    *show_config_viewport = true;
                }

                if ui.button("Load").clicked() {
                    // Show warning
                    // if yes, load config
                    *button_clicked = counter;
                    *clicked = true;
                    *show_popup = true;
                                        
                }

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

            // Fill unassigned buttons with placeholders
            let col_max = 2;
            let mut col = 1;

            if counter <= 9 {
                for _i in counter..10 {                
                    ui.horizontal(|ui| {
                        ui.label("Unassigned")
                    });
    
                    if ui.button( "Overwrite").clicked() {
                        *button_clicked = counter;
                        *clicked = true;
                        *show_config_viewport = true;
                    }
                    
                    ui.label(RichText::new(" Load ")
                        .color(Color32::LIGHT_GRAY)
                        .background_color(Color32::DARK_GRAY));

                    ui.label(" ");

                    //TODO: Should add ui.end_row() here!
                    if col == col_max {
                        ui.end_row();
                        col = 1;
                    } else {
                        col += 1;
                    }

                }
            }
        });
    }

    // Load the preset connected to the first button (which is always default.json - for now)
}