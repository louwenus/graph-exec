use {
    crate::{
        atomic_queue::{AtomicQueue, QueueElem},
        wrapper::SingleDataWrapper,
    },
    ctor::ctor,
    pin_project::pin_project,
    std::{
        cell::UnsafeCell,
        hint::{cold_path, unreachable_unchecked},
        mem::{offset_of, transmute, MaybeUninit},
        pin::Pin,
        ptr::addr_of_mut,
        sync::atomic::{
            AtomicU8, AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
        },
        thread::yield_now,
    },
};

pub(crate) struct VirtualTask {
    callback: fn(*mut ()),
    counter: AtomicUsize,
}

type TaskContext<const N: u8> = [*const (); N as usize];

pub(crate) struct RealTask<const N: u8>
where
    [(); N as usize]:,
{
    task: QueueElem<VirtualTask>,
    context: TaskContext<N>,
}

impl VirtualTask {
    fn call(&self) {
        todo!();
    }
}

static WORK_QUEUE: AtomicQueue<VirtualTask> = unsafe { AtomicQueue::new_uninit() };

#[ctor]
fn initialise_wq() {
    WORK_QUEUE.init_valid();
}

pub fn one_arg_ready(task: &QueueElem<VirtualTask>) {
    let cnt = (*task).data.counter.fetch_sub(1, Release);
    if cnt == 1 {
        WORK_QUEUE.push(&*task);
    }
}

pub fn work_once() -> bool {
    let work = WORK_QUEUE.pop();
    if let Some(task) = work {
        task.data.call();
        return true;
    } else {
        return false;
    }
}

pub fn work_infinitely() {
    loop {
        while work_once() {}
        yield_now();
    }
}

fn cast_ptr<const N: u8>(ptr: *const VirtualTask) -> *const TaskContext<N> {
    unsafe {
        let ptr =
            ptr.byte_offset((offset_of!(RealTask<N>, task) as isize) * -1) as *const RealTask<N>;
        ptr.byte_offset(offset_of!(RealTask<N>, context) as isize) as *const TaskContext<N>
    }
}
