use std::{
    mem::offset_of,
    ptr::null_mut,
    sync::atomic::{
        AtomicPtr,
        Ordering::{Acquire, Relaxed, Release},
    },
};

pub struct QueueElem<T> {
    next: AtomicPtr<QueueElem<T>>,
    pub data: T,
}
impl<T> QueueElem<T> {
    pub fn new(elem:T) -> QueueElem<T> {
        QueueElem {
            next: AtomicPtr::new(null_mut()),
            data: elem
        }
    }
}

pub struct AtomicQueue<T> {
    head: AtomicPtr<QueueElem<T>>,
    tail: AtomicPtr<AtomicPtr<QueueElem<T>>>,
}

impl<T> AtomicQueue<T> {
    pub fn new() -> Box<AtomicQueue<T>> {
        let mut n = Box::<AtomicQueue<T>>::new_uninit();
        unsafe {
            //used as *const, but there is no AtomicConstPtr (that I know of)
            n.as_mut_ptr().write(AtomicQueue {
                head: AtomicPtr::new(null_mut()),
                tail: AtomicPtr::new(
                    n.as_ptr()
                        .byte_offset(offset_of!(AtomicQueue<T>, head) as isize)
                        as *mut AtomicPtr<QueueElem<T>>,
                ),
            });

            n.assume_init()
        }
    }

    pub fn push<'a, 'b>(&'a self, element: &'b mut QueueElem<T>) where 'b: 'a {
        element.next.store(null_mut(), Relaxed);
        let nptr = element as *mut _;
        let npptr = &raw mut element.next;
        let old = self.tail.swap(npptr, Release);
        unsafe {
            (*old).store(nptr, Release);
        }
    }
    pub fn pop(&self) -> Option<&mut QueueElem<T>> {
        let tmp = self.head.load(Acquire);
        if tmp == null_mut() {
            return None;
        } else {
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
