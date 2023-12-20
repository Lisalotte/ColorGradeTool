use serde_json::json;
use serde_json::Value;

use crate::colorgrade;

fn to_json(color_values: &colorgrade::ColorValues) -> Value {
    let json_obj = json!({
        "X": color_values.r,
        "Y": color_values.g,
        "Z": color_values.b,
        "W": color_values.a,
    });

    return json_obj;
}

pub fn save_preset(color_grade: &colorgrade::ColorGrade) {
    // Create a json object
    
    let mut json_object = json!({
        "fullscreen" : {},
        "scene" : {},
        "camera" : {}
    });

    // Write function input to json object
    for component in color_grade.components.iter() {

        let sat_json = to_json(&component.saturation);
        let con_json = to_json(&component.contrast);
        let gam_json = to_json(&component.gamma);
        let gai_json = to_json(&component.gain);

        let component_json = json!({
            "saturation" : sat_json,
            "contrast" : con_json,
            "gamma" : gam_json,
            "gain" : gai_json
        });

        if component.name == "fullscreen" {
            json_object["fullscreen"] = component_json;
        }
        else if component.name == "scene" {
            json_object["scene"] = component_json;
        }
        else if component.name == "camera" {
            json_object["camera"] = component_json;
        } else {
            println!("Error saving JSON. No matching name found.");
        }
    }

    // Serialize the JSON object to a JSON string
    let json_string = serde_json::to_string(&json_object).expect("Failed to serialize to JSON");

    // Write the JSON string to a file
    std::fs::write("test_preset.json", json_string).expect("Failed to write to file");
}