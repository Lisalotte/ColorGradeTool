use reqwest;
use reqwest::Response;
use serde_json::json;
use serde_json::Value;

use crate::colorgrade;

pub struct GetRequest {
    pub get_fullscreen: Value,
    pub get_scene: Value,
    pub get_camera: Value,
}

impl GetRequest {
    pub fn init() -> Self {
        let path = String::from("/Game/Maps/UEDPIE_0_ConcreteWorld.ConcreteWorld:PersistentLevel.ColorGrading_C_1");
        Self {
            get_fullscreen: json!({
                "objectPath": path,
                "functionName": "ReadFullscreen",
                "generateTransaction": "True"
            }),
            get_scene: json!({
                "objectPath": path,
                "functionName": "ReadScene",
                "generateTransaction": "True"
            }),
            
            get_camera: json!({
                "objectPath": path,
                "functionName": "ReadCamera",
                "generateTransaction": "True"
            })
        }
    }
}

pub fn check_connection() -> Result<(), reqwest::Error> {
    let url_call = "http://127.0.0.1:30010/";

    let url = url_call.to_owned() + "remote/object/call";

    let client = reqwest::blocking::Client::new();
    let response = client.put(url)
        .json("test")
        .send()?;

    Ok(())
}

pub fn update_everything(color_grade: &mut colorgrade::ColorGrade, path: String, ip: String) -> Result<(), reqwest::Error>{

    let url = ip.to_owned() + "remote/object/call";

    let client = reqwest::blocking::Client::new();

    let mut items = json!({});

    for (i, component) in color_grade.components.iter().enumerate() {
        let prefix = match i {
            0 => "fullscreen",
            1 => "scene",
            2 => "camera",
            _ => unreachable!(),
        };

        items[format!("sat_{}", prefix)] = component.saturation.to_json();
        items[format!("con_{}", prefix)] = component.contrast.to_json();
        items[format!("gam_{}", prefix)] = component.gamma.to_json();
        items[format!("gain_{}", prefix)] = component.gain.to_json();
    }

    let request = json!({
        "objectPath": path,
        "functionName": "UpdateEverything",
        "parameters": items,
        "generateTransaction": "True"
    });

    let response= client.put(url)
        .json(&request)
        .send()?;

    Ok(())
}