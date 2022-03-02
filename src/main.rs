use std::env;
use std::ffi::CString;
use std::process::exit;

use nix::unistd::execvp;
use prctl::set_no_new_privileges;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        usage()
    }

    let argv: Vec<_> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    match set_no_new_privileges(true) {
        Ok(_) => (),
        Err(status) => exit(status),
    }

    execvp(&argv[0], &argv)?;

    unreachable!()
}
