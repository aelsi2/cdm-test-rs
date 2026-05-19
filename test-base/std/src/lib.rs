#![no_std]
#![feature(core_intrinsics)]

extern crate alloc as alloc_crate;

pub use core::any;
pub use core::array;
pub use core::cell;
pub use core::char;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::future;
pub use core::hint;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::option;
pub use core::pin;
pub use core::ptr;
pub use core::result;
pub use core::intrinsics;

pub use alloc_crate::alloc;
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::rc;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;

pub mod task {
    pub use alloc_crate::task::*;
    pub use core::task::*;
}

pub use core::matches;
pub use core::primitive;
pub use core::primitive::*;
pub use core::todo;

pub use core::{
    assert, cfg, column, compile_error, concat, env, file, format_args, include, include_bytes,
    include_str, line, module_path, option_env, stringify,
};

pub use alloc_crate::collections;
pub use core::panic;

#[macro_export]
macro_rules! println {
    () => {{}};
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{}};
}

pub mod prelude {
    pub use crate::borrow::ToOwned;
    pub use crate::boxed::Box;
    pub use crate::print;
    pub use crate::println;
    pub use crate::string::{String, ToString};
    pub use crate::vec;
    pub use crate::vec::Vec;
    pub use crate::format;
}
