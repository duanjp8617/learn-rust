use std::collections::HashMap;
use std::hash::Hash;
// rfc 2094 is about the proposal for non-lexical lifetime. However, it contains many interesting
// discussions about the current lifetime mechenisms of rust. Here I take some notes that I learned
// from this rfc.

// lifetime: the lifetime of a reference indicates the span of time in which the reference is used.
// scope: the scope of a variable indicates the span of time from when the variable is declared to the time the variabled is freed.
// if you create a reference to a variable, then the lifetime of the reference can not exceed the 
// scope of the variable.

// utility function shared by some code examples.
fn capitalize(data: &mut [char]) {
    // Do nothing
}

// the following function is totally valid, because each borrow does not escape from the 
// function scope
fn foo() {
    let mut data = vec!['a', 'b', 'c'];
    capitalize(&mut data[..]);
    data.push('d');
    data.push('e');
    data.push('f');
}
/* 
fn foo() {
    'a: {
        let mut data = vec!['a', 'b', 'c'];
        'b: {
            capitalize(&'b mut data[..]);
        }
        'c: {
            Vec::push(&'c mut data, 'd');
        }
        'd: {
            Vec::push(&'d mut data, 'e');
        }
        'f: {
            Vec::push(&'f mut data, 'f');
        }
    }
}
*/

// However, the current lifetime mechanism used by rust has some serious drawbacks.capitalize,
// and quite surprisingly, some proposals raised in rfc 2094 have been accepted in rust 1.39..
// what the hack...

// Problem1: references assigned to a variable (this is no longer a problem now)

fn p1() {
    let mut data = vec!['a', 'b', 'c'];

    // This is no longer a problem now, the rust compiler would recognize this 
    // program as the following desugared one.
    /* 
    'a : {
        let slice = &'a mut data[..];
        capatalize(slice);
    }
    */ 
    let slice = &mut data[..];
    capitalize(slice);

    data.push('d');
    data.push('e');
    data.push('f');
}

// Problem2: conditional control flow

fn process<V>(v : &mut V) {

}

// The process_or_default code is problematic because: 
/* 
fn process_or_default_desugar<K, V> (map: HashMap<K, V>, key : K)
where K : Clone + Eq + Hash, 
      V : Default, {
    'a : {
        // &'a mut map has life time 'a
        let temp : Option<&'a mut V> = HashMap::get_mut(&'a mut map, &key);
        match tmp {
            Some(value) => process(value),
            None => {
                'c: {
                    // can't borrow map as mutable again when map has already
                    // be borrowed as mutable
                    HashMap::insert(&'c mut map, V::default);
                }
            }
        }
    }
}

*/

/* 
fn process_or_default<K, V> (map: HashMap<K, V>, key : K)
where K : Clone + Eq + Hash, 
      V : Default, {
    match map.get_mut(&key) {
        Some(value) => process(value),
        None => {
            map.insert(key, V::default());
        }
    }
}
 */

// The solution to problem two is like this
fn process_or_default<K, V> (mut map: HashMap<K, V>, key : K)
where K : Clone + Eq + Hash, 
      V : Default, {
    // the lifetime of the mutable borrow only lasts for the match block
    match map.get_mut(&key) {
        Some(value) => {
            process(value);
            return;
        },
        None => {
        }
    }

    // we are free to mutably borrow map again
    map.insert(key, V::default());
}

// problem3: conditional control flow across functions

// the following code is problematic due to similar reason as problem 2
// the lifetime of the first mutable borrow coveres the entire match clause
/* 
fn get_default<'a, K, V> (map: &'a mut HashMap<K, V>, key : K) -> &'a mut V
where K : Clone + Eq + Hash, 
      V : Default, {
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }
    }
}
*/

// we can't solve this problem using similar technique discussed in problem 2
/* 
fn get_default<'a, K, V> (map: &'a mut HashMap<K, V>, key : K) -> &'a mut V
where K : Clone + Eq + Hash, 
      V : Default, {
    match map.get_mut(&key) {
        Some(value) => {
            // returnning value cause the map to be borrowed for 'a
            // extending the life time to cover the entire function body
            return value;
        },
        None => {
        }
    };

    map.insert(key.clone(), V::default());
    map.get_mut(&key).unwrap()
}
*/

// the work around is to rely on control flow, because the control flow
// confines the analysis of borrow checker. But at the expense of running more
// code...
fn get_default<'a, K, V> (map: &'a mut HashMap<K, V>, key : K) -> &'a mut V
where K : Clone + Eq + Hash, 
      V : Default, {
    if map.contains_key(&key) {
        return map.get_mut(&key).unwrap();
    }

    map.insert(key.clone(), V::default());
    map.get_mut(&key).unwrap()
}

// There is a more efficient version, which is to use the entry method of hashmap
// problem 3 is the major motivation for designing the entry method.
fn get_default2<'a, K, V> (map: &'a mut HashMap<K, V>, key : K) -> &'a mut V
where K : Clone + Eq + Hash, 
      V : Default, {
    map.entry(key).or_insert_with(|| V::default())
}

// Problem 4: mutating &mut references (this works now!)

struct List<T> {
    elem : T,
    next : Option<Box<List<T>>>,
}

fn to_refs<T> (mut list : &mut List<T>) -> Vec<&mut T> {
    let mut res_vec = vec![];
    
    loop {
        res_vec.push(&mut list.elem);
        if let Some(next_list) = list.next.as_mut() {
            list = &mut (**next_list);
        }
        else {
            return res_vec;
        }
    }
}

/* fn foo(mut vec: Vec<i32>) {
    let mut iter = vec.iter_mut();
    let mut cur = iter.next().unwrap();
    let mut next = iter.next().unwrap();
    loop {
        *next = 22;
        next = iter.next().unwrap();
    }
}
 */

pub fn run() {
}

