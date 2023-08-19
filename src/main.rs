pub mod cli;
pub mod update_dependency;
pub mod merge;
use update_dependency::handle_update;
use merge::handle_merge;
use cli::cli;


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("update", sub)) => {
            handle_update(sub.clone());
        }
        Some(("merge", sub)) => {
            handle_merge(sub.clone());
        },
        _ => unreachable!("No subcommand specified"),
    }
}
