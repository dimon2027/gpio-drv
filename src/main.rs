use std::ffi;

const O_RDWR: ffi::c_int = 2;
const O_SYNC: ffi::c_int = 0x4000000;

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

    unsafe {
        close(fd);
    };
}

extern "C" {
    fn open(pathname: *const ffi::c_char, flags: ffi::c_int) -> ffi::c_int;
    fn close(fd: ffi::c_int);
}
