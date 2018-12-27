use failure::Error;
use serde_derive::{Deserialize, Serialize};

pub fn run() -> Result<(), Error> {
    println!("\n======== Extract fields to structs ========\n");

    let string = r#"[
        {"op": "Add",  "type": "Miscellaneous", "name": "clean up room"},
        {"op": "Add",  "type": "Technical",     "name": "fix Wi-Fi"},
        {"op": "Take", "type": "Technical"},
        {"op": "Take", "type": "Miscellaneous"}
    ]"#;

    let operations: Vec<Operation> = serde_json::from_str(&string)?;

    println!("String: {}", string);
    println!("Decoded: {:?}", operations);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op")]
enum Operation {
    Add {
        #[serde(flatten)]
        task_type: TaskType,
        #[serde(flatten)]
        task: Task,
    },
    Take(TaskType),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TaskType {
    Technical,
    Miscellaneous,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
}
