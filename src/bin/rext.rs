use std::{env::current_dir, fs::read_dir};

macro_rules! better_panic {
    ($($arg:tt)*) => {
        println!($($arg)*);
        std::process::exit(1);
    };
}

fn main() {
    let cwd = current_dir().expect("could not read current directory");
    let src_path = cwd.join("src");
    let pages_path = src_path.join("pages");

    let has_cargo = {
        let exists = read_dir(cwd)
            .expect("could not read directory")
            .flatten()
            .any(|x| x.path().ends_with("Cargo.toml"));

        let file = "Cargo.toml";

        (exists, file)
    };

    let has_src = {
        let exists = src_path.exists();

        let file = "src directory";

        (exists, file)
    };

    let has_pages = {
        let exists = pages_path.exists();

        let file = "pages directory";

        (exists, file)
    };

    let problems = vec![has_cargo, has_src, has_pages];

    if !problems.iter().any(|x| !x.0) {
        better_panic!(
            "rext: no {} found in current directory",
            problems
                .iter()
                .filter(|x| !x.0)
                .map(|x| x.1)
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
}
