use std::env;
use std::process::ExitCode;
use namesniffer::Target;

fn main() -> ExitCode {
    let mut args = env::args();
    args.next().expect("every program has a name");
    let Some(username) = args.next() else {
        eprintln!("expected username");
        return ExitCode::FAILURE
    };

    let targets = [
        Target {
            website: "https://twitter.com/".to_owned(),
            content_denoting_availability: "This account doesn't exist".to_owned()
        },
        Target {
            website: "https://deviantart.com/".to_owned(),
            content_denoting_availability: "Page Not Found".to_owned()
        },
        Target {
            website: "https://instagram.com/".to_owned(),
            content_denoting_availability: "Sorry, this page isn't available.".to_owned()
        }
    ];

    for target in targets {
        if namesniffer::is_username_available(&target, &username) {
            println!("Available: {}", target.website);
        } else {
            println!("UNAVAILABLE: {}", target.website);
        }
    }

    ExitCode::SUCCESS
}