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
        }
    ];

    for target in targets {
        if namesniffer::is_username_available(&target, &username) {
            println!("OK: {}", target.website);
        } else {
            println!("UNAVAILABLE: {}", target.website);
        }
    }

    ExitCode::SUCCESS
}