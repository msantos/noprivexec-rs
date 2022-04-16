use std::env;
use std::ffi::CString;
use std::process::exit;

use noprivexec::{disable_setuid, execvp};

const PROGNAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn usage() -> ! {
    eprintln!(
        r#"{} {}
usage: <COMMAND> <...>"#,
        PROGNAME, VERSION,
    );
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        usage()
    }

    let argv: Vec<_> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    match disable_setuid() {
        Ok(_) => (),
        Err(errno) => exit(errno),
    }

    exit(execvp(argv));
}
