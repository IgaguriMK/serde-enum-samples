mod samples;

use failure::Error;

fn main() {
    let samples: Vec<fn() -> Result<(), Error>> = vec![
        samples::enum_default::run,
        samples::internally::run,
        samples::adjacently::run,
        samples::untagged::run,
    ];

    for s in samples {
        match s() {
            Ok(_) => {}
            Err(err) => {
                println!("\nError: {}", err);
            }
        }
    }
}
