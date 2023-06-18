use crate::{build_create_one, build_find_one};

use super::models::*;

build_create_one!(create_thread, threads, NewThread, Thread);
build_find_one!(find_one_thread, threads, Thread, i32);
