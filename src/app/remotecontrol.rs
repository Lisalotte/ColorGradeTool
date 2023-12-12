use reqwest;
use serde_json::json;

pub fn send_request() -> Result<(), reqwest::Error> {
    let url_call = "http://127.0.0.1:30010/";

    let url = url_call.to_owned() + "remote/object/call";

    let path = "/Game/Maps/UEDPIE_0_ConcreteWorld.ConcreteWorld:PersistentLevel.ColorGrading_C_1";

    let request = json!({
        "objectPath": path,
        "functionName": "ReadFullscreen",
        "generateTransaction": "True"
    });

    // let client = reqwest::Client::new(); TODO: async
    let client = reqwest::blocking::Client::new();
    let response = client.put(url)
        .json(&request)
        .send()?;

    let body = response.text()?;
    println!("Response: {}", body);

    Ok(())
}