use serde_json::json;
use serde_json::Value;

pub fn setup(config_file: &str, object_path: &mut String, ip_address: &mut String) {

    // load config file
    if let Ok(current_dir) = std::env::current_dir() {
        let config_folder = "config";
        let config_path = current_dir.join(config_folder).join(config_file);

        if (config_path.exists()) {            
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
        }
        else {
            println!("Cannot find config file.")
        }
    }
}