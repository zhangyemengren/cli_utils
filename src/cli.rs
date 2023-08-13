use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("cli")
        .about("cli utils")
        .subcommand_required(true)
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("update")
                .about("update projects common dependencies")
                .arg(arg!(--dir[DIR] "dir name"))
                .arg(arg!(--package <PACKAGE> "dependencies package name"))
                .arg(arg!(--version <VERSION> "dependencies package version"))
                .arg(arg!(--include [INCLUDE] "include files name"))
                .arg(arg!(--exclude [EXCLUDE] "exclude files name"))
                .arg(arg!(--branch <BRANCH> "branch name")),
        )
        .subcommand(Command::new("b").about("Prints 'b'"))
}