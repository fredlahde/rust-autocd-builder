use regex;
use std::env;
use std::fs;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::path;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("please supply a path to cargo project as first argument");
        std::process::exit(1);
    }
    let f_arg = &args[1];
    let base_path = path::Path::new(f_arg);
    println!("building {:?}", base_path);
    let mut build_command = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&base_path)
        .spawn()?;

    if !build_command.wait()?.success() {
        panic!("failed to build!");
    }

    let mut toml = fs::File::open(base_path.join("Cargo.toml"))?;
    let name = extract_name(&mut toml)?;
    let target_path = base_path.join("target/release");
    let bin_path = target_path.join(name);
    let new_path = path::Path::new("app");
    println!("copying from {:?} to {:?}", bin_path, new_path);
    fs::copy(&bin_path, new_path)?;
    Ok(())
}

fn extract_name(toml: &mut dyn Read) -> std::io::Result<String> {
    let mut toml_string = String::default();
    toml.read_to_string(&mut toml_string)?;

    for line in toml_string.split('\n') {
        if line.contains("name =") {
            let re = regex::Regex::new("name\\s=\\s\"(.*)\"").expect("invalid regex");
            let caps = re.captures(&line);
            match caps {
                None => {
                    return Err(Error::new(ErrorKind::NotFound, "no name found"));
                }
                Some(caps) => match caps.get(1) {
                    None => {
                        return Err(Error::new(ErrorKind::NotFound, "no name found"));
                    }
                    Some(name) => return Ok(name.as_str().to_owned()),
                },
            }
        }
    }
    Err(Error::new(ErrorKind::NotFound, "no name found"))
}
