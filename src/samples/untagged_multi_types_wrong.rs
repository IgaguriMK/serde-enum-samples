use failure::Error;
use serde_derive::{Deserialize, Serialize};

pub fn run() -> Result<(), Error> {
    println!("\n======== Untagged enum with multiple types ========\n");

    let string = r#"[1.0, 42, "foo"]"#;

    let values: Vec<Values> = serde_json::from_str(&string)?;

    println!("String: {}", string);
    println!("Decoded: {:?}", values);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Values {
    Float(f64),
    Int(i64),
    Str(String),
}
