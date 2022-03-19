use std::env;
use std::ffi::CString;
use std::process::exit;

use libc::{__errno_location, prctl, PR_SET_NO_NEW_PRIVS};
use nix::unistd::execvp;

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

fn disable_setuid() -> Result<(), i32> {
    unsafe {
        let rv = prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
        let errno = __errno_location();

        if rv != 0 {
            return Err(*errno);
        }
    }
    Ok(())
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

    match disable_setuid() {
        Ok(_) => (),
        Err(errno) => exit(errno),
    }

    execvp(&argv[0], &argv)?;

    unreachable!()
}
