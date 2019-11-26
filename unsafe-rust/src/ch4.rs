// this does not work, because x is not initialized
// fn uninitialized_var() {
//     let x : i32;
//     println!("{}", x)
// }

use std::mem::{self, MaybeUninit};

// this however, works, because rust knows that on 
// every brach the variable x will be initialized.
fn initialized_var() {
    let x : i32;
    if true {
        x = 3;
    }
    else {
        x = 4;
    }
    println!("{}", x);
}

// this however, doesn't work because only a single branch 
// initialize x

// fn uninitialized_var() {
//     let x : i32;
//     if true {
//         x = 3;
//     }
//     println!("{}", x);
// }

fn initialized_var1() {
    let x : i32;
    if true {
        x = 3;
        println!("{}", x);
    }

    // This works, because rust knows that x will not be live here
}


fn what_the_fuck_is_this() {
    let x : i32;
    loop {
        if true {
            x = 3;
            break;
        }
    }
    println!("{}", x);
}

// Drop flag: rust stores drop flag for each variable on the stack to track whether the 
// variable has been initialized.
// If the variable is not initialized, it will not be dropped.
// If the value is moved out from a variable, the variable becomes uninitialized,
// it will not be dropped.

fn drop_behavior() {
    let mut x = Box::new(0); // x was initialized
    let y = x; // x was moved out, become uninitialized, y is moved in,
               // become initialized
    // y will be dropped.
    // x will not be dropped
}

// you can't visit the element of an uninitalized array
// fn invalid_array_definition() {
//     let x : [i32; 4];
//     println!("{}", x[0]);
// }

// the initialization of the array is pretty regid in Rust
fn array_definition() {
    // you can either create an array with four identical elements.
    let a1 : [i32; 4] = [1; 4];

    // or you must specify every element of the array
    let a2 : [i32; 4] = [1,2,3,4];
}

const SIZE : usize = 10;

fn mayuninit_test() {
    let mut x : [MaybeUninit<Box<i32>>; SIZE] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    for i in 0..SIZE {

        // x[i] = MaybeUninit::new(Box::new(i as i32));
        
        // This is erroneous because it may cause the drop of uninitialized variable
        // unsafe{*x[i].as_mut_ptr() = Box::new(i as i32);}
    }

    unsafe {mem::transmute::<_, [Box<i32>; SIZE]>(x)};
}



pub fn ch4_run() {
    println!("ch4 run!");
    initialized_var();
    initialized_var1();
    what_the_fuck_is_this();
}