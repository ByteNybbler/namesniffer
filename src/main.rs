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
        Target::TWITTER,
        Target::DEVIANTART,
        Target::INSTAGRAM
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