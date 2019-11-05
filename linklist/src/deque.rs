use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head : Link<T>,
    tail : Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem : T,
    next : Link<T>,
    prev : Link<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head : None,
            tail : None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_node = Rc::new(RefCell::new(Node {
            elem : elem,
            next : None,
            prev : None,
        }));

        match self.head.take() {
            Some(old_node) => {
                old_node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_node);
                self.head = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|pop_node|{
            match pop_node.borrow_mut().next.take() {
                None => {
                    self.tail.take();
                },
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
            };
            Rc::try_unwrap(pop_node).ok().unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| {
                &node.elem
            })
        })
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|head_node|{
            RefMut::map(head_node.borrow_mut(), |node|{
                &mut node.elem
            })
        })
    }

    fn push_back(&mut self, elem : T) {
        let new_node = Rc::new(RefCell::new(Node {
            elem : elem,
            next : None,
            prev : None,
        }));
        
        match self.tail.take() {
            Some(last_node) => {
                last_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(last_node);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|last_node| {
            match last_node.borrow_mut().prev.take() {
                Some(prev_node) => {
                    prev_node.borrow_mut().next.take();
                    self.tail = Some(prev_node);
                },
                None => {
                    self.head.take();
                }
            };
            Rc::try_unwrap(last_node).ok().unwrap().into_inner().elem
        })
    }
    
    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|last_node|{
            Ref::map(last_node.borrow(), |node|{
                &node.elem
            })
        })
    }

    fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|last_node|{
            RefMut::map(last_node.borrow_mut(), |node| {
                &mut node.elem
            })
        })
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/* struct Iter<'a, T> (Option<Ref<'a, Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|inner|{
            let (elem_ref, next) = Ref::map_split(inner, |node| {                        
                (&node.elem, &node.next)
            });

            self.0 = next.as_ref().map(|inner|{
                inner.borrow()
            });  

            elem_ref
        })
    }
} */

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        /* let mut sth = self.pop_back();
        while let Some(_) = sth {
            sth = self.pop_back();
        }; */
        let mut sth = self.pop_front();
        while let Some(_) = sth {
            sth = self.pop_front();
        };
    }
}

struct IntoIter<T> (List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

/* struct Iter<'a, T> (Option<Ref<'a, Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|inner|{
            let (elem_ref, next) = Ref::map_split(inner, |node| {                        
                (&node.elem, &node.next)
            });

            self.0 = next.as_ref().map(|inner|{
                inner.borrow()
            });  

            elem_ref
        })
    }
} */

#[cfg(test)]
mod tests {
    use super::List;
    use std::cell::RefMut;

    #[test]
    fn test_front() {
        let mut l = List::new();
        
        l.push_front(3);
        l.push_front(2);
        l.push_front(1);
        
        {
            let mut r = l.peek_front_mut().unwrap();
            *r = 5;
        }

        assert_eq!(&*l.peek_front().unwrap(), &5);
        assert_eq!(l.pop_front(), Some(5));

        assert_eq!(&*(l.peek_front().unwrap()), &2);
        assert_eq!(l.pop_front(), Some(2));

        assert_eq!(&*l.peek_front().unwrap(), &3);
        assert_eq!(l.pop_front(), Some(3));

        assert_eq!(l.pop_front(), None);
        assert_eq!(l.pop_front(), None);
    }

    #[test]
    fn test_back() {
        let mut l = List::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);

        {
            let mut r = l.peek_back_mut().unwrap();
            *r = 6;
        }

        assert_eq!(&*l.peek_back().unwrap(), &6);
        assert_eq!(l.pop_back(), Some(6));

        assert_eq!(&*(l.peek_back().unwrap()), &2);
        assert_eq!(l.pop_back(), Some(2));

        assert_eq!(&*l.peek_back().unwrap(), &1);
        assert_eq!(l.pop_back(), Some(1));

        assert_eq!(l.pop_back(), None);
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn destroy_long_list() {
        let mut l = List::new();
        for i in 1 .. 1000 {
            l.push_back(i);
        }        
    }
}