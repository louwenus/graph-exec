use std::thread;
use tomasulo_parrallel::simple_mutex::SimpleMutexRust;

const NUM_THREAD: i8 = 100;
const NUM_ITERATION: i64 = 1000000;

static FUTEX: SimpleMutexRust<i64> = SimpleMutexRust::init(0);

fn counter_thread() {
    let mut i = 0;
    while i < NUM_ITERATION {
        *FUTEX.lock() += 1;
        i += 1;
    }
}

#[test]
fn increment_protected() {
    let threads: Vec<_> = (0..NUM_THREAD)
        .map(|_| {
            thread::spawn(move || {
                counter_thread();
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }
    assert_eq!(*FUTEX.lock(), NUM_ITERATION * NUM_THREAD as i64);
}
