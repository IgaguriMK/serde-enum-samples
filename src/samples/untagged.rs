use failure::Error;
use serde_derive::{Deserialize, Serialize};

pub fn run() -> Result<(), Error> {
    println!("\n======== Untagged enum serialize & deserialize ========\n");

    let members = vec![
        Member::Permanent {
            id: 1,
            name: "Jebediah Kerman".to_string(),
            nickname: Some("Jeb".to_string()),
        },
        Member::Permanent {
            id: 2,
            name: "Bob Kerman".to_string(),
            nickname: None,
        },
        Member::SingleChannel {
            channel_id: 8,
            name: "Kamler Kerman".to_string(),
            nickname: None,
        },
    ];

    let string = serde_json::to_string(&members)?;

    let members_deserialized: Vec<Member> = serde_json::from_str(&string)?;

    println!("Data: {:?}", members);
    println!("String: {}", string);
    println!("Decoded: {:?}", members_deserialized);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Member {
    Permanent {
        id: u64,
        name: String,
        nickname: Option<String>,
    },
    SingleChannel {
        channel_id: u64,
        name: String,
        nickname: Option<String>,
    },
}
