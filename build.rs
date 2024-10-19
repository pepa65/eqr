use clap::CommandFactory;
use clap_mangen;
use std::{env, fs};
use clap_complete::{generate_to, Shell::*};

// Include Args struct.
include!("src/args.rs");

fn main() {
    // Generate man & completions directories.
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("unable to determine manifest dir"));
    let man_dir = manifest_dir.join("man");
    let comp_dir = manifest_dir.join("completions");

    fs::create_dir_all(&man_dir).expect("unable to create man directory");
    fs::create_dir_all(&comp_dir).expect("unable to create completions directory");

    // Retrieve Args and set binary name.
    let mut cmd = Args::command();

    // Generate & write man page.
    let mut buffer: Vec<u8> = Vec::new();
    clap_mangen::Man::new(cmd.clone()).render(&mut buffer).expect("unable to generate man page");
    let buf = std::str::from_utf8(&buffer).unwrap();
    let new = buf.replace("fBeqr", "fBqr");
    fs::write(man_dir.join("eqr.1"), new).expect("unable to write man page");

    // Generate shell completions.
    for shell in [Bash, Elvish, Fish, PowerShell, Zsh] {
        generate_to(shell, &mut cmd, "eqr", &comp_dir).expect("unable to generate completions");
    }
}
