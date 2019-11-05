use std::ptr;


pub struct List<T> {
    head : Link<T>,
    tail : *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem : T,
    next : Link<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head : None,
            tail : ptr::null_mut()
        }
    }

    pub fn push(&mut self, elem : T) {
        let mut new_node = Box::new(Node{
            elem : elem, 
            next : None
        });

        if self.tail.is_null() {
            self.tail = &mut (*new_node);
            self.head = Some(new_node);
        }
        else {
            let new_node_ptr : *mut Node<T> = &mut (*new_node);
            unsafe {
                (*self.tail).next = Some(new_node);
            }
            self.tail = new_node_ptr;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                match node.next.take() {
                    Some(next_node) => {
                        self.head = Some(next_node)
                    }, 
                    None => {
                        self.tail = ptr::null_mut();
                    }
                };
                Some(node.elem)
            },
            None => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_push_pop() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);
        
        l.push(3);
        l.push(2);
        l.push(1);

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
        assert_eq!(l.pop(), None);
    }
}