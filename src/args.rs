use clap::{Parser};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help="Source directory of backup")]
    source: String,

    #[arg(help="Target directory of backup")]
    target: Option<String>,

    #[arg(short='f', long, help="File containing checksums (Default: TARGET/checkusums.txt)")]
    checksum_file: Option<String>,

    #[arg(short, long, help="Check against checksum file only")]
    pub check: bool,

    #[arg(short, long, help="Restore files")]
    pub restore: bool,
}

impl Args {

    pub fn get_source(&self) -> PathBuf {
        PathBuf::from(self.source.clone())
    }

    pub fn get_target(&self) -> Option<PathBuf> {
        match &self.target {
            Some(value) => Some(PathBuf::from(value.clone())),
            _ => None,
        }
    }

    pub fn get_checksum_file(&self) -> PathBuf {
        if let Some(value) = &self.checksum_file {
            PathBuf::from(value.clone())
        }
        else if let Some(mut value) = self.get_target() {
            value.push("checksums.txt");
            value
        }
        else {
            let mut source = self.get_source();
            source.push("checksums.txt");
            source
        }
    }

}

#[cfg(test)]
mod test {

use super::*;

#[test]
fn parse_args_source_only() {
    let in_args: Vec<&str> = vec!("backup", "source");
    let result = Args::try_parse_from(in_args);

    assert!(result.is_ok());
    let args = result.unwrap();

    assert!(args.source == "source");
    assert!(args.target == None);
    assert!(args.checksum_file == None);
    assert!(args.check == false);
    assert!(args.restore == false);
}

#[test]
fn parse_args_with_target() {
    let in_args: Vec<&str> = vec!("backup", "source", "target");
    let result = Args::try_parse_from(in_args);

    assert!(result.is_ok());
    let args = result.unwrap();

    assert!(args.source == "source");
    assert!(args.target == Some(String::from("target")));
    assert!(args.checksum_file == None);
    assert!(args.check == false);
    assert!(args.restore == false);
}

#[test]
fn parse_args_with_checksum_file() {
    let in_args: Vec<&str> = vec!("backup", "-f", "checksums.txt", "source", "target");
    let result = Args::try_parse_from(in_args);

    assert!(result.is_ok());
    let args = result.unwrap();

    assert!(args.source == "source");
    assert!(args.target == Some(String::from("target")));
    assert!(args.checksum_file == Some(String::from("checksums.txt")));
    assert!(args.check == false);
    assert!(args.restore == false);
}

#[test]
fn parse_args_check() {
    let in_args: Vec<&str> = vec!("backup", "-c", "source");
    let result = Args::try_parse_from(in_args);

    assert!(result.is_ok());
    let args = result.unwrap();

    assert!(args.source == "source");
    assert!(args.target == None);
    assert!(args.checksum_file == None);
    assert!(args.check == true);
    assert!(args.restore == false);
}

#[test]
fn parse_args_restore() {
    let in_args: Vec<&str> = vec!("backup", "-r", "source", "target");
    let result = Args::try_parse_from(in_args);

    assert!(result.is_ok());
    let args = result.unwrap();

    assert!(args.source == "source");
    assert!(args.target == Some(String::from("target")));
    assert!(args.checksum_file == None);
    assert!(args.check == false);
    assert!(args.restore == true);
}

#[test]
fn args_get_source() {
    let args = Args {
        source: String::from("source"),
        target: None,
        checksum_file: None,
        check: false,
        restore: false
    };

    assert!(args.get_source() == PathBuf::from("source"));
}

#[test]
fn args_get_target_none() {
    let args = Args {
        source: String::from("source"),
        target: None,
        checksum_file: None,
        check: false,
        restore: false
    };

    assert!(args.get_target() == None);
}

#[test]
fn args_get_target_some() {
    let args = Args {
        source: String::from("source"),
        target: Some(String::from("target")),
        checksum_file: None,
        check: false,
        restore: false
    };

    assert!(args.get_target() == Some(PathBuf::from("target")));
}

#[test]
fn args_get_checksum_file_given() {
    let args = Args {
        source: String::from("source"),
        target: None,
        checksum_file: Some(String::from("checksums.txt")),
        check: false,
        restore: false
    };

    assert!(args.get_checksum_file() == PathBuf::from("checksums.txt"));
}

#[test]
fn args_get_checksum_file_default_target() {
    let args = Args {
        source: String::from("source"),
        target: Some(String::from("target")),
        checksum_file: None,
        check: false,
        restore: false
    };

    let mut checksum_file = PathBuf::from("target");
    checksum_file.push("checksums.txt");
    assert!(args.get_checksum_file() == checksum_file);
}

#[test]
fn args_get_checksum_file_default_source() {
    let args = Args {
        source: String::from("source"),
        target: None,
        checksum_file: None,
        check: false,
        restore: false
    };

    let mut checksum_file = PathBuf::from("source");
    checksum_file.push("checksums.txt");
    assert!(args.get_checksum_file() == checksum_file);
}

}