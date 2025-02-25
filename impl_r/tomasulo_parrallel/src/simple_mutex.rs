use libc;
use std::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};

unsafe fn futex_wait(mux: *const AtomicU32, expected: u32) -> () {
    let _ = libc::syscall(
        libc::SYS_futex,
        mux,
        libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
        expected,
        0,
        0,
        0,
    );
}
unsafe fn futex_wake(mux: *const AtomicU32, num: u32) -> () {
    let _ = libc::syscall(
        libc::SYS_futex,
        mux,
        libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
        num,
        0,
        0,
        0,
    );
}

pub struct SimpleMutex {
    state: AtomicU32,
}

impl SimpleMutex {
    pub const fn init() -> SimpleMutex {
        SimpleMutex {
            state: AtomicU32::new(0),
        }
    }
    pub fn lock(&self) -> () {
        if (&self.state)
            .compare_exchange(0, 1, Acquire, Relaxed)
            .is_ok()
        {
            return;
        } else {
            loop {
                let tmp: u32 = (&self.state).swap(2, Acquire);
                if tmp == 0 {
                    return;
                }
                unsafe {
                    let _ = futex_wait(&raw const self.state, 2);
                }
            }
        }
    }
    pub fn unlock(&self) -> () {
        let tmp: u32 = (&self.state).swap(0, Release);
        if tmp != 1 {
            unsafe {
                futex_wake(&raw const self.state, 1);
            }
        }
    }
}
