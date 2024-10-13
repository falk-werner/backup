use crate::checksum_file::ChecksumFile;

use std::path::PathBuf;
use std::process::ExitCode;
use std::fs::{canonicalize, create_dir_all, copy};

use walkdir::WalkDir;
use sha256::try_digest;

pub fn backup(source: PathBuf, target: PathBuf, checksum_file: PathBuf) -> ExitCode {
    if !source.is_dir() {
        eprintln!("error: SOURCE must be an existing directory");
        return ExitCode::FAILURE;
    }
    let source = canonicalize(source);
    if !source.is_ok() {
        eprintln!("error: failed to canocialize SOURCE");
        return ExitCode::FAILURE;
    }
    let source = source.unwrap();

    if !target.is_dir() {
        if !create_dir_all(target.clone()).is_ok() {
            eprintln!("error: failed to create TARGET");
            return ExitCode::FAILURE;
        }
    }
    let target = canonicalize(target);
    if !target.is_ok() {
        eprintln!("error: failed to canocicalize TARGET");
        return ExitCode::FAILURE;
    }
    let target = target.unwrap();

    let mut exit_code = ExitCode::SUCCESS;
    let mut checksums = ChecksumFile::new();
    for entry in WalkDir::new(source.clone()) {
        if let Ok(entry) = entry {
            let source_path = entry.path();            
            if source_path.is_file() {
                println!("process {}", source_path.to_string_lossy());
                let relative_path = source_path.strip_prefix(source.clone()).unwrap();

                let digest = try_digest(source_path);
                if !digest.is_ok() {
                    eprintln!("error: failed to create checksum");
                    exit_code = ExitCode::FAILURE;
                    continue;
                }
                let digest = digest.unwrap();
                checksums.add(&relative_path.to_string_lossy(), &digest);

                let target_path = target.join(relative_path);

                let parent = target_path.parent().unwrap();
                if !create_dir_all(parent).is_ok() {
                    eprintln!("error: failed to create path {:?}", target_path.parent().unwrap());
                    exit_code = ExitCode::FAILURE;
                    continue;
                }

                if !copy(source_path, target_path).is_ok() {
                    eprintln!("error: failed to backup {:?}", source_path);
                    exit_code = ExitCode::FAILURE;
                    continue;
                }
            }
        }
    }
    if !checksums.save(checksum_file).is_ok() {
        eprintln!("error: failed to save checksum file");
        exit_code = ExitCode::FAILURE;
    }


    exit_code
}