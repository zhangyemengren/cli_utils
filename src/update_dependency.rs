use clap::ArgMatches;
use serde_json::Value;
use std::error::Error;
use std::process::{Command as Cmd, Stdio};
use std::{fs, path::Path, thread};

pub fn handle_update(sub: ArgMatches) {
    let default_dir = ".".to_string();
    let dir = sub.get_one::<String>("dir").unwrap_or(&default_dir);
    let branch = sub.get_one::<String>("branch").unwrap();
    let package = sub.get_one::<String>("package").unwrap();
    let version = sub.get_one::<String>("version").unwrap();
    let mut workers = vec![];

    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
        .filter(|e| {
            let r: Result<bool, Box<dyn Error>> = (|| {
                let path = e.join("package.json");
                let json = fs::read_to_string(path)?;
                let json = serde_json::from_str::<Value>(&json)?;

                let v = json
                    .get("dependencies")
                    .ok_or("")?
                    .get(package)
                    .ok_or("")?
                    .as_str()
                    .ok_or("")?;
                Ok(!v.contains(version))
            })();
            r.unwrap_or(false)
        })
        .for_each(|dir| {
            let dir = dir.clone();
            let branch = branch.clone();
            let package = package.clone();
            let version = version.clone();
            workers.push(thread::spawn(move || {
                handle_update_command((dir, branch, package, version));
            }));
        });
    workers.into_iter().for_each(|w| w.join().unwrap());
    println!("success");
}
pub fn handle_update_command<P, S>((dir, branch, package, version): (P, S, S, S))
where
    P: AsRef<Path>,
    S: AsRef<str> + std::fmt::Debug + std::fmt::Display,
{
    println!("{:?}", dir.as_ref());
    let mut is_new_branch = false;
    let branch = branch.as_ref();
    let package = package.as_ref();
    let version = version.as_ref();
    let output = Cmd::new("git")
        .current_dir(&dir)
        .args(["diff", "--name-only"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();
    let str = String::from_utf8(output.stdout).unwrap();
    if !str.is_empty() {
        println!("\x1b[31m {} {str}\x1b[0m", "work space not clean");
        return;
    }
    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .args(["checkout", "master"])
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
    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .args(["submodule", "update"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.wait().unwrap();

    let output = Cmd::new("git")
        .current_dir(&dir)
        .args(["checkout", &branch])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    let str = String::from_utf8(output.stderr).unwrap();
    if str.contains("error") {
        println!("{} now create a new branch", str);
        let mut child = Cmd::new("git")
            .current_dir(&dir)
            .args(["checkout", "-b", &branch])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        child.wait().unwrap();
        is_new_branch = true;
    } else {
        let mut child = Cmd::new("git")
            .current_dir(&dir)
            .args(["pull"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        child.wait().unwrap();
        let mut child = Cmd::new("git")
            .current_dir(&dir)
            .args(["submodule", "update"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
    let install_value = format!("{}@{}", package, version);

    let mut child = Cmd::new("npm")
        .current_dir(&dir)
        .args(["install", &install_value])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.wait().unwrap();
    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(["add", "."])
        .spawn()
        .unwrap();
    child.wait().unwrap();
    let mut child = Cmd::new("git")
        .current_dir(&dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(["commit", "-m", "build: update"])
        .spawn()
        .unwrap();
    child.wait().unwrap();
    if is_new_branch {
        let output = Cmd::new("git")
            .current_dir(&dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(["push", "--set-upstream", "origin", &branch])
            .output()
            .unwrap();
        let err = String::from_utf8(output.stderr).unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        println!("\x1b[31m {} \x1b[0m", err);
        println!("{}", out);
    } else {
        let output = Cmd::new("git")
            .current_dir(&dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(["push"])
            .output()
            .unwrap();
        let err = String::from_utf8(output.stderr).unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        println!("\x1b[31m {} \x1b[0m", err);
        println!("{}", out);
    }
}
