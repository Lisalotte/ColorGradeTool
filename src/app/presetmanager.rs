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

fn from_json(color_values: &mut colorgrade::ColorValues, json_object: &Value) {
    
    let r = &json_object["X"];
    color_values.r = serde_json::from_value(r.clone()).unwrap();
    
    let g = &json_object["Y"];
    color_values.g = serde_json::from_value(g.clone()).unwrap();

    let b = &json_object["Z"];
    color_values.b = serde_json::from_value(b.clone()).unwrap();

    let a = &json_object["W"];
    color_values.a = serde_json::from_value(a.clone()).unwrap();

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
    let json_string = serde_json::to_string_pretty(&json_object).expect("Failed to serialize to JSON");

    // Write the JSON string to a file
    std::fs::write("test_preset.json", json_string).expect("Failed to write to file");
}

pub fn load_preset(color_grade: &mut colorgrade::ColorGrade) {

    let json_string = std::fs::read_to_string("test_preset.json").expect("Failed to read file");

    let json_object: Value = serde_json::from_str(&json_string).expect("Failed to deserialize JSON"); 

    for component in color_grade.components.iter_mut() {
        if component.name == "fullscreen" {
            from_json(&mut component.saturation, &json_object["fullscreen"]["saturation"]);
            from_json(&mut component.contrast, &json_object["fullscreen"]["contrast"]);
            from_json(&mut component.gamma, &json_object["fullscreen"]["gamma"]);
            from_json(&mut component.gain, &json_object["fullscreen"]["gain"]);
        }
        else if component.name == "scene" {
            from_json(&mut component.saturation, &json_object["scene"]["saturation"]);
            from_json(&mut component.contrast, &json_object["scene"]["contrast"]);
            from_json(&mut component.gamma, &json_object["scene"]["gamma"]);
            from_json(&mut component.gain, &json_object["scene"]["gain"]);            
        }
        else if component.name == "camera" {
            from_json(&mut component.saturation, &json_object["camera"]["saturation"]);
            from_json(&mut component.contrast, &json_object["camera"]["contrast"]);
            from_json(&mut component.gamma, &json_object["camera"]["gamma"]);
            from_json(&mut component.gain, &json_object["camera"]["gain"]);            
        } else {
            println!("Error loading JSON. No matching name found.");
        }
    }
}