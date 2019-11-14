use std::collections::HashMap;
use std::hash::Hash;

// section 3.3

fn pass_reference_to_outer_scope() {
    let x = 0;
    let z;
    let y = &x;
    z = y;

    /* 
    Desugar:
    'a : {
        let x = 0;
        'b : {
            let z : &'b i32;
            'c : {
                let y : &'b i32 = &'b x;
                z : &'b i32 = y;
            }
        }
    }
    */
}

/*
fn as_str<'a>(data: &'a u32) -> &'a str {
    'b : {
        let s = format!("{}", data);
        &'a s
    }
}
*/

/* 
fn invalid_alising() {
    'a : {
        let mut data = vec![1,2,3];
        'b : {
            let x : &'b i32 = Index::index::<'b>(&'b data, 0);
            'c : {
                Vec::push(&'c mut data, 4);                
            }
            println!("{}", x);
        }
    }
} 
*/

fn valid_aliasing() {
    let mut data = vec![1,2,3];
    let x = &data[0];
    println!("{}", x);
    data.push(4);
    /*
    Desugar
    'a : {
        let mut data = vec![1,2,3];
        'b : {
            let x : &'b i32 = Index::index::<'b>(&'b data, 0);            
            println!("{}", x);
        }
        'c : {
            Vec::push(&'c mut data, 4);                
        }
    }
    */
}

/*
#[derive(Debug)]
struct X<'a>(&'a i32);

impl<'a> Drop for X<'a> {
    fn drop(&mut self) {

    }
}

fn still_invalid() {
    'a: {
        let mut data = vec![1,2,3];
        'b : {
            let x: struct X<'b> = X(&'b data[0]);
            println!("{:?}", x);
            'c : {
                Vec::push(&'c mut data, 4);
            }
            drop(&'b mut x);
        }
        
    }
}
*/

// section 3.4

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {&*self}
    fn share(&self) {}
}

/* fn do_not_compile() {
    // foo is a zero-sized object
    let mut foo = Foo;
    let foo_ref = foo.mutate_and_share();
    foo.share();
    println!("{:?}", foo_ref);    
}
 */


// Limits of life time
/*  
fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }
    }
}
 */

// section 3.5 lifetime elission

// lifetime eliding rule
// 1. Each elided lifetime in input position becomes a distinct lifetime parameter
// 2. If there is exactly one input lifetime position (elided or not), that elided lifetime
//    is assigned to all elided output lifetime positions.
// 3. If there are multiple input lifetime positions, but one of them is &self and &mut self, then
//    the lifetime of self is assigned to all the elided output lifetime positions.
// Otherwise, an error would occur.
//Examples:
/* 
fn print(&str);
fn print<'a>(&'a str);

fn debug(lvl: usize, s: &str);
fn debug<'a>(lvl: usize, s: &'a str);

fn substr(s: &str, param : usize) -> &str;
fn substr<'a>(s: &'a str, param: usize) -> &'a str;

fn get_str() -> &str; // This is illegal, the output parameter has unbounded lifetime

fn frob(s: &str, t: &str) -> &str; // This is illegal, we must specify two input lifetimes.
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &'b str; This is legal.

fn get_mut(&mut self) -> &mut T; 
fn get_mut<'a>(&'a mut self) -> &'a mut T;

fn args<T: ToCStr>(&mut self, args: &[T])  -> &mut Command;
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

fn new(buf: &mut [u8]) -> BufWriter;
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;
*/

// section 3.6 unbounded lifetime
// This function produce a reference with unbounded lifetime
// fn get_str<'a>() -> &'a str;
// So better use lifetime elision to bound the lifetime
// fn get_str()->&str; This function will never compile

// section 3.7 higher-rank trait bound
struct Closure<F> {
    data: (u8, u8),
    func: F,
}

impl<F> Closure<F> where F : Fn(&(u8, u8)) -> &u8 {
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn real_func(input: &(u8, u8)) -> &u8 {
    &(input.0)
}

fn call_closure() {
    let clo = Closure {
        data : (1, 2),
        func : real_func,
    };
    let res = clo.call();
    println!("{}", res);
}

// The previous code can be desugared into:
/* 
struct Closure<F> {
    data: (u8, u8),
    func: F,
}

impl<F> Closure<F> where for<'a>, F : Fn(&'a (u8, u8)) -> &'a u8 {
    fn call<'b>(&<'b>self) -> &<'b>u8 {
        (self.func)(&'b self.data)
    }
}

fn real_func<'a>(input: &'a (u8, u8)) -> &'a u8 {
    &'a (input.0)
}

fn call_closure() {
    let clo = Closure {
        data : (1, 2),
        func : real_func,
    };
    let res = clo.call();
    println!("{}", res);
}

*/

// section 3.8 subtyping and variance

// Objective Rust...
/*
trait Animal {
    fn snuggle(&self);
    fn eat(&mut self);
}

trait Cat : Animal {
    fn meow(&self);
}

trait Dog : Animal {
    fn bark(&self);
}
*/ 
// Ok, rust doesn't have this feature. 
// The actual subtyping in Rust happens over lifetime type paramters
// 'big : 'small : 'big lifetime is a subtype of 'small lifetime. Meaning that 
// 'big lifetime can outlive 'small lifetime. Also mean that 'big lifetime can be 
// used just like 'small lifetime.

// Type constructor:
// F<T>: F is the type constructor, whereas T is the type paramter to F
// A type constructor F's variance is how the subtyping of its inputs affects the subtyping 
// of type constructor F's output

// There are three types of type variance in Rust. Given Sub : Super,
// We have:
// F is covariant if F<Sub> : F<Super> (F<Sub> is a subtype of F<Super>)
// F is contravariant if F<Super> : F<Sub> (F<Super> is a subtype of F<Sub>)
// F is invariant if F<Sub> and F<Super> have no subtyping relationship

// &'a T, covariant over 'a, covariant over T
// &'a mut T, covariant over 'a, invariant over T
// Box<T>, covariant over T
// Vec<T>, covariant over T
// UnsafeCell<T>, invariant over T
// Cell<T> invariant over T
// Fn(U) -> T, contravariant over U, covariant over T
// *const T, covariant over T
// *mut T, invariant over T

// fn test(&'a str) -> &'a str;
// 'long can be used for 'short, when used in input argument, since 'long : 'short, it is 
//  contravariant.
// when used in output argument, it is contravariant.

// section 3.9 drop check:

// The following code defines a struct which can do self-refernce
struct Inspector<'a>(&'a u8);

struct World<'a> {
    inspector : Option<Inspector<'a>>,
    days : Box<u8>,
}

// Condition1: the following code can successfully compile, because both inspector and days
// will be dropped automatically when world goes out of scope.
/* 
fn drop_checker() {
    let mut world = World {
        inspector : None,
        days : Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days));
} 
*/

// Condition2, if we implement the drop trait for World, then the previous code would not compile
// because the lifetime of the borrow of world.days would be extended to the drop call, where 
// world.days would be dropped. Apparently, we can't drop something that is still being borrowed.

/* 
impl<'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("I was only {} days from retirement!", self.0);
    }
}


fn drop_checker1() {
    let mut world = World {
        inspector : None,
        days : Box::new(1),
    };

    world.inspector = Some(Inspector(&world.days));
} 
*/

// Condition 3, the code shown in drop_checker2 is also very interesting. Firstly, world.days
// is borrowed. Then the reference variable world.inspector is reset to None. From a practical
// point of view, this code is sound and safe because the reference variable which stores the borrow
// now no longer has the borrow, and the borrow should end right after world.inspector is reset 
// to None. However, the conservative Rust borrow checker only recognized that world.days is 
// borrowed for the lifetime that is extended to the end of the function scope. Therefore, when we 
// modify world.days, the borrow checker would complain that world.days is still being borrowed.
// This indicates that the borrow checker only keep record of the borrow and its life time, it 
// doesn't care whether the borrow has been assigned to some reference variables.

/* 
fn drop_checker2() {
    let mut world = World {
        inspector : None,
        days : Box::new(1),
    };

    world.inspector = Some(Inspector(&world.days));
    world.inspector = None;
    world.days = Box::new(2);
} 
*/

// drop_checker3 is still problematic because world.inspector has type Option<Inspector<&'a u8>>
// 'a is shown in drop_checker3, so world.days still can't be modified because it is kept 
// borrowing for the scope of 'a.
/* 
fn drop_checker3() {
    // 'a : {
    let mut world = World {
        inspector : None,
        days : Box::new(1),
    };

    {
        world.inspector = Some(Inspector(&world.days));
        world.inspector = None;
    }

    world.days = Box::new(2);
    // }
}
*/

// Condition 4: This is incorrect because world is imutably borrowed for the entire scope of 
// drop_checker4. Hence we can't mutably borrow world. An implication of this is that, one may 
// change world.days inside mutate_world function, making world.inspector a dangling reference.

// My hypothesis is that, the drop trait will not take into account whether the variable
// being dropped has already been borrowed before. When the variable is dropped, it appears
// that the variable will be mutably borrowed, but since it is the last use of this variable,
// the borrow checker will not complain and only account it as a simple use

/* 
fn mutate_world<'a>(world: &mut World<'a>) {

}

fn drop_checker4() {
    let mut world = World {
        inspector : None,
        days : Box::new(1),
    };

    world.inspector = Some(Inspector(&world.days));
    mutate_world(&mut world);
}
*/

// section 3.10: 

use std::marker;

struct Iter<'a, T : 'a> {
    ptr : *const T,
    end : *const T,
    _marker: marker::PhantomData<&'a T>,
}

struct Vec<T> {
    ptr : *const T,
    len: usize,
    cap : usize,
    _marker : marker::PhantomData<T>,
}

// section 3.11:

// 1. Rust understands mutable borrow into sub fields
struct Foo311 {
    i : i32,
    j : i32,
    k : i32,
}

fn mutate_foo311(foo: &mut Foo311) {

}

/* 
fn demo311_myown_invalid() {
    let mut foo = Foo311 {
        i : 1,
        j : 2,
        k : 3,
    };

    let a = &foo.i;

    mutate_foo311(&mut foo);
    println!("{}", a);    
} 
*/

fn demo311() {
    let mut foo = Foo311 {
        i : 1,
        j : 2,
        k : 3,
    };

    let a = &mut foo.i;
    let b = &mut foo.j;
    let c = &foo.k;

    *a += 1;
    *b += 1;
    println!("{}, {}, {}", a, b, c);
}

// 2. However, rust doesn't understand mutable borrow into arrays or slices
/* 
fn demo311_2() {
    let mut x = [1,2,3];
    let a = &mut x[0];
    let b = &mut x[1];

    println!("{}, {}", a, b);
} 
*/

pub fn ch3_run() {
    let x = 0;
    let y = &x;
    let z = &y;
    /*
    Desugar:
    'a : {
        let x = 0;
        'b : {
            let y : &'b i32 = &'b x;
            'c : {
                let z : &'c &'b i32 = &'c y;
            }
        }
    }
    
    */ 

    println!("ch3_run");
    call_closure();
    demo311();
}