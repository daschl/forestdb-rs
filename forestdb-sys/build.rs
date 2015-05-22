use std::env;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {

	let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

	let _ = fs::create_dir("forestdb/build");

	 run(Command::new("cmake").arg("../").current_dir("forestdb/build"), "cmake");
	 run(Command::new("make").arg("all").current_dir("forestdb/build"), "make");
	 run(Command::new("cp").arg("-r").arg("forestdb/build").arg(dst), "mv");

	 println!("cargo:rustc-link-lib=dylib=forestdb");
	 println!("cargo:rustc-link-search=all={}", &env::var("OUT_DIR").unwrap());
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}