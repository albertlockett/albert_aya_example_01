#![no_std]
#![no_main]

use aya_bpf::{
    macros::kprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;
use aya_log_ebpf::debug;

#[kprobe(name="hello")]
pub fn hello(ctx: ProbeContext) -> u32 {
    info!(&ctx, "Hello world");
    match unsafe { 
        debug!(&ctx, "Hello world");

        try_hello(ctx) 
    } {
        Ok(ret) => {
            
            ret
        },
        Err(ret) => ret,
    }
}

unsafe fn try_hello(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "Hello world 2");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
