use std::mem::{self};

use common::mem::{get_raw_bytes, wrap_bytes};

#[no_mangle]
pub extern "C" fn allocate(size: u32) -> u32 {
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as u32
}
#[no_mangle]
pub extern "C" fn deallocate(pointer: u32, capacity: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer as *mut u8, 0, capacity as usize);
    }
}

#[no_mangle]
pub extern "C" fn greet(subject_pointer: u32) -> u32 {
    let subject: Vec<u8> = get_raw_bytes(subject_pointer as *const u8);
    let mut output = b"Hello, ".to_vec();
    output.extend(&subject);
    output.extend([b'!']);

    let wrapped_output = wrap_bytes(&output);

    let x = wrapped_output.as_ptr();
    x as u32
}

#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world!!!");
}
