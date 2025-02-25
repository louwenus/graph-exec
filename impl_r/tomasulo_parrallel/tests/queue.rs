use std::{
    ptr::null_mut,
    sync::atomic::{
        AtomicPtr, AtomicU64, AtomicUsize, Ordering::{Acquire, Relaxed, Release}
    },
};
use tomasulo_parrallel::atomic_queue::{AtomicQueue, QueueElem};
use std::thread;

const NUM_THREAD: i8 = 5;
const NUM_ITERATION: i64 = 1000000;

static CNT : AtomicUsize = AtomicUsize::new(0);



fn pusher(queue : &AtomicQueue<i64>, & elems:&Vec<QueueElem<i64>>) {
    let mut i = 0;
    while i<NUM_ITERATION {
        let cur = CNT.fetch_add(1, Relaxed);
        queue.push(&mut (elems[cur]));
        i+=1;
    }
}

fn collect_q(queue : &AtomicQueue<i64>){
    let mut i = 0_i64;
    while i < NUM_ITERATION*NUM_THREAD as i64 {
        let mut ret = None;
        while ret.is_none() {
            ret = queue.pop();
        }
        unsafe {
            let elem = ret.unwrap_unchecked();
            assert_ne!(elem.data,-1);
            elem.data = -1;
        }
        i+=1;
    }
}

#[test]
fn concurent_usage() -> () {
    let queue = AtomicQueue::<i64>::new();
    let mut elems :Vec<_>= (0..NUM_ITERATION*NUM_THREAD as i64).map(
        |i| {
            QueueElem::new(i)
        }
    ).collect();
    let collector = thread::spawn(move || {
        collect_q(queue.as_ref());
    });
    let pushers :Vec<_> = (0..NUM_THREAD).map(
        |_| {
            thread::spawn(move || {
                pusher(queue.as_ref(), & elems);
            })
        }
    ).collect();
}
