use std::{
    marker::PhantomPinned,
    mem::{offset_of, MaybeUninit},
    pin::Pin,
    ptr::null_mut,
    sync::atomic::{
        AtomicIsize, AtomicPtr,
        Ordering::{AcqRel, Acquire, Relaxed, Release},
    },
};

pub struct QueueElem<T> {
    next: AtomicPtr<QueueElem<T>>,
    pub data: T,
}
impl<T> QueueElem<T> {
    pub fn new(elem: T) -> QueueElem<T> {
        QueueElem {
            next: AtomicPtr::new(null_mut()),
            data: elem,
        }
    }
}

pub struct AtomicQueue<T> {
    head: AtomicPtr<QueueElem<T>>,
    tail: AtomicPtr<AtomicPtr<QueueElem<T>>>,
    counter: AtomicIsize,
    _pin: PhantomPinned,
}

unsafe fn ptr_to_ref<'a, T>(ptr: *mut T) -> Pin<&'a mut MaybeUninit<T>> {
    // Cast to *mut MaybeUninit<T>
    let mu_ptr = ptr.cast::<MaybeUninit<T>>();
    // Convert to &mut MaybeUninit<T>
    let mu_ref = &mut *mu_ptr;
    // Create Pin (unsafe because we assume pinning holds)
    Pin::new_unchecked(mu_ref)
}

impl<T> AtomicQueue<T> {
    pub fn new() -> Pin<Box<AtomicQueue<T>>> {
        let mut new = Box::<AtomicQueue<T>>::new_uninit();
        unsafe {
            Self::init(ptr_to_ref(new.as_mut_ptr()));
            Box::into_pin(new.assume_init())
        }
    }

    pub unsafe fn init(pinned: Pin<&mut MaybeUninit<Self>>) {
        // Compute the `tail` pointer **before** moving `pinned`
        let tail_ptr = pinned
            .as_ref() // Borrow `pinned` immutably here
            .as_ptr()
            .byte_offset(offset_of!(Self, head) as isize)
            as *mut AtomicPtr<QueueElem<T>>;

        // Now move `pinned` into `get_unchecked_mut`
        pinned.get_unchecked_mut().write(AtomicQueue {
            head: AtomicPtr::new(null_mut()),
            tail: AtomicPtr::new(tail_ptr), // Use precomputed pointer
            counter: 0.into(),
            _pin: PhantomPinned,
        });
    }

    pub fn push<'a, 'b>(self: Pin<&'a Self>, element: &'b mut QueueElem<T>)
    where
        'b: 'a,
    {
        element.next.store(null_mut(), Relaxed);
        let pointer_to_element = element as *mut _;
        let double_pointer_to_elt = &raw mut element.next;
        let old = self.tail.swap(double_pointer_to_elt, AcqRel);
        unsafe {
            (*old).store(pointer_to_element, Release);
        }
        self.counter.fetch_add(1, Release);
    }
    pub fn pop(self: Pin<&Self>) -> Option<&mut QueueElem<T>> {
        let count = self.counter.fetch_sub(1, Acquire);
        if count <= 0 {
            if self.counter.fetch_add(1, Release) < 0 {
                return None;
            } else {
                return self.pop();
            }
        } else {
            let tmp = self.head.load(Acquire);
            unsafe {
                let mut next = (*tmp).next.load(Acquire);
                while next == null_mut() {
                    let old = self.tail.compare_exchange(
                        &raw mut (*tmp).next,
                        &raw const self.head as *mut _,
                        Relaxed,
                        Relaxed,
                    );
                    if old.is_ok() {
                        //compare in case already modified by write to the tail
                        let _ = self
                            .head
                            .compare_exchange(tmp, null_mut(), Relaxed, Relaxed);
                        return Some(&mut *(tmp as *mut QueueElem<T>));
                    }
                    std::thread::yield_now();
                    next = (*(tmp as *mut QueueElem<T>)).next.load(Acquire);
                }
                self.head.store(next, Relaxed);
                Some(&mut *(tmp as *mut QueueElem<T>))
            }
        }
    }
}
