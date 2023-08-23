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
                .arg(arg!(-d --dir[DIR] "dir name"))
                .arg(arg!(-p --package <PACKAGE> "dependencies package name"))
                .arg(arg!(-v --version <VERSION> "dependencies package version"))
                .arg(arg!(-i --include [INCLUDE] "include files name (split by ,)"))
                .arg(arg!(-e --exclude [EXCLUDE] "exclude files name (split by ,)"))
                .arg(arg!(-b --branch <BRANCH> "branch name")),
        )
        .subcommand(
            Command::new("merge")
                .about("merge branch")
                .arg(arg!(-d --dir[DIR] "dir name"))
                .arg(arg!(-i --include [INCLUDE] "include files name (split by ,)"))
                .arg(arg!(-e --exclude [EXCLUDE] "exclude files name (split by ,)"))
                .arg(arg!(-b --branch <BRANCH> "base branch name"))
                .arg(arg!(-t --target [TARGET] "merge branch name ( default master)"))
                .arg(arg!(-m --mode [MODE] "merge or rebase( default merge)")),

        )
}
