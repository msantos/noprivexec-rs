use std::env;
use std::ffi::CString;
use std::process::exit;

use libc::{c_char, execvp};

use noprivexec::{disable_setuid, errno};

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

    let mut p_argv: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();

    p_argv.push(std::ptr::null());

    let p: *const *const c_char = p_argv.as_ptr();

    match disable_setuid() {
        Ok(_) => (),
        Err(errno) => exit(errno),
    }

    unsafe {
        execvp(p_argv[0], p);
    };

    exit(errno());
}
