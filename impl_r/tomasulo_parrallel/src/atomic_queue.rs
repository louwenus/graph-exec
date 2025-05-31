use pin_project::pin_project;
use std::{
    hint::{likely, unlikely},
    mem::{offset_of, transmute, MaybeUninit},
    pin::Pin,
    ptr::null_mut,
    sync::atomic::{
        AtomicIsize, AtomicPtr,
        Ordering::{SeqCst, AcqRel, Acquire, Relaxed, Release},
    },
};
/// A single element of the queue, holding data and a pointer to the next element.
pub struct QueueElem<T> {
    pub(crate) next: AtomicPtr<QueueElem<T>>,
    pub data: T,
}

impl<T> QueueElem<T> {
    /// Creates a new queue element wrapping the provided data.
    pub fn new(elem: T) -> QueueElem<T> {
        QueueElem {
            next: AtomicPtr::new(null_mut()),
            data: elem,
        }
    }
}

/// A lock-free, thread-safe queue allowing multiple concurrent producers and consumers.
#[pin_project(!Unpin)]
pub struct AtomicQueue<T> {
    head: AtomicPtr<QueueElem<T>>,
    pub(crate) tail: AtomicPtr<AtomicPtr<QueueElem<T>>>,
    counter: AtomicIsize,
}

impl<T> AtomicQueue<T> {
    /// Constructs a new, empty `AtomicQueue` wrapped in a pinned box.
    pub fn new() -> Pin<Box<Self>> {
        let mut new = Box::into_pin(Box::<Self>::new_uninit());
        Self::init(new.as_mut());
        unsafe { transmute(new) }
    }

    /// Initializes a queue in place on uninitialized memory.
    pub fn init(pinned: Pin<&mut MaybeUninit<Self>>) {
        unsafe {
            let head_ptr = pinned
                .as_ref()
                .as_ptr()
                .byte_offset(offset_of!(Self, head) as isize)
                as *mut AtomicPtr<QueueElem<T>>;

            pinned.get_unchecked_mut().write(Self {
                head: AtomicPtr::new(null_mut()),
                tail: AtomicPtr::new(head_ptr),
                counter: 0.into(),
            });
        }
    }
    pub const unsafe fn new_uninit() -> AtomicQueue<T> {
        AtomicQueue {
            head: AtomicPtr::new(null_mut()),
            tail: AtomicPtr::new(null_mut()),
            counter: AtomicIsize::new(0),
        }
    }
    pub(crate) fn init_valid(self:&Self){
        self.tail.store(
                    (&raw const self.head) as *mut _
            , SeqCst);
    }
    /// Pushes a queue element onto the queue.
    /// Safe to call from multiple threads without additional synchronization.
    pub fn push<'a, 'b>(self: &'a Self, element: &'b QueueElem<T>)
    where
        'b: 'a,
    {
        element.next.store(null_mut(), Relaxed);
        let pointer_to_element = element as *const _;
        let double_pointer_to_elt = &raw const element.next;
        let old = self.tail.swap(double_pointer_to_elt as *mut _, AcqRel);
        unsafe {
            (*old).store(pointer_to_element as *mut _, Relaxed);
        }
        self.counter.fetch_add(1, Release);
    }

    /// Pops an element from the queue, or returns `None` if the queue is empty.
    /// Safe to call from multiple threads without additional synchronization.
    pub fn pop(self: &Self) -> Option<&QueueElem<T>> {
        if self.counter.fetch_sub(1, Acquire) <= 0 {
            self.counter.fetch_add(1, Relaxed);
            return None;
        }

        let mut tmp = self.head.swap(null_mut(), Relaxed);
        while unlikely(tmp == null_mut()) {
            std::thread::yield_now();
            tmp = self.head.swap(null_mut(), Relaxed);
        }

        let mut next = unsafe { (*tmp).next.load(Relaxed) };
        while unlikely(next == null_mut()) {
            let old_tail = unsafe {
                self.tail.compare_exchange(
                    &raw mut (*tmp).next,
                    &raw const self.head as *mut _,
                    Relaxed,
                    Relaxed,
                )
            };
            if old_tail.is_ok() {
                unsafe {
                    return Some(&mut *(tmp as *mut QueueElem<T>));
                };
            }
            std::thread::yield_now();
            next = unsafe { (*tmp).next.load(Relaxed) };
        }
        self.head.store(next, Relaxed);
        unsafe {
            return Some(& *(tmp as *const QueueElem<T>));
        };
    }
}
