// A simple implementation of a singly linked list.

pub struct List<T> {
    head : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem : T,
    next: Link<T>,
}

// method implementation
impl<T> List<T> {
    pub fn new() -> Self {
        List{head : None}
    }

    pub fn push(&mut self, elem : T) {
        let new_node = Box::new(Node{
            elem : elem,
            next : self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }

    pub fn peak_n(&self, n: u32) -> Option<&T> {
        let mut iter = self.iter();
        let mut n_mut = n;
        while let Some(elem) = iter.next() {
            match n_mut {
                0 => {
                    return Some(elem);
                },
                _ => {
                    n_mut -= 1;
                }
            }
        };
        return None;
    }

    pub fn push_n(&mut self, n: u32, elem: T) {
        // handle special case
        if n == 0 || self.head.is_none() {
            let new_node = Box::new(Node {
                elem : elem,
                next : self.head.take()
            });
            self.head = Some(new_node);
            return;
        }
        // obtain the first node on the list
        let mut node = self.head.as_mut().map(|node| {
            &mut **node
        }).unwrap();
        // initialize the counter
        let mut n_mut = n-1;
        // find the insertion node
        while n_mut != 0 && node.next.is_some() {
            node = node.next.as_mut().map(|node|{
                &mut **node
            }).unwrap();
            n_mut -= 1;  
        };
        // insert
        let new_node = Box::new(Node{
            elem: elem,
            next : node.next.take()
        });
        node.next = Some(new_node); 
    }

    pub fn pop_n(&mut self, n: u32) -> Option<T> {
        if self.head.is_none() {
            None
        } else if n == 0 {
            self.head.take().map(|node|{
                self.head = node.next;
                node.elem
            })
        } else {
            let mut node = self.head.as_mut().map(|node| {
                &mut **node
            }).unwrap();
            let mut n_mut = n-1;
            
            while n_mut != 0 && node.next.is_some() {
                node = node.next.as_mut().map(|node|{
                    &mut **node
                }).unwrap();
                n_mut -= 1;  
            };

            if node.next.is_none() {
                None
            } else {
                node.next.take().map(|node_to_pop|{
                    node.next = node_to_pop.next;
                    node_to_pop.elem
                })
            }
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    } 

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter(self.head.as_ref().map(|node|{
            &(**node)
        }))
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.head.as_mut().map(|node| &mut **node))
    }
}

// Drop trait implementation to prevent stack overflow when deconstructing 
// an extremely long list
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut box_node = self.head.take();
        while let Some(ref mut drop_node) = box_node {
            box_node = drop_node.next.take();
        }
    }
}

// intoiter
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// iter 
pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node|{
            self.0 = node.next.as_ref().map(|next_node|{
                &(**next_node)
            });
            &node.elem
        })
    }
}

// iter_mut
pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

/* fn you_know_nothing(jon_snow: &mut i32) {
    let nights_watch = &mut (*jon_snow);
    println!("{}", nights_watch);
    
    let mut starks = || {
        *jon_snow = 3; // error: closure requires unique access to `jon_snow`
                       //        but it is already borrowed
    };
    starks();

    let mut s = String::from("fuck!");
    let r1 = &mut s;
    println!("{}", r1);
    
    let r2 = &mut s;
    println!("{}", r2);
} */

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn push_pop() {
        let mut l = List::<i32>{head:None};

        l.push(3);
        l.push(2);
        l.push(1);

        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut l = List::<i32>{head:None};

        l.push(3);
        l.push(2);
        l.push(1);

        let mut iter = l.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter(){
        let mut l = List::<i32>{head:None};

        l.push(3);
        l.push(2);
        l.push(1);

        let mut iter = l.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None); 
    }

    #[test]
    fn iter_mut(){
        let mut l = List::<i32>{head:None};

        l.push(3);
        l.push(2);
        l.push(1);

        let mut iter = l.iter_mut();
        while let Some(elem) = iter.next() {
            *elem += 1;
        }

        let mut iter2 = l.iter();        
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&4));
        assert_eq!(iter2.next(), None);
    }

    #[test]
    fn use_move(){
        let mut l = List::new();
        l.push(3);
        l.push(2);
        l.push(1);

        let mut iter = l.iter();
        let opt = iter.next();
        let refvar : &i32;
        match opt {
            Some(elem) => refvar = elem,
            None => refvar = &5,
        };

        assert_eq!(*refvar, 1);

        // try un comment this line and see the error.
        // l.pop();
        
        assert_eq!(*refvar, 1);
    }

    #[test]
    fn peek_n() {
        let mut l = List::new();
        l.push(3);
        l.push(2);
        l.push(1);

        assert_eq!(l.peak_n(0), Some(&1));
        assert_eq!(l.peak_n(1), Some(&2));
        assert_eq!(l.peak_n(2), Some(&3));
        assert_eq!(l.peak_n(4), None);
        assert_eq!(l.peak_n(5), None);
    }

    #[test]
    fn push_n() {
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            l.push_n(0, 0);

            let mut iter = l.iter();

            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3)); 
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            l.push_n(1, 1);

            let mut iter = l.iter();

            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3)); 
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            l.push_n(2, 2);

            let mut iter = l.iter();

            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3)); 
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            l.push_n(3, 3);

            let mut iter = l.iter();

            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&3));     
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            l.push_n(1, 1);
            l.push_n(2, 2);

            let mut iter = l.iter();

            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3)); 
        }
    }

    #[test]
    fn pop_n() {
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            assert_eq!(l.pop_n(0), Some(1));
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            assert_eq!(l.pop_n(1), Some(2));
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            assert_eq!(l.pop_n(2), Some(3));
        }
        {
            let mut l = List::new();
            l.push(3);
            l.push(2);
            l.push(1);

            assert_eq!(l.pop_n(4), None);

        }
    }
    
}
