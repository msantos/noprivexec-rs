use std::env;
use std::ffi::CString;
use std::process::exit;

use nix::unistd::execvp;

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

#[cfg(target_os = "openbsd")]
fn disable_setuid() -> Result<(), i32> {
    let pledgenames = CString::new(PLEDGENAMES).expect("CString::new failed");
    unsafe {
        let rv = pledge(std::ptr::null(), pledgenames.as_ptr());
        let errno = __errno();

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
