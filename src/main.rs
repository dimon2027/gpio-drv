use std::ffi;
use std::os::unix::raw;

const O_RDWR: ffi::c_int = 2;
const O_SYNC: ffi::c_int = 0x4000000;
const PERIPH: ffi::c_int = 0x7E000000;
const GPIO_OFFSET: ffi::c_int = 0x200000;
const PROT_READ: ffi::c_int = 0x1;
const PROT_WRITE: ffi::c_int = 0x2;
const PROT_RDWR: ffi::c_int = PROT_READ | PROT_WRITE;
const PAGE_SIZE: isize = 4096;

const MAP_SHARED: ffi::c_int = 0x01;

const GPIO_MEM_DEV: &str = "/dev/gpiomem";

fn main() {
    let res = ffi::CString::new(GPIO_MEM_DEV);
    if !res.is_ok() {
        println!("Error creating a CString");
        return;
    }

    let pathname = res.unwrap();

    let fd = unsafe { open(pathname.as_ptr(), O_RDWR | O_SYNC) };
    println!("fd = {}", fd);

    let addr = unsafe {
        mmap(
            0 as *const ffi::c_void,
            PAGE_SIZE,
            PROT_RDWR,
            MAP_SHARED,
            fd,
            (PERIPH + GPIO_OFFSET) as raw::off_t,
        )
    };

    if (addr as isize) < 0 {
        println!("Failed to map memory!");
        return;
    }

    println!("address = {}", addr as isize);

    let res = unsafe { munmap(addr, PAGE_SIZE) };
    if res < 0 {
        println!("Failed to unmap memory!");
    }

    unsafe {
        close(fd);
    };
}

extern "C" {
    fn open(pathname: *const ffi::c_char, flags: ffi::c_int) -> ffi::c_int;
    fn close(fd: ffi::c_int);
    fn mmap(
        addr: *const ffi::c_void,
        length: isize,
        prot: ffi::c_int,
        flags: ffi::c_int,
        fd: ffi::c_int,
        offset: raw::off_t,
    ) -> *const ffi::c_void;
    fn munmap(addr: *const ffi::c_void, length: isize) -> ffi::c_int;
}
