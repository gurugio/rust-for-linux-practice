/*
linux/samples/rust/rust_chrdev.rs
I add two file-operations: read and write
- write-operation failed with EFAULT
- I checked the comments of read_slice: Returns `EFAULT` if the byte slice is bigger than the remaining size of the user slice 
- So I must use chunk instead of chunkbuf.
- This souce works well.
*/

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::{c_str, chrdev, file::File, 
    file_operations::FileOperations, io_buffer::{IoBufferReader, IoBufferWriter}};
//use kernel::str::CStr;

use kernel::prelude::*;
module! {
    type: RustChrdev,
    name: b"rust_chrdev",
    author: b"Rust for Linux Contributors",
    description: b"Rust character device sample",
    license: b"GPL v2",
}
#[derive(Default)]
struct RustFile;

impl FileOperations for RustFile {
    kernel::declare_file_operations!(read, write, read_iter, write_iter);

    fn read(_this: &Self, _file: &File, buf: &mut impl IoBufferWriter, _: u64) -> Result<usize> {
        let total = buf.len();
        let chunkbuf = b"a";
        while !buf.is_empty() {
            pr_crit!("buf-len={}\n", buf.len());
            buf.write_slice(chunkbuf)?;
        }
        Ok(total)
    }

    fn write(_this: &Self, _file: &File, buf: &mut impl IoBufferReader, _: u64) -> Result<usize> {
        let total = buf.len();
        let mut chunkbuf = [0; 256];
        pr_crit!("write total={}\n", total);

        while !buf.is_empty() {
            let len = chunkbuf.len().min(buf.len());
            let chunk = &mut chunkbuf[0..len];

            pr_alert!("before: buf-len={} chunk-len={}\n", buf.len(), chunk.len());
            buf.read_slice(chunk)?;
            pr_alert!("write-{}\n", chunkbuf[0] as i32);
        }
        Ok(total)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for RustChrdev {
    fn init() -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(c_str!("rust_chrdev"), 0, &THIS_MODULE)?;

        // Register the same kind of device twice, we're just demonstrating
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
