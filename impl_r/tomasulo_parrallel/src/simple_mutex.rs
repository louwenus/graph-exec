use libc;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicU32,
        Ordering::{Acquire, Relaxed, Release},
    },
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
    pub const fn init() -> Self {
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

pub struct SimpleMutexRust<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Send for SimpleMutexRust<T> {}
unsafe impl<T> Sync for SimpleMutexRust<T> {}

impl<T> SimpleMutexRust<T> {
    pub const fn init(value: T) -> Self {
        SimpleMutexRust {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }
    pub fn lock(&self) -> SimpleMutexRustGuard<T> {
        if (&self.state)
            .compare_exchange(0, 1, Acquire, Relaxed)
            .is_ok()
        {
            return SimpleMutexRustGuard::new(self);
        } else {
            loop {
                let tmp: u32 = (&self.state).swap(2, Acquire);
                if tmp == 0 {
                    return SimpleMutexRustGuard::new(self);
                }
                unsafe {
                    let _ = futex_wait(&raw const self.state, 2);
                }
            }
        }
    }
    fn unlock(&self) {
        let tmp: u32 = (&self.state).swap(0, Release);
        if tmp != 1 {
            unsafe {
                futex_wake(&raw const self.state, 1);
            }
        }
    }
}

pub struct SimpleMutexRustGuard<'a, T> {
    mutex: &'a SimpleMutexRust<T>,
}

impl<'a, T> SimpleMutexRustGuard<'a, T> {
    pub fn new(mutex: &'a SimpleMutexRust<T>) -> Self {
        Self { mutex }
    }
}

impl<'a, T> Deref for SimpleMutexRustGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.mutex.value.as_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for SimpleMutexRustGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.mutex.value.as_mut_unchecked() }
    }
}

impl<'a, T> Drop for SimpleMutexRustGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
