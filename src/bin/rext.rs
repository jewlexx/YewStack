use std::{env::current_dir, fs::read_dir};

fn main() {
    let cwd = current_dir().expect("could not read current directory");

    let has_cargo = read_dir(cwd)
        .expect("could not read directory")
        .flatten()
        .any(|x| x.path().ends_with("Cargo.toml"));

    if !has_cargo {
        panic!("rext: no Cargo.toml found in current directory");
    }
}
