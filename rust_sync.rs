
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::{
    condvar_init, mutex_init, spinlock_init,
    sync::{CondVar, Mutex, SpinLock},
};

module! {
    type: RustSync,
    name: b"rust_sync",
    author: b"Rust for Linux Contributors",
    description: b"Rust synchronisation primitives sample",
    license: b"GPL v2",
}

struct RustSync;

impl KernelModule for RustSync {
    fn init() -> Result<Self> {
        pr_info!("Rust synchronisation primitives sample (init)\n");

        // Test mutexes.
        {
            // SAFETY: `init` is called below.
            let mut data = Pin::from(Box::try_new(unsafe { Mutex::new(0) })?);
            // I don't understand what 'pinned' is.
            // I guess it prevent data moved because there is a self-pointer inside of strct.
            // The contents of data can be changed and freed, but cannot be moved.

            mutex_init!(data.as_mut(), "RustSync::init::data1");
            *data.lock() = 10;
            // According to other books, mutex.lock() locks the data until the scope of the variable ends.
            // What is this? "data" is a mutex. There is not scope because data.lock() is not assigned to a variable.

            pr_info!("Value: {}\n", *data.lock());

            // SAFETY: `init` is called below.
            let mut cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_mut(), "RustSync::init::cv1");

            {
                let mut guard = data.lock();
                pr_info!("mutex guard={}", *guard);
                while *guard != 10 {
                    pr_info!("mutex guard={}", *guard);
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        // Test spinlocks.
        {
            // SAFETY: `init` is called below.
            let mut data = Pin::from(Box::try_new(unsafe { SpinLock::new(0) })?);
            spinlock_init!(data.as_mut(), "RustSync::init::data2");
            *data.lock() = 10;
            pr_info!("Value: {}\n", *data.lock());

            // SAFETY: `init` is called below.
            let mut cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_mut(), "RustSync::init::cv2");
            {
                let mut guard = data.lock();
                pr_info!("spinlock guard={}", *guard);
                while *guard != 10 {
                    pr_info!("spinlock guard={}", *guard);
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        Ok(RustSync)
    }
}

impl Drop for RustSync {
    fn drop(&mut self) {
        pr_info!("Rust synchronisation primitives sample (exit)\n");
    }
}
