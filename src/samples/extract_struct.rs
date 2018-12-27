use failure::Error;
use serde_derive::{Deserialize, Serialize};

pub fn run() -> Result<(), Error> {
    println!("\n======== Extract fields to struct ========\n");

    let string = r#"[
        {"education": "high school", "name": "Tokyo metropolitan Hibiya High School"},
        {"education": "college", "name": "Kyouritsu Women's Junior College", "speciality": "english literature"},
        {"education": "bachelor", "name": "Meiji University", "speciality": "laws"},
        {"education": "master", "name": "Tokyo Institute of Technology", "speciality": "engineering"},
        {"education": "doctor", "name": "The University of Tokyo", "speciality": "science"},
        {"education": "baka", "name": "バカ田大学"}
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
    College(Course),
    #[serde(rename = "bachelor")]
    Bachelor(Course),
    #[serde(rename = "master")]
    Master(Course),
    #[serde(rename = "doctor")]
    Doctor(Course),
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
struct Course {
    name: String,
    speciality: String,
}
