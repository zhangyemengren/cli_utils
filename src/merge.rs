use clap::ArgMatches;
use serde_json::Value;
use std::error::Error;
use std::process::{Command as Cmd, Stdio};
use std::{fs, path::Path, thread};
pub fn handle_merge(sub: ArgMatches) {
    let default_dir = ".".to_string();
    let empty = "".to_string();
    let default_mode = "pull".to_string();
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
    let branch = branch.as_ref();
    let target = target.as_ref();
    let mode = mode.as_ref();

    let output = Cmd::new("git")
        .current_dir(&dir)
        .args(["diff", "--name-only"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();
    let str = String::from_utf8(output.stdout).unwrap();
    if !str.is_empty() {
        println!("\x1b[31m {:?} {} {str}\x1b[0m", dir.as_ref(), "work space not clean");
        return;
    }


    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .args(["checkout", &branch])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.wait().unwrap();

    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .args(["pull"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.wait().unwrap();


    let output = Cmd::new("git")
        .current_dir(&dir)
        .args([&mode, "origin", &target])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let err = String::from_utf8(output.stderr).unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    println!("\x1b[31m {} \x1b[0m", err);
    println!("{} out", out);

    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .args(["submodule", "update"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.wait().unwrap();


}