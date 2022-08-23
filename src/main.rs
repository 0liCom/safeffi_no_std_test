#![no_main]
#![no_std]

use core::panic::PanicInfo;
use libc::{c_int, exit, printf};

use example_no_std_crate;
use example_no_std_crate::{convert, SomeStruct, spawn_struct};

extern crate libc;

extern "C" {
    #[allow(unused)]
    fn lshift_32(dest: *mut u32, a: u32);
    #[allow(unused)]
    fn sum_32(vec: *mut i32, len: usize) -> i32;
    fn sum_from_struct_32(array_struct: *const SomeStruct) -> i32;
}

#[no_mangle]
fn main() -> () {
    unsafe {
        printf("start\n\0".as_ptr() as *const i8);
    }

    let src: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];
    let mut dest: [u8; 16] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f,
    ];

    unsafe {
        printf(
            "created objects src at %p and dest at %p\n\0".as_ptr() as *const i8,
            src.as_ptr(),
            dest.as_ptr(),
        );
        print_array("src\0", &src);
        print_array("dest\0", &dest);
    }

    example_no_std_crate::copy_prefix(&mut dest, &src, 8);
    unsafe {
        printf("copied prefix\n\0".as_ptr() as *const i8);
        print_array("src\0", &src);
        print_array("dest\0", &dest);
    }

    let cmp = example_no_std_crate::cmp(&src, &dest);
    unsafe {
        printf("equal with half copy: %b\n\0".as_ptr() as *const i8, cmp as c_int);
    }

    example_no_std_crate::copy_prefix(&mut dest, &src, 16);
    let cmp = example_no_std_crate::cmp(&src, &dest);
    unsafe {
        printf("equal after full copy: %b\n\0".as_ptr() as *const i8, cmp as c_int);
    }

    let data = convert(&dest);
    let data_struct = spawn_struct(data);

    unsafe {
        let sum = sum_from_struct_32(&data_struct as *const SomeStruct);

        printf("sum: %d\n\0".as_ptr() as *const i8, sum);
        exit(0);
    }
}

fn print_array<T: Copy, const N: usize>(name: &str, array: &[T; N]) {
    unsafe {
        printf(name.as_ptr() as *const i8);
        printf(":\0".as_ptr() as *const i8);

        for i in 0..array.len() {
            printf(" %x\0".as_ptr() as *const i8, *(&array[i] as *const T));
        }
        printf("\n\0".as_ptr() as *const i8);
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
