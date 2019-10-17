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
        });
        None
    }
}


fn you_know_nothing(jon_snow: &mut i32) {
    let nights_watch = &mut (*jon_snow);
    println!("{}", nights_watch);
    
    let mut starks = || {
        *jon_snow = 3; // error: closure requires unique access to `jon_snow`
                       //        but it is already borrowed
    };
    starks();
}

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
}
