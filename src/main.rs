mod samples;

use failure::Error;

fn main() {
    let samples: Vec<fn() -> Result<(), Error>> = vec![
        samples::enum_default::run,
        samples::internally::run,
        samples::adjacently::run,
        samples::untagged::run,
        samples::untagged_multi_types::run,
        samples::untagged_multi_types_wrong::run,
        samples::other::run,
        samples::other_null::run,
        samples::extract_struct::run,
        samples::extract_multiple_struct::run,
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
