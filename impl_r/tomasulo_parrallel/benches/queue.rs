#![feature(test)]

extern crate test;
use test::Bencher;

use libc::fflush;
use std::sync::LazyLock;
use std::thread;
use std::{
    collections::VecDeque,
    io::{self, Stdout, Write},
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use std::sync::{Arc, Mutex};

use tomasulo_parrallel::atomic_queue::{AtomicQueue, QueueElem};

const NUM_THREAD: i8 = 1;
const NUM_ITERATION: i64 = 5000;
const NUM_ELT: i8 = 2;

use threadpool::ThreadPool;

fn push_pop_q_b(queue: Pin<&AtomicQueue<i64>>) {
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

static POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPool::new(2 * NUM_THREAD as usize));

#[bench]
fn bench_ato_q(b: &mut Bencher) {
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

    b.iter(|| {
        for _ in 0..NUM_THREAD {
            POOL.execute(move || {
                push_pop_q_b(qref);
            })
        }
        POOL.join()
    });

    unsafe {
        //let _edropper = Vec::from_raw(eref.0);
        let _qdropper =
            Box::from_raw(Pin::into_inner_unchecked(qref) as *const _ as *mut AtomicQueue<i64>);
    }
}

fn thread_push(queue: Arc<Mutex<VecDeque<i64>>>) {
    for _ in 0..NUM_ITERATION {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.lock().unwrap().pop_front();
        }
        queue.lock().unwrap().push_back(ret.unwrap());
    }
}

#[bench]
pub fn bench_mutex_queue(b: &mut Bencher) {
    let mut queue = VecDeque::with_capacity(NUM_ELT as usize);
    (0..NUM_ELT).for_each(|i| queue.push_back(i as i64));

    let queue = Arc::new(Mutex::new(queue));

    b.iter(|| {
        (0..NUM_THREAD).for_each(|_| {
            let queue_c = Arc::clone(&queue);
            POOL.execute(|| thread_push(queue_c))
        });
        POOL.join();
    })
}

static CNT: AtomicUsize = AtomicUsize::new(0);



fn pusher_ato(queue: Pin<&AtomicQueue<i64>>, elems: &Test) {
    for _ in 0..NUM_ITERATION {
        let cur = CNT.fetch_add(1, Relaxed);
        unsafe { queue.push(&mut ((*elems.0)[cur])) };
    }
}

fn collect_ato(queue: Pin<&AtomicQueue<i64>>) {
    for _ in 0..NUM_ITERATION * NUM_THREAD as i64 {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.pop();
        }
        unsafe {
            let elem = ret.unwrap_unchecked();
        }
    }
}

fn pusher_mux(queue: Arc<Mutex<VecDeque<i64>>>) {
    for _ in 0..NUM_ITERATION {
        let cur = CNT.fetch_add(1, Relaxed) as i64;
        queue.lock().unwrap().push_back(cur);
    }
}

fn poper_mux(queue: Arc<Mutex<VecDeque<i64>>>) {
    for _ in 0..(NUM_ITERATION* (NUM_THREAD as i64)) {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.lock().unwrap().pop_front();
        }
    }
}

#[derive(Clone, Copy)]
struct Test(*mut [QueueElem<i64>]);
unsafe impl Sync for Test {}
unsafe impl Send for Test {}

#[bench]
fn ato_one_pop(b: &mut Bencher) -> () {
    let queue = AtomicQueue::<i64>::new();
    let elems: Vec<_> = (0..NUM_ITERATION * NUM_THREAD as i64)
        .map(|_| QueueElem::new(1))
        .collect();
    let qref: Pin<&AtomicQueue<i64>> =
        unsafe { Pin::new_unchecked(Box::leak(Pin::into_inner_unchecked(queue))) };
    let eref = Test(Vec::leak(elems));
    b.iter(|| {
        CNT.store(0,Relaxed);
        POOL.execute(move || {
            collect_ato(qref);
        });
        (0..NUM_THREAD).for_each(|_| {
            POOL.execute(move || {
                pusher_ato(qref, &eref);
            })
        });

        POOL.join();
    });
    unsafe {
        //let _edropper = Vec::from_raw(eref.0);
        let _qdropper =
            Box::from_raw(Pin::into_inner_unchecked(qref) as *const _ as *mut AtomicQueue<i64>);
    }
}

#[bench]
pub fn mutex_one_pop(b: &mut Bencher) {
    let queue = VecDeque::with_capacity(NUM_ITERATION as usize*NUM_THREAD as usize);

    let queue = Arc::new(Mutex::new(queue));

    b.iter(|| {
        CNT.store(0,Relaxed);
        let queue_c = Arc::clone(&queue);
        POOL.execute(|| poper_mux(queue_c));
        (0..NUM_THREAD).for_each(|_| {
            let queue_c = Arc::clone(&queue);
            POOL.execute(|| pusher_mux(queue_c))
        });
        POOL.join();
    })
}
