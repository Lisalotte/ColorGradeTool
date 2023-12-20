use serde_json::json;
use serde_json::Value;

pub fn setup(config_file: &str, object_path: &mut String, ip_address: &mut String) {

    // load config file
    if let Ok(current_dir) = std::env::current_dir() {
        let config_folder = "config";
        let config_path = current_dir.join(config_folder);

        if (config_path.exists()) {            
            let json_string = std::fs::read_to_string(config_path).expect("Failed to read file");

            let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

            object_path = serde_json::from_value(json_object["path"].clone()).unwrap();
            ip_address = json_object["ip"];
        }
        else {
            println!("Cannot find config file.")
        }
    }
}