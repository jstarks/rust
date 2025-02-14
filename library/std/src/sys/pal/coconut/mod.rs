#![deny(unsafe_op_in_unsafe_fn)]

#[path = "../unsupported/args.rs"]
pub mod args;
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/time.rs"]
pub mod time;

mod common;
pub use common::*;
