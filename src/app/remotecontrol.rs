use reqwest;
use serde_json::json;
use serde_json::Value;

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

pub fn send_request(request: Value) -> Result<(), reqwest::Error> {
    let url_call = "http://127.0.0.1:30010/";

    let url = url_call.to_owned() + "remote/object/call";

    // let client = reqwest::Client::new(); TODO: async
    let client = reqwest::blocking::Client::new();
    let response = client.put(url)
        .json(&request)
        .send()?;

    let body = response.text()?;
    println!("Response: {}", body);

    Ok(())
}