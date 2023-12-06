use libc::{c_long, mmap, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_NONE};
use std::io::Error;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;

fn main() {
    let addr: *mut c_void = null_mut();
    let length: usize = 6444598427648;
    let prot: c_int = PROT_NONE;
    let flags: c_int = MAP_PRIVATE | MAP_ANONYMOUS;
    let fd: c_int = -1;
    let offset: c_long = 0;

    let result = unsafe { mmap(addr, length, prot, flags, fd, offset) };
    if result == MAP_FAILED {
        let err = Error::last_os_error();
        eprintln!("mmap failed: {}", err);
        std::process::exit(1);
    }

    println!("Memory mapped at address: {:p}", result);
}
