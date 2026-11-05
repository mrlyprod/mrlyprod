use mrly_core::hand::cli::*;

mod doors;

fn main() -> std::process::ExitCode {
    run("mrly-core", doors::DOORS)
}
