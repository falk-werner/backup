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
        if target.is_none() {
            eprintln!("error: TARGET missing");
            return ExitCode::FAILURE;
        }

       return restore(args.get_source(), target.unwrap(), args.get_checksum_file(), args.policy);
    }
    else if args.check {
        return check(args.get_source(), args.get_checksum_file());
    }
    else {
        let target = args.get_target();
        if target.is_none() {
            eprintln!("error: TARGET missing");
            return ExitCode::FAILURE;
        }

        return backup(args.get_source(), target.unwrap(), args.get_checksum_file());
    }
}
