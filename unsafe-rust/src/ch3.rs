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

// section 3.5
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
}