
use {
    atomic_queue,
    std::{
        sync::atomic::{
            AtomicPtr,
            AtomicChar,
            Ordering::{Acquire, Relaxed, Release},
        },
    },
};

struct SingleWrapper<T,U> {
  data: MaybeUnintialized<T>,
  status: AtomicChar,
  waiters: atomic_queue::AtomicQueue<U>,
}

pub struct Wrapper<T> {

    data: [SingleWrapper<T;fn(&T,*mut)->()>; 4]
}

impl<T> Wrapper<T> {
    pub fn new_empty() -> Wrapper<T> {
        Wrapper {
            is_init : false,
            data : 0
        }
    }
}
