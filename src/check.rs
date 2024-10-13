use std::path::PathBuf;
use std::process::ExitCode;
use crate::checksum_file::ChecksumFile;

pub fn check(source: PathBuf, checksum_file: PathBuf) -> ExitCode {
    let checksums = ChecksumFile::from_file(checksum_file);
    if checksums.is_err() {
        eprintln!("error: failed to load checksum file");
        return ExitCode::FAILURE;
    }
    let checksums = checksums.unwrap();

    let result = checksums.check(source);
    if let Err(result) = result {
        for name in result.file_names {
            eprintln!("error: checksum mismatch: {}", name);
        }

        return ExitCode::FAILURE;
    }

    println!("Ok");
    ExitCode::SUCCESS
}