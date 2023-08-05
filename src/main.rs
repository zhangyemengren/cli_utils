pub mod cli;
pub mod update_dependency;
use update_dependency::handle_update;
use cli::cli;


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("update", sub)) => {
            handle_update(sub.clone());
        }
        Some(("b", _sub)) => println!("b"),
        _ => unreachable!("No subcommand specified"),
    }
}
