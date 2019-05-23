use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}


