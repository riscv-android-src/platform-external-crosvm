// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Small system utility modules for usage by other modules.

extern crate libc;

mod mmap;
mod errno;

pub use mmap::*;
pub use errno::{Error, Result};
use errno::errno_result;
