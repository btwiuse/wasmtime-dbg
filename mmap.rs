use libc::{c_long, mmap, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_NONE};
use std::env;
use std::io::Error;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;

fn main() {
    const LENGTH: usize = 6444598427648;
        let args: Vec<String> = env::args().collect();
    let length: usize = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(len) => len,
            Err(_) => {
                eprintln!("Invalid length specified. Using default value.");
                LENGTH
            }
        }
    } else {
        LENGTH
    };
    let addr: *mut c_void = null_mut();
    let prot: c_int = PROT_NONE;
    let flags: c_int = MAP_PRIVATE | MAP_ANONYMOUS;
    let fd: c_int = -1;
    let offset: c_long = 0;

    let result = unsafe { mmap(addr, dbg!(length), prot, flags, fd, offset) };
    if result == MAP_FAILED {
        let err = Error::last_os_error();
        eprintln!("mmap failed: {}", err);
        std::process::exit(1);
    }

    println!("Memory mapped at address: {:p}", result);
}
