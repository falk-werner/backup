use crate::checksum_file::ChecksumFile;
use crate::args::Policy;

use std::path::PathBuf;
use std::process::ExitCode;
use std::fs::{canonicalize, create_dir_all, copy};

pub fn restore(source: PathBuf, target: PathBuf, checksum_file: PathBuf, policy: Policy) -> ExitCode {
    if !target.is_dir() {
        eprintln!("error: TARGET must be an existing directory");
        return ExitCode::FAILURE;
    }
    let target = canonicalize(target);
    if target.is_err() {
        eprintln!("error: failed to canocialize TARGET");
        return ExitCode::FAILURE;
    }
    let target = target.unwrap();

    if !source.is_dir() && create_dir_all(source.clone()).is_err() {
            eprintln!("error: failed to create SOURCE");
            return ExitCode::FAILURE;
    }
    let source = canonicalize(source);
    if source.is_err() {
        eprintln!("error: failed to canocicalize SOURCE");
        return ExitCode::FAILURE;
    }
    let source = source.unwrap();

    let checksums = ChecksumFile::from_file(checksum_file);
    if checksums.is_err() {
        eprintln!("error: failed to load checksum file");
        return ExitCode::FAILURE;
    }
    let checksums = checksums.unwrap();

    let mut exit_code = ExitCode::SUCCESS;
    for file in checksums.files {
        let source_path = source.join(&file.path);
        let target_path = target.join(&file.path);

        print!("restore {}... ", source_path.to_string_lossy());
        let do_restore = match policy {
            Policy::All => true,
            Policy::OnlyMissing => !source_path.exists(),
            Policy::OnlyNewer => true,
        };

        if do_restore {
            let parent = source_path.parent().unwrap();
            if create_dir_all(parent).is_err() {
                println!("error");
                eprintln!("error: failed to create path {:?}", source_path.parent().unwrap());
                exit_code = ExitCode::FAILURE;
                continue;
            }

            if copy(target_path.clone(), source_path).is_err() {
                println!("error");
                eprintln!("error: failed to restore {:?}", target_path);
                exit_code = ExitCode::FAILURE;
                continue;
            }

            println!("ok");    
        }
        else {
            println!("skip");
        }
    }

    exit_code
}