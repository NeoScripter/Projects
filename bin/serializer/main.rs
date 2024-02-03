use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;

#[derive(Serialize, Deserialize, Debug)]
struct CustomStruct {
    id: i32,
    name: String,
    active: bool,
}

impl CustomStruct {
    fn new() -> Self {
        Self { id: 3, name: "Ilya".to_string(), active: true }
    }
}

fn serialize(data: &CustomStruct) -> JsonResult<String> {
    serde_json::to_string(data)
}

fn main() {
    let data = CustomStruct::new();

    match serialize(&data) {
        Ok(serde_data) => println!("Success {}", serde_data),
        Err(e) => println!("Error occured {:?}", e),
    }
}