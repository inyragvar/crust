pub mod core;
pub mod log_tag;

use crate::{core::logs};

fn main() {
    logs::out(log_tag!(), "Hello, world!");
}
