use std::thread;

// Unfortunately, none of these can be defined because they are unstable library features:(.
/* 
use std::alloc::{Alloc, Global, GlobalAlloc,Layout};
use std::mem;
use std::ptr::{drop_in_place, NonNull, Unique};
 */

// It seems that thread::scoped has been eliminated from std::thread
/* 
fn join_guard_deterministic() {
    let mut data = [1,2,3,4];
    {
        let guards = vec![];
        for x in &mut data {
            let guard = std::thread::scoped(move || {
                *x += 2;
            });
            guards.push(guard);
        }
    }
} */

pub fn run_ch6() {
    println!("ch6 runs");
}