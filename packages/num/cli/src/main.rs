use mrly_core::hand::cli::*;

mod doors;

fn main() -> std::process::ExitCode {
    run("mrly-num", doors::DOORS)
}
