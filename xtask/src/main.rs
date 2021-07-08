use std::{
    env,
    path::{Path, PathBuf},
    process::{self, Command},
};

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(xtask =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand xtest =>
            (about: "xtask test")
            (@arg release: --release "Test for xtask")
        )
    )
    .get_matches();
    println!("{:?}", matches);
}
