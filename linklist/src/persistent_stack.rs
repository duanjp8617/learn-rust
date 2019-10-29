use std::rc::Rc;

pub struct List<T> {
    head : Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem : T,
    next : Link<T>
}


impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head : None
        }
    }

    pub fn append(&self, elem: T) -> List<T> {
        let new_node = Rc::new(Node {
            elem : elem,
            next : self.head.clone()
        });

        List {
            head : Some(new_node)
        }
    }

    pub fn tail(&self) -> List<T> {
        match self.head {
            None => {
                List {
                    head : None
                }
            },
            Some (ref rc_node) => {                
                List {
                    head : rc_node.next.clone()
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            cur : self.head.as_ref().map(|node| {
                & **node
            })
        }
    }
}

pub struct Iter<'a, T> {
    cur : Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next.as_ref().map(|node| {
                & **node
            });
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node_opt = self.head.take();
        while let Some(node) = node_opt {
            if let (Ok(mut node)) = Rc::try_unwrap(node) {
                node_opt = node.next.take()
            } else {
                break;
            }
        }
    }
}