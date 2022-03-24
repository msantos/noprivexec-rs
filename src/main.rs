use std::env;
use std::ffi::CString;
use std::process::exit;

use libc::{c_char, execvp};

#[cfg(target_os = "linux")]
use libc::{__errno_location, prctl, PR_SET_NO_NEW_PRIVS};

#[cfg(target_os = "openbsd")]
use libc::{__errno, pledge};

#[cfg(target_os = "openbsd")]
const PLEDGENAMES: &str = "stdio rpath wpath cpath dpath tmppath \
                           inet mcast fattr chown flock unix dns getpw \
                           sendfd recvfd tape tty proc exec prot_exec \
                           settime ps vminfo id pf route wroute \
                           audio video bpf unveil error disklabel \
                           drm vmm";

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

#[cfg(target_os = "linux")]
fn errno() -> i32 {
    unsafe {
        let e = __errno_location();
        *e
    }
}

#[cfg(target_os = "openbsd")]
fn errno() -> i32 {
    unsafe {
        let e = __errno();
        *e
    }
}

#[cfg(target_os = "linux")]
fn disable_setuid() -> Result<(), i32> {
    match unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } {
        0 => Ok(()),
        _ => Err(errno()),
    }
}

#[cfg(target_os = "openbsd")]
fn disable_setuid() -> Result<(), i32> {
    let pledgenames = CString::new(PLEDGENAMES).expect("CString::new failed");
    match unsafe { pledge(std::ptr::null(), pledgenames.as_ptr()) } {
        0 => Ok(()),
        _ => Err(errno()),
    }
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
