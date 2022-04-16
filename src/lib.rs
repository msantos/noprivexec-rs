use libc::c_char;
use std::ffi::CString;

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

#[cfg(target_os = "linux")]
pub fn errno() -> i32 {
    unsafe { *__errno_location() }
}

#[cfg(target_os = "openbsd")]
pub fn errno() -> i32 {
    unsafe { *__errno() }
}

#[cfg(target_os = "linux")]
pub fn disable_setuid() -> Result<(), i32> {
    match unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } {
        0 => Ok(()),
        _ => Err(errno()),
    }
}

#[cfg(target_os = "openbsd")]
pub fn disable_setuid() -> Result<(), i32> {
    let pledgenames = CString::new(PLEDGENAMES).expect("CString::new failed");
    match unsafe { pledge(std::ptr::null(), pledgenames.as_ptr()) } {
        0 => Ok(()),
        _ => Err(errno()),
    }
}

pub fn execvp(argv: Vec<CString>) -> i32 {
    let mut p_argv: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();

    p_argv.push(std::ptr::null());

    let p: *const *const c_char = p_argv.as_ptr();

    unsafe {
        libc::execvp(p_argv[0], p);
    };

    errno()
}
