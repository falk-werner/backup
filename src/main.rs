/*
mod config;
mod checksum_file;
mod checksum_error;

*/

/*
fn create(source: &Vec<String>, target: &String) {
    let target_path = Path::new(target);
    if !target_path.exists() {
        let _ = create_dir_all(target_path);
    }

    if !target_path.is_dir() {
        println!("error: path {} does not exists", target);
        return;
    }

    if !target_path.read_dir().unwrap().next().is_none() {
        println!("error: path {} is not empty", target);
        return;
    }

    let mut count = 0;
    for source_entry in source {
        let cpath = Path::new(source_entry);
        if let Ok(source_path) = canonicalize(cpath) {
            if source_path.is_file() {
                println!("backup {:?} -> {}", source, target);
                count += 1;
            }
    
            if source_path.is_dir() {
                if let Some(file_name) = source_path.file_name() {
                    println!("backup {:?} -> {}/{}", source_path, target, file_name.to_string_lossy());
                }
                else {
                    println!("error");
                }
                for entry in WalkDir::new(source_path.clone()) {
                    if let Ok(e) = entry {
                        let p = e.path().strip_prefix(source_path.clone()).unwrap();
                        if p.is_file() {
                            count += 1;
                            println!("{}", p.display());
                        }
                    }
                }
    
            }
    
        }
    }

    println!("count: {}", count);
}

fn check(target: &String) {
    println!("check {}", target);
}

*/

mod args;
mod backup;
mod check;
mod restore;
mod checksum_file;
mod checksum_error;

use crate::args::Args;
use crate::backup::backup;
use crate::restore::restore;
use crate::check::check;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    if args.restore {
        let target = args.get_target();
        if None == target {
            eprintln!("error: TARGET missing");
            return ExitCode::FAILURE;
        }

        restore(args.get_source(), target.unwrap(), args.get_checksum_file());
    }
    else if args.check {
        check(args.get_source(), args.get_checksum_file());
    }
    else {
        let target = args.get_target();
        if None == target {
            eprintln!("error: TARGET missing");
            return ExitCode::FAILURE;
        }

        return backup(args.get_source(), target.unwrap(), args.get_checksum_file());
    }

    return ExitCode::SUCCESS;
}
