use std::ffi;
use std::os::unix::raw;

const O_RDWR: ffi::c_int = 2;
const O_SYNC: ffi::c_int = 0x4000000;
const PERIPH: ffi::c_int = 0x3f000000; // for BCM2835 processor
const GPIO_OFFSET: ffi::c_int = 0x200000;
const PROT_READ: ffi::c_int = 0x1;
const PROT_WRITE: ffi::c_int = 0x2;
const PROT_RDWR: ffi::c_int = PROT_READ | PROT_WRITE;
const PAGE_SIZE: isize = 4096;

const MAP_SHARED: ffi::c_int = 0x01;

const GPIO_MEM_DEV: &str = "/dev/gpiomem";

// TODO: thread safety
// TODO: error handling
// TODO: turn this into a module
// TODO: stop using deprecated types, switch to clib crate
// TODO: implement safe manipulation of raw mapped address (what if one calls close() and then tries to set GPIO function?)
enum PinFunction {
    Input,
    Output,
}

enum PinLevel {
    Low,
    High,
}

struct GPIO {
    fd: i32,
    base_addr: *const ffi::c_void,
}

impl GPIO {
    fn new() -> GPIO {
        GPIO {
            fd: -1,
            base_addr: 0 as *const ffi::c_void,
        }
    }

    fn init(&mut self) -> bool {
        let res = ffi::CString::new(GPIO_MEM_DEV);
        if !res.is_ok() {
            println!("Error creating a CString");
            return false;
        }

        let pathname = res.unwrap();

        self.fd = unsafe { open(pathname.as_ptr(), O_RDWR | O_SYNC) };
        println!("fd = {}", self.fd);

        let base_addr = unsafe {
            mmap(
                0 as *const ffi::c_void,
                PAGE_SIZE,
                PROT_RDWR,
                MAP_SHARED,
                self.fd,
                (PERIPH + GPIO_OFFSET) as raw::off_t,
            )
        };

        if (base_addr as isize) < 0 {
            println!("Failed to map memory!");
            return false;
        }

        self.base_addr = base_addr;

        println!("Base address = {}", base_addr as isize);

        true
    }
    fn close(&mut self) -> bool {
        let res = unsafe { munmap(self.base_addr, PAGE_SIZE) };
        if res < 0 {
            println!("Failed to unmap memory!");
            return false;
        }

        self.base_addr = 0 as *const ffi::c_void;

        unsafe {
            close(self.fd);
        };

        self.fd = -1;

        true
    }

    fn set_function(&self, pin: u8, func: PinFunction) {
        let reg_num = pin / 10;
        let bits_group_num = pin % 10;
        let reg_addr: usize = (self.base_addr as usize) + 0x04 * reg_num as usize;
        let val = unsafe { *(reg_addr as *const u32) };
        let val = val & (!(0b111u32 << (3 * bits_group_num)));

        let val = match func {
            PinFunction::Output => val | (0b001u32 << (3 * bits_group_num)),
            PinFunction::Input => val,
        };

        unsafe {
            *(reg_addr as *mut u32) = val;
        }
    }

    fn set_level(&self, pin: u8, level: PinLevel) {
        let reg_num = pin / 32;
        let bit_num: u8 = pin % 32;
        let reg_addr: usize = match level {
            PinLevel::High => self.base_addr as usize + 0x1C + 0x4 * reg_num as usize,
            PinLevel::Low => self.base_addr as usize + 0x28 + 0x4 * reg_num as usize,
        };

        let val = unsafe { *(reg_addr as *const u32) };
        let val = val | (1 << bit_num);

        unsafe {
            *(reg_addr as *mut u32) = val;
        }
    }

    fn get_level(&self, pin: u8) -> PinLevel {
        let reg_num = pin / 32;
        let bit_num: u8 = pin % 32;
        let reg_addr: usize = self.base_addr as usize + 0x34 + 0x4 * reg_num as usize;

        let val = unsafe { *(reg_addr as *const u32) };
        let val = val & (1 << bit_num);

        match val {
            0 => PinLevel::Low,
            _ => PinLevel::High,
        }
    }
}

fn main() {
    let mut gpio = GPIO::new();
    let res = gpio.init();
    if !res {
        return;
    }

    //gpio.set_function(7, PinFunction::Output);
    //gpio.set_level(7, PinLevel::Low);

    let level = gpio.get_level(7);
    match level {
        PinLevel::Low => println!("Pin 7 is Low"),
        PinLevel::High => println!("Pin 7 is High"),
    }

    let res = gpio.close();
    if !res {
        println!("Failed to close gpio");
    }
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
