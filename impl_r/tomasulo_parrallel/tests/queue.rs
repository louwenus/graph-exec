use libc::fflush;
use std::io::{self, Stdout, Write};
use std::thread;
use std::{
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};
use tomasulo_parrallel::atomic_queue::{AtomicQueue, QueueElem};

const NUM_THREAD: i8 = 8;
const NUM_ITERATION: i64 = 5000000;
const NUM_ELT: i8 = 4;

static CNT: AtomicUsize = AtomicUsize::new(0);

fn pusher(queue: Pin<&AtomicQueue<i64>>, elems: &Test) {
    let mut i = 0;
    while i < NUM_ITERATION {
        let cur = CNT.fetch_add(1, Relaxed);
        unsafe { queue.push(&mut ((*elems.0)[cur])) };
        i += 1;
    }
}

fn collect_q(queue: Pin<&AtomicQueue<i64>>) {
    let mut i = 0_i64;
    while i < NUM_ITERATION as i64 {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.pop();
        }
        unsafe {
            let elem = ret.unwrap_unchecked();
            assert_ne!(elem.data, -1);
            elem.data = -1;
        }
        i += 1;
    }
}

#[derive(Clone, Copy)]
struct Test(*mut [QueueElem<i64>]);
unsafe impl Sync for Test {}
unsafe impl Send for Test {}

#[test]
fn concurent_usage() -> () {
    let queue = AtomicQueue::<i64>::new();
    let elems: Vec<_> = (0..NUM_ITERATION * NUM_THREAD as i64)
        .map(|_| QueueElem::new(1))
        .collect();
    let qref: Pin<&AtomicQueue<i64>> =
        unsafe { Pin::new_unchecked(Box::leak(Pin::into_inner_unchecked(queue))) };
    let eref = Test(Vec::leak(elems));
    let collectors: Vec<_> = (0..NUM_THREAD)
        .map(|_| {
            thread::spawn(move || {
                collect_q(qref);
            })
        })
        .collect();
    let pushers: Vec<_> = (0..NUM_THREAD)
        .map(|_| {
            thread::spawn(move || {
                pusher(qref, &eref);
            })
        })
        .collect();
    for collector in collectors {
        collector.join().unwrap();
    }
    for pusher in pushers {
        pusher.join().unwrap();
    }
    let _ = (0..NUM_ITERATION * NUM_THREAD as i64).for_each(|i| unsafe {
        assert_eq!(-1, (*eref.0)[i as usize].data);
    });
    unsafe {
        //let _edropper = Vec::from_raw(eref.0);
        let _qdropper =
            Box::from_raw(Pin::into_inner_unchecked(qref) as *const _ as *mut AtomicQueue<i64>);
    }
}

fn push_pop(queue: Pin<&AtomicQueue<i64>>) {
    let mut i = 0_i64;
    while i < NUM_ITERATION as i64 {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.pop();
        }
        unsafe {
            let elem = ret.unwrap_unchecked();
            queue.push(elem);
        }
        i += 1;
    }
}

#[test]
fn few_elems_check() {
    let queue = AtomicQueue::<i64>::new();
    let elems: Vec<_> = (0..NUM_ELT as i64)
        .map(|_| QueueElem::new(1 as i64))
        .collect();
    let qref: Pin<&AtomicQueue<i64>> =
        unsafe { Pin::new_unchecked(Box::leak(Pin::into_inner_unchecked(queue))) };
    let eref = Vec::leak(elems);
    for i in 0..NUM_ELT {
        qref.push(&mut eref[i as usize]);
    }
    let ths: Vec<_> = (0..NUM_THREAD)
        .map(|_| {
            thread::spawn(move || {
                push_pop(qref);
            })
        })
        .collect();

    for th in ths {
        th.join().unwrap();
    }
    for _ in 0..NUM_ELT as i8 {
        let next = qref.pop().expect("Not enough elements got out");
        assert_eq!(1, next.data);
        next.data = -1;
    }
    unsafe {
        //let _edropper = Vec::from_raw(eref.0);
        let _qdropper =
            Box::from_raw(Pin::into_inner_unchecked(qref) as *const _ as *mut AtomicQueue<i64>);
    }
}
