use crate::sheduler::{self, VirtualTask};
use pin_project::pin_project;

use {
    crate::{
        atomic_queue::{AtomicQueue, QueueElem},
        sheduler::{RealTask},
    },
    std::{
        cell::UnsafeCell,
        hint::{cold_path, unreachable_unchecked},
        mem::{transmute, MaybeUninit},
        pin::Pin,
        ptr::addr_of_mut,
        sync::atomic::{
            AtomicU8, AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
        },
    },
};

#[derive(Debug)]
#[repr(u8)]
enum Status {
    Empty,
    Initialised,
    FilledSignaling,
    FilledResting,
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        self as u8
    }
}
impl Into<AtomicU8> for Status {
    fn into(self) -> AtomicU8 {
        AtomicU8::new(self as u8)
    }
}

#[pin_project]
pub(crate) struct SingleDataWrapper<T,const N:u8> where
    [(); N as usize]: {
    pub(crate) data: UnsafeCell<MaybeUninit<T>>,
    status: AtomicU8, //Will be a "AtomicStatus"
    #[pin]
    waiters: AtomicQueue<VirtualTask>,
    active_users: AtomicUsize,
    task: RealTask<N>,
}

impl<T, const N:u8> SingleDataWrapper<T,N> where [(); N as usize]: {
    fn new() -> Pin<Box<Self>> {
        let mut new = Box::<MaybeUninit<Self>>::pin(MaybeUninit::uninit());
        unsafe {
            Self::init(new.as_mut().get_unchecked_mut().as_mut_ptr());
            transmute::<_, _>(new)
        }
    }

    fn init(ptr: *mut Self) {
        unsafe {
            addr_of_mut!((*ptr).data).write(UnsafeCell::new(MaybeUninit::uninit()));
            addr_of_mut!((*ptr).status).write(Status::Empty.into());
            AtomicQueue::<T>::init(Pin::new_unchecked(
                &mut *(addr_of_mut!((*ptr).waiters).cast::<MaybeUninit<AtomicQueue<T>>>()),
            ));
            addr_of_mut!((*ptr).active_users).write(AtomicUsize::new(0));
        }
    }

    fn init_assuming_empty(self: &Self) {
        debug_assert_eq!(self.status.load(Relaxed), Status::Empty.into());
        (&self.status as &AtomicU8).swap(Status::Initialised as u8, Relaxed);
    }

    fn effective_write(mut self: Pin<&mut Self>, val: T) {
        let this = self.as_mut().project();

        //safe because noone read as long as not signaled to do so (status and emptyqueue)
        unsafe { this.data.as_mut_unchecked().write(val) };

        self.status.store(Status::FilledSignaling.into(), Release);
        self.empty_queue::<false>();
        self.status.store(Status::FilledResting as u8, Release);
        self.empty_queue::<true>();
    }

    fn empty_queue<const COLD: bool>(self: &Self) {
        while let Some(fun) = (&self.waiters).pop() {
            if COLD {
                cold_path();
            }
            sheduler::one_arg_ready(fun);
        }
    }

    fn submit_reader(self: &Self, f: &QueueElem<VirtualTask>) {
        let status = unsafe { transmute::<_, Status>(self.status.load(Relaxed)) };
        match status {
            Status::Initialised => {
                self.active_users.fetch_add(1, Relaxed);
                self.waiters.push(f);
                loop {
                    match unsafe { transmute::<_, Status>(self.status.load(Acquire)) } {
                        Status::Initialised | Status::FilledSignaling => {
                            return;
                        }
                        Status::FilledResting => {
                            cold_path();
                            self.empty_queue::<true>();
                            return;
                        }
                        _ => unsafe {
                            unreachable_unchecked();
                        },
                    }
                }
            }
            Status::FilledSignaling | Status::FilledResting => {
                sheduler::one_arg_ready(f);
            }
            _ => unsafe {
                unreachable_unchecked();
            },
        }
    }
    pub(crate) fn signal_read_done(&self) {
        let cnt=self.active_users.fetch_sub(1, Relaxed);
        if cnt==1 {
            self.status.store(Status::Empty.into(), Release);
        }
    }
}

#[pin_project]
pub struct Wrapper<T,const N:u8=8> where [(); N as usize]: {
    #[pin]
    data: [UnsafeCell<SingleDataWrapper<T,N>>; 4],
    next: u8,
}

impl<T, const N:u8> Wrapper<T,N> where [(); N as usize]: {
    pub fn new_empty() -> Pin<Box<Self>> {
        let mut new: Pin<Box<MaybeUninit<Self>>> = Box::into_pin(Box::new_uninit());
        unsafe {
            Self::init_empty(new.as_mut().get_unchecked_mut().as_mut_ptr());
            transmute(new)
        }
    }
    pub unsafe fn init_empty(ptr: *mut Self) {
        unsafe {
            addr_of_mut!((*ptr).next).write(0.into());
            for i in 0..4 {
                SingleDataWrapper::<T,N>::init((*ptr).data[i].get());
            }
        }
    }
    pub(crate) fn submit_reader(self: &Self, f: &QueueElem<VirtualTask>) -> *const SingleDataWrapper<T,N> {
        unsafe {
            self.data[self.next as usize]
                .as_ref_unchecked()
                .submit_reader(f);
            self.data[self.next as usize].get()
        }
    }
    pub(crate) fn deffered_write(self: Pin<&mut Self>) -> *mut SingleDataWrapper<T,N> {
        let this = self.project();
        *this.next = (*this.next + 1) % 4;
        unsafe {
            while this.data[*this.next as usize]
                .as_ref_unchecked()
                .status
                .load(Acquire)
                != Status::Empty.into()
            {
                todo!();
            }
            this.data[*this.next as usize]
                .as_ref_unchecked()
                .init_assuming_empty();
        }
        this.data[*this.next as usize].get()
    }
}
