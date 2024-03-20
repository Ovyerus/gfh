use shellexpand::tilde;
use std::{
    env, fs,
    process::{self, exit},
};

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0); // First `argv` is the current filename.

    let (index, _) = args
        .iter()
        .enumerate()
        .find(|(_, arg)| **arg == String::from("-f"))
        .expect("git did not provide -f somehow");
    let index = index + 1; // We want the file name, which comes after `-f`

    let file = args[index].clone();
    let new_file = fs::read_to_string(file)
        .expect("file should exist")
        .trim()
        .to_owned();
    let new_file = tilde(&new_file);
    args[index] = new_file.into_owned();

    let status = process::Command::new("ssh-keygen")
        .arg("-v")
        .args(args)
        .status()
        .expect("failed to run ssh-keygen");

    if !status.success() {
        exit(status.code().or(Some(1)).unwrap());
    }
}
