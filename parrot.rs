// SPDX-License-Identifier: GPL-2.0
//! Rust Parrot Module

mod frames;
use frames::{calc_frame_and_offset, FRAMES};

use core::{cmp, time};
use kernel::{self, delay, file, io_buffer, miscdev, prelude::*, str};

module! {
    type: Parrot,
    name: "parrot",
    author: "Pablo Alessandro Santos Hugen",
    description: "Dancing parrot",
    license: "GPL",
}

struct Parrot(Pin<Box<miscdev::Registration<Self>>>);

#[vtable]
impl file::Operations for Parrot {
    fn open(_: &Self::OpenData, _: &file::File) -> Result<Self::Data> {
        Ok(())
    }

    fn read(
        _: (),
        _: &file::File,
        buf: &mut impl io_buffer::IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        if buf.len() < 1 {
            pr_info!("parrot device driver requires a buffer of at least 1 byte");
            return Err(EINVAL);
        }
        let (frame, frame_offset) = calc_frame_and_offset(offset);
        let frame = FRAMES.get(frame).ok_or(EIO)?;
        let offset_usize: usize = frame_offset.try_into()?;
        let s =
            &frame.as_bytes()[offset_usize..][..cmp::min(frame.len() - offset_usize, buf.len())];
        buf.write_slice(s)?;
        if offset_usize + s.len() == frame.len() {
            delay::coarse_sleep(time::Duration::from_millis(50));
        }
        Ok(s.len())
    }
}

impl kernel::Module for Parrot {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Loading {} module", name.to_str()?);
        Ok(Self(miscdev::Registration::new_pinned(fmt!("parrot"), ())?))
    }
}

impl Drop for Parrot {
    fn drop(&mut self) {
        pr_info!("Dropping parrot");
    }
}
