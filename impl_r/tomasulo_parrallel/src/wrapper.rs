use {
    crate::atomic_queue::{AtomicQueue, QueueElem},
    std::{
        mem::MaybeUninit,
        sync::atomic::{
            AtomicU8, AtomicPtr, AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
        },
    },
};

#[derive(Debug)]
#[repr(AtomicU8)]
enum Status {
    Empty,
    Initialised,
    FilledSignaling,
    FilledResting,
} 

struct SingleFunctionWrapper<T> {
    function: fn(&T, *mut null) -> (),
    context: *mut null,
}

struct SingleDataWrapper<T> {
    data: MaybeUninit<T>,
    status: Status,
    waiters: Box<AtomicQueue<SingleFunctionWrapper<T>>>,
    active_users: AtomicUsize,
}

impl<T> SingleDataWrapper<T> {
    fn new() -> SingleDataWrapper<T> {
        SingleDataWrapper {
            data: MaybeUninit::uninit(),
            status: Status::Empty,
            waiters: AtomicQueue::new(),
            active_users: AtomicUsize::new(0),
        }
    }
    unsafe fn init_assuming_empty(self:&SingleDataWrapper<T>) {
        debug_assert_eq!(self.status,Status::Empty);
        self.status.into().swap(Status::Initialised);
    }

    
}

pub struct Wrapper<T> {
    data: [SingleDataWrapper<T>; 4],
    next: AtomicU8,
}

impl<T> Wrapper<T> {
    pub fn new_empty() -> Wrapper<T> {
        Wrapper {
            data: std::array::from_fn(|_| SingleDataWrapper::new()),
            next: 0.into(),
        }
    }
}
