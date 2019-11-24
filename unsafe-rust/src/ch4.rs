// this does not work, because x is not initialized
// fn uninitialized_var() {
//     let x : i32;
//     println!("{}", x)
// }

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

fn invalid_array_definition() {
    let x : [i32; 4];
    x = [i32; 4];
}

pub fn ch4_run() {
    println!("ch4 run!");
    initialized_var();
    initialized_var1();
    what_the_fuck_is_this();
}