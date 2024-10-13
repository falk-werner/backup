
use crate::checksum_error::ChecksumError;

use std::io::{BufReader, BufRead, Result, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use regex::Regex;
use sha256::try_digest;

pub struct ChecksumFile {
    pub files: Vec<FileInfo>,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub sha256: String
}

impl ChecksumFile {
    pub fn new() -> Self {
        ChecksumFile { files: vec!() }
    }

    pub fn from_file<P>(filename: P) -> Result<Self> where P: AsRef<Path>, {
        let re = Regex::new(r"^SHA256 \((.+)\) = ([0-9a-fA-F]+)$").unwrap();

        let mut files = Vec::<FileInfo>::new();
        let file = File::open(filename)?;
        let lines = BufReader::new(file).lines();
        for line in lines.flatten() {
            if let Some(caps) = re.captures(&line) {
                let path = String::from(caps.get(1).unwrap().as_str());
                let sha256 = String::from(caps.get(2).unwrap().as_str());

                files.push(FileInfo { path: path, sha256: sha256})
            }
        }

        Ok(ChecksumFile { files: files })
    }

    pub fn add(&mut self, path: &str, sha256: &str) {
        self.files.push( FileInfo {path: String::from(path), sha256: String::from(sha256)});
    }

    pub fn check<P>(&self, base_path: P) -> std::result::Result<(), ChecksumError> where P: AsRef<Path> {
        let mut error = ChecksumError::new();

        for file in &self.files {
            let result = check_file(base_path.as_ref(), &file.path, &file.sha256);
            if let Err(mismatch) = result {
                error.file_names.push(mismatch)
            }
        }


        if error.file_names.len() == 0 {
            Ok(())
        } else {
            Err(error)
        }
    }

    pub fn save(&self, path: PathBuf) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for entry in &self.files {
            writeln!(file, "SHA256 ({}) = {}", entry.path, entry.sha256)?;
        }

        Ok(())
    }
}

fn check_file(base_path: &Path, name: &str, sha256: &str) -> std::result::Result<(), String> {
    let mut path = base_path.to_path_buf();
    path.push(name);

    let result = try_digest(path);
    if let Ok(digest) = result {
        if digest == sha256 {
            Ok(())
        }
        else {
            Err(String::from(name))
        }
    }
    else {
        Err(String::from(name))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn new() {
        let checksums = ChecksumFile::new();
        assert_eq!(0, checksums.files.len());
    }

    #[test]
    fn add() {
        let mut checksums = ChecksumFile::new();
        checksums.add("foo", "1234");
        checksums.add("bar", "abcd");
        assert_eq!(2, checksums.files.len());

        assert_eq!("foo", checksums.files.get(0).unwrap().path);
        assert_eq!("1234", checksums.files.get(0).unwrap().sha256);

        assert_eq!("bar", checksums.files.get(1).unwrap().path);
        assert_eq!("abcd", checksums.files.get(1).unwrap().sha256);
    }

    #[test]
    fn read_checksums() {
        let dir = tempdir().unwrap();
        let mut path = PathBuf::from(dir.as_ref());
        path.push("checksums.txt");
        fs::write(path.clone(), "SHA256 (foo) = 1234").unwrap();

        let checksums = ChecksumFile::from_file(path);
        assert!(checksums.is_ok());

        let checksums = checksums.unwrap();
        assert_eq!(1, checksums.files.len());
        
        let file = checksums.files.get(0).unwrap();
        assert_eq!("foo", file.path);
        assert_eq!("1234", file.sha256);
    }

    #[test]
    fn read_checksums_with_parentheses() {
        let dir = tempdir().unwrap();
        let mut path = PathBuf::from(dir.as_ref());
        path.push("checksums.txt");
        fs::write(path.clone(), "SHA256 (foo().txt) = 1234").unwrap();

        let checksums = ChecksumFile::from_file(path);
        assert!(checksums.is_ok());

        let checksums = checksums.unwrap();
        assert_eq!(1, checksums.files.len());
        
        let file = checksums.files.get(0).unwrap();
        assert_eq!("foo().txt", file.path);
        assert_eq!("1234", file.sha256);
    }

}
