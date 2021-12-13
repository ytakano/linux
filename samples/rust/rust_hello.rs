// SPDX-License-Identifier: GPL-2.0

//! Rust miscellaneous device sample

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::{
    file::File,
    file_operations::{FileOpener, FileOperations},
    io_buffer::IoBufferWriter,
    miscdev,
};

module! {
    type: RustMiscdev,
    name: b"rust_hello",
    author: b"Yuuki Takano",
    description: b"Hello, World! in Rust",
    license: b"GPL v2",
}

#[derive(Clone)]
struct Hello;

impl FileOpener<Hello> for Hello {
    fn open(shared: &Hello, _file: &File) -> Result<Self::Wrapper> {
        Ok(Box::try_new(shared.clone())?)
    }
}

impl FileOperations for Hello {
    kernel::declare_file_operations!(read);

    fn read(
        _shared: &Hello,
        file: &File,
        data: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("rust_hello: read, pos = {}", file.pos());
        if file.pos() == 0 {
            let hello = b"Hello, World!\n";
            data.write_slice(hello)?;
            Ok(hello.len())
        } else {
            Ok(0)
        }
    }
}

struct RustMiscdev {
    _dev: Pin<Box<miscdev::Registration<Hello>>>,
}

impl KernelModule for RustMiscdev {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, World! in Rust (init)\n");

        Ok(RustMiscdev {
            _dev: miscdev::Registration::new_pinned::<Hello>(name, None, Hello)?,
        })
    }
}

impl Drop for RustMiscdev {
    fn drop(&mut self) {
        pr_info!("Hello, World! in Rust (exit)\n");
    }
}
