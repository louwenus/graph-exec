use {
    crate::atomic_queue::{AtomicQueue, QueueElem},
    std::{
        mem::{offset_of, MaybeUninit},
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

struct SingleDataWrapper<T> {
    data: MaybeUninit<T>,
    status: AtomicU8, //Will be a "AtomicStatus"
    waiters: AtomicQueue<SingleFunctionWrapper<T>>,
    active_users: AtomicUsize,
}

unsafe fn ptr_to_ref<'a, T>(ptr: *mut T) -> Pin<&'a mut MaybeUninit<T>> {
    // Cast to *mut MaybeUninit<T>
    let mu_ptr = ptr.cast::<MaybeUninit<T>>();
    // Convert to &mut MaybeUninit<T>
    let mu_ref = &mut *mu_ptr;
    // Create Pin (unsafe because we assume pinning holds)
    Pin::new_unchecked(mu_ref)
}

impl<T> SingleDataWrapper<T> {
    fn new() -> Pin<Box<SingleDataWrapper<T>>> {
        let mut new = Box::<MaybeUninit<SingleDataWrapper<T>>>::new(MaybeUninit::uninit());
        unsafe {
            Self::init(ptr_to_ref(new.as_mut_ptr()));
            Box::into_pin(new.assume_init())
        }
    }

    fn init(pinned: Pin<&mut MaybeUninit<Self>>) {
        unsafe {
            let ptr = pinned.get_unchecked_mut().as_mut_ptr();

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
        unsafe {
            let s = self.get_unchecked_mut();
            s.data.write(val);
            s.status.store(Status::FilledSignaling.into(), Release);
        }
    }
}

pub struct Wrapper<T> {
    data: [SingleDataWrapper<T>; 4],
    next: AtomicU8,
}

impl<T> Wrapper<T> {
    pub fn new_empty() -> Pin<Box<Wrapper<T>>> {
        let mut new = Box::new(MaybeUninit::uninit());
        unsafe {Self::init_empty(ptr_to_ref(new.as_mut_ptr()));
        Box::into_pin(new.assume_init())}
    }
    pub fn init_empty(pinned: Pin<&mut MaybeUninit<Wrapper<T>>>) {
        unsafe{ let ptr = pinned.get_unchecked_mut().as_mut_ptr() ;
            addr_of_mut!((*ptr).next).write(0.into());
            for i in 0..4 {
                SingleDataWrapper::<T>::init( ptr_to_ref( addr_of_mut!((*ptr).data[i])));
            }
        }
    }
}
