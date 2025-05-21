use {
    crate::atomic_queue::{AtomicQueue, QueueElem},
    std::{
        mem::{offset_of, transmute, MaybeUninit},
        pin::Pin,
        ptr::addr_of_mut,
        str::Matches,
        sync::atomic::{
            AtomicPtr, AtomicU8, AtomicUsize,
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

struct SingleFunctionWrapper<T> {
    function: fn(&T, *mut ()) -> (),
    context: *mut (),
}

impl<T> SingleFunctionWrapper<T> {
    fn call(&self,t:&T) {
        (self.function)(t,self.context)
    }
}

struct SingleDataWrapper<T> {
    data: MaybeUninit<T>,
    status: AtomicU8, //Will be a "AtomicStatus"
    waiters: AtomicQueue<SingleFunctionWrapper<T>>,
    active_users: AtomicUsize,
}

impl<T> SingleDataWrapper<T> {
    fn new() -> Pin<Box<SingleDataWrapper<T>>> {
        let mut new = Box::<MaybeUninit<SingleDataWrapper<T>>>::pin(MaybeUninit::uninit());
        unsafe {
            Self::init(
                new.as_mut().get_unchecked_mut().as_mut_ptr() );
            transmute::<_, _>(new)
        }
    }

    fn init(ptr: *mut Self) {
        unsafe {
            addr_of_mut!((*ptr).data).write(MaybeUninit::uninit());
            addr_of_mut!((*ptr).status).write(Status::Empty.into());
            AtomicQueue::<T>::init(Pin::new_unchecked(
                &mut *(addr_of_mut!((*ptr).waiters).cast::<MaybeUninit<AtomicQueue<T>>>()),
            ));
            addr_of_mut!((*ptr).active_users).write(AtomicUsize::new(0));
        }
    }

    fn init_assuming_empty(self: Pin<&Self>) {
        debug_assert_eq!(self.status.load(Relaxed), Status::Empty.into());
        (&self.status as &AtomicU8).swap(Status::Initialised as u8, Relaxed);
    }

    fn effective_write(self: Pin<&mut Self>, val: T) {
            self.data.write(val);
            self.status.store(Status::FilledSignaling.into(), Release);
            while let Some(fun)=self.waiters.pop(){
                
            }
    }
}

pub struct Wrapper<T> {
    data: [SingleDataWrapper<T>; 4],
    next: AtomicU8,
}

impl<T> Wrapper<T> {
    pub fn new_empty() -> Pin<Box<Wrapper<T>>> {
        let mut new = Box::pin(MaybeUninit::uninit());
        unsafe {Self::init_empty(new.as_mut().get_unchecked_mut().as_mut_ptr());
         transmute(new) }
    }
    pub fn init_empty(ptr: *mut Wrapper<T>) {
        unsafe {
            addr_of_mut!((*ptr).next).write(0.into());
            for i in 0..4 {
                SingleDataWrapper::<T>::init(addr_of_mut!((*ptr).data[i]));
            }
        }
    }
}
