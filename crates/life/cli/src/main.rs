use mrly_core_cli_hand::*;

#[path = "doors.gen.rs"]
mod doors;

fn main() -> std::process::ExitCode {
    run("mrly-life", doors::DOORS)
}
