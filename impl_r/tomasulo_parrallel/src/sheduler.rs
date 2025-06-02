use {
    crate::{
        atomic_queue::{AtomicQueue, QueueElem},
        wrapper::{SingleDataWrapper, Wrapper},
    },
    ctor::ctor,
    std::{
        mem::{offset_of, MaybeUninit}, sync::atomic::{
            AtomicUsize,
            Ordering::Release,
        }, thread::yield_now
    },
};

pub(crate) struct VirtualTask {
    callback: fn(*const Self),
    counter: AtomicUsize,
}

union RawData {
    ptr: *const (),
    value: MaybeUninit<usize>,
}

type TaskContext<const N: u8> = [RawData; N as usize];

#[repr(C)]
pub(crate) struct RealTask<const N: u8>
where
    [(); N as usize]:,
{
    task: QueueElem<VirtualTask>,
    context: TaskContext<N>,
}

impl VirtualTask {
    fn call(&self) {
        (self.callback)(self as *const VirtualTask)
    }
}

static WORK_QUEUE: AtomicQueue<VirtualTask> = unsafe { AtomicQueue::new_uninit() };

#[ctor]
fn initialise_wq() {
    WORK_QUEUE.init_valid();
}

pub(crate) fn one_arg_ready(task: &QueueElem<VirtualTask>) {
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

trait TorWrapperT<T> {
    fn prepare_read(&self, task: &QueueElem<VirtualTask>) -> RawData;
    unsafe fn read(data: RawData) -> *const T;
    unsafe fn liberate(data: RawData);
    const TASK_WAIT: bool;
}

impl<T, const N: u8> TorWrapperT<T> for Wrapper<T, N>
where
    [(); N as usize]:,
{
    fn prepare_read(&self, task: &QueueElem<VirtualTask>) -> RawData {
        let ptr = self.submit_reader(task);
        RawData { ptr: ptr as _ }
    }
    ///ptr must have been obtained by prepare read
    unsafe fn read(data: RawData) -> *const T {
        let ptr = data.ptr as *const SingleDataWrapper<T, N>;
        unsafe { (*ptr).data.as_ref_unchecked().as_ptr() }
    }
    ///ptr must have been obtained by prepare read
    unsafe fn liberate(data: RawData) {
        let ptr = data.ptr as *const SingleDataWrapper<T, N>;
        (*ptr).signal_read_done();
    }
    const TASK_WAIT: bool = true;
}

impl<T> TorWrapperT<T> for &'static T {
    fn prepare_read(&self, _task: &QueueElem<VirtualTask>) -> RawData {
        RawData {
            ptr: *self as *const T as _,
        }
    }
    ///ptr must have been obtained by prepare read
    unsafe fn read(data: RawData) -> *const T {
        data.ptr as _
    }
    ///ptr must have been obtained by prepare read
    unsafe fn liberate(_contextptr: RawData) {}
    const TASK_WAIT: bool = false;
}

impl<T> TorWrapperT<T> for T
where
    T: Copy,
{
    
    fn prepare_read(&self, _task: &QueueElem<VirtualTask>) -> RawData {
        if size_of::<T>() > size_of::<*const ()>() {
            panic!()
        } else {
            #[repr(C)]
            union Converter<T2:Copy> {
                converted: MaybeUninit<usize>,
                value: T2,
            }
            let converter = Converter {value: *self};
            RawData {
                value: unsafe {
                    
                converter.converted
                }
            }
        }
    }

    unsafe fn read(data: RawData) -> *const T {
        todo!()
    }

    unsafe fn liberate(data: RawData) {
        todo!()
    }
    const TASK_WAIT: bool = false;
}
