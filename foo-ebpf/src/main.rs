#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;
use aya_bpf::macros::map;
use aya_bpf::{
    bindings::bpf_spin_lock,
    helpers::{
        bpf_spin_lock as bpf_spin_lock_lock,
        bpf_spin_unlock as bpf_spin_lock_unlock,
    },
    maps::Array,
};

#[map]
static Foo: Array<bpf_spin_lock> = Array::with_max_entries(1, 0);

#[xdp]
pub fn foo(ctx: XdpContext) -> u32 {
    match try_foo(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_foo(ctx: XdpContext) -> Result<u32, u32> {
    let x = unsafe {Foo.get_ptr_mut(0).ok_or(xdp_action::XDP_PASS)?.as_mut().unwrap()};
    unsafe {
        bpf_spin_lock_lock(x);
        bpf_spin_lock_unlock(x);
    }
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
