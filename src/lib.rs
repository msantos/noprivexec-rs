//! Disable setuid privileges
//!
//! Remove the ability of a process to escalate privileges using the `setresuid(2)` family of
//! syscalls or file capabilities.
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

#[cfg(target_os = "freebsd")]
use libc::{
    __error, c_int, c_void, procctl, PROC_NO_NEW_PRIVS_CTL, PROC_NO_NEW_PRIVS_ENABLE, P_PID,
};

/// Retrieve the last error number of a system or library call.
pub fn errno() -> i32 {
    #[cfg(target_os = "linux")]
    {
        unsafe { *__errno_location() }
    }

    #[cfg(target_os = "openbsd")]
    {
        unsafe { *__errno() }
    }

    #[cfg(target_os = "freebsd")]
    {
        unsafe { *__error() }
    }

    #[cfg(not(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd")))]
    {
        libc::ENOSYS
    }
}

/// Remove the capability to escalate privileges from the running process.
pub fn disable_setuid() -> Result<(), i32> {
    #[cfg(target_os = "linux")]
    {
        match unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } {
            0 => Ok(()),
            _ => Err(errno()),
        }
    }

    #[cfg(target_os = "openbsd")]
    {
        let pledgenames = CString::new(PLEDGENAMES).expect("CString::new failed");
        match unsafe { pledge(std::ptr::null(), pledgenames.as_ptr()) } {
            0 => Ok(()),
            _ => Err(errno()),
        }
    }

    #[cfg(target_os = "freebsd")]
    {
        let mut data: c_int = PROC_NO_NEW_PRIVS_ENABLE;
        let p_data = &mut data as *mut c_int as *mut c_void;

        match unsafe { procctl(P_PID, 0, PROC_NO_NEW_PRIVS_CTL, p_data) } {
            0 => Ok(()),
            _ => Err(errno()),
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "openbsd", target_os = "freebsd")))]
    {
        Err(errno())
    }
}

/// Replace the current process image with the new process image specified by path and
/// arguments.
pub fn execvp(argv: Vec<CString>) -> i32 {
    let mut p_argv: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();

    p_argv.push(std::ptr::null());

    let p: *const *const c_char = p_argv.as_ptr();

    unsafe {
        libc::execvp(p_argv[0], p);
    };

    errno()
}
