use failure::Error;
use serde_derive::{Deserialize, Serialize};

pub fn run() -> Result<(), Error> {
    println!("\n======== Using other fails with null ========\n");

    let string = r#"[
        {"education": "high school", "name": "Tokyo metropolitan Hibiya High School"},
        {"education": "college", "name": "Kyouritsu Women's Junior College", "speciality": "english literature"},
        {"education": "bachelor", "name": "Meiji University", "speciality": "laws"},
        {"education": "master", "name": "Tokyo Institute of Technology", "speciality": "engineering"},
        {"education": "doctor", "name": "The University of Tokyo", "speciality": "science"},
        {"name": "バカ田大学"}
    ]"#;

    let educations: Vec<Education> = serde_json::from_str(&string)?;

    println!("String: {}", string);
    println!("Decoded: {:?}", educations);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "education")]
enum Education {
    #[serde(rename = "high school")]
    HighSchool { name: String },
    #[serde(rename = "college")]
    College { name: String, speciality: String },
    #[serde(rename = "bachelor")]
    Bachelor { name: String, speciality: String },
    #[serde(rename = "master")]
    Master { name: String, speciality: String },
    #[serde(rename = "doctor")]
    Doctor { name: String, speciality: String },
    #[serde(other)]
    Other,
}
