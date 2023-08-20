use clap::ArgMatches;
use serde_json::Value;
use std::error::Error;
use std::process::{Command as Cmd, Stdio};
use std::{fs, path::Path, thread};
pub fn handle_merge(sub: ArgMatches) {
    let default_dir = ".".to_string();
    let empty = "".to_string();
    let default_mode = "merge".to_string();
    let default_target = "master".to_string();
    let dir = sub.get_one::<String>("dir").unwrap_or(&default_dir);
    let branch = sub.get_one::<String>("branch").unwrap();
    let target = sub.get_one::<String>("target").unwrap_or(&default_target);
    let mode = sub.get_one::<String>("mode").unwrap_or(&default_mode);
    let include = sub
        .get_one::<String>("include")
        .unwrap_or(&empty)
        .split(",")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let exclude = sub
        .get_one::<String>("exclude")
        .unwrap_or(&empty)
        .split(",")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut workers = vec![];

    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter_map(|d| d.ok())
        .filter(|d| d.path().is_dir())
        .map(|d| d.path())
        .filter_map(|p| {
            let name = p.file_name()?.to_str()?;
            if exclude.contains(&name) {
                return None;
            }
            if !include.is_empty() && !include.contains(&name) {
                None
            } else {
                Some(p)
            }
        })
        .for_each(|dir| {
            let dir = dir.clone();
            let branch = branch.clone();
            let target = target.clone();
            let mode = mode.clone();
            workers.push(thread::spawn(move || {
                handle_update_command((dir, branch, target, mode));
            }));
        });
    workers.into_iter().for_each(|w| w.join().unwrap());
    println!("success");
}
pub fn handle_update_command<P, S>((dir, branch, target, mode): (P, S, S, S))
    where
        P: AsRef<Path>,
        S: AsRef<str> + std::fmt::Debug + std::fmt::Display,
{
    println!("{:?}", dir.as_ref());
}