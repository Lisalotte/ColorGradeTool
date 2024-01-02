use serde_json::json;
use serde_json::Value;
use std::path::PathBuf;

pub fn setup(config_file: &str, object_path: &mut String, ip_address: &mut String, project_name: &mut String) {

    // load config file
    if let Ok(current_dir) = std::env::current_dir() {
        let config_folder = "config";
        let config_path = current_dir.join(config_folder).join(config_file);

        if config_path.exists() {            
            let json_string = std::fs::read_to_string(config_path).expect("Failed to read file");

            let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

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
    
    let mut json_object = json!({
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

pub fn configure_buttons(ui: &mut egui::Ui, clicked: &mut bool, show_config_viewport: &mut bool, button_clicked: &mut i32) {

    // For all files in the config folder
    if let Ok(current_dir) = std::env::current_dir() {
        let config_folder = "config/buttons";

        let paths = std::fs::read_dir(config_folder).unwrap();

        let mut counter = 0;

        for path in paths {
        // Create a button, with a maximum of 10 buttons
            let mut project_name = String::from("");

            let actualpath = path.unwrap().path();

            get_projectname(&actualpath, &mut project_name);

            ui.horizontal(|ui| {
                ui.label(format!("{} - {}", project_name, get_presetname(&actualpath)));
                if ui.button( "Overwrite").clicked() {
                    *button_clicked = counter;
                    *clicked = true;
                    *show_config_viewport = true;
                }
            });

            if counter > 9 {
                break;
            } else {
                counter += 1;
            }
        }
    }

    // Load the preset connected to the first button (which is always default.json - for now)
}