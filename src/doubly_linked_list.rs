use std::ptr;

#[derive(PartialEq)]
struct Node<T> {
    elem: T,
    next: *mut Node<T>,
    prev: *mut Node<T>
}

struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>
}

impl<T> List<T> {

    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    fn push_back(&mut self, element: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                elem: element,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            
            if !self.tail.is_null() {
                (*(self.tail)).next = new_node;
                (*new_node).prev = self.tail;
            } else {
                self.head = new_node;
            }
            self.tail = new_node;
        }
    }

    fn pop_back(&mut self) -> Option<T> { 
        unsafe {
            if self.tail.is_null() {
                return None;
            }
            if self.head == self.tail {
                self.head = ptr::null_mut();
            } else {
                (*(*self.tail).prev).next = ptr::null_mut();
            }
            let popped_node = Box::from_raw(self.tail);
            self.tail = popped_node.prev;
            return Some(popped_node.elem);
        }
        
    }

    fn push_front(&mut self, element: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                elem: element,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            
            if !self.head.is_null() {
                (*(self.head)).prev = new_node;
                (*new_node).next = self.head;
            } else {
                self.tail = new_node;
            }
            self.head = new_node;
        }
    }

    fn pop_front(&mut self) -> Option<T> { 
        unsafe {
            if self.head.is_null() {
                return None;
            }
            if self.tail == self.head {
                self.tail = ptr::null_mut();
            } else {
                (*(*self.head).next).prev = ptr::null_mut();
            }
            let popped_node = Box::from_raw(self.head);
            self.head = popped_node.next;
            return Some(popped_node.elem);
        }
    }

    fn peek_front(&self) -> Option<&T> {
        unsafe {
            if !self.head.is_null() {
                return Some(&(*self.head).elem)
            }
            None
        }
        
    }

    fn peek_back(&self) -> Option<&T> {
        unsafe {
            if !self.tail.is_null() {
                return Some(&(*self.tail).elem)
            }
            None
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        unsafe {
            Iter { next: Some(&*self.head ), next_back: Some(&*self.tail)}
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut { next: Some(&mut *self.head ), next_back: Some(&mut *self.tail)}
        }
    }
    
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    next_back: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    next_back: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {

    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<'a,T> Iterator for Iter<'a, T> where T: PartialEq {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next = unsafe { node.next.as_ref() };
            }
            return Some(&node.elem)
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> where T: PartialEq {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_back {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next_back = unsafe { node.prev.as_ref() };
            }
            return Some(&node.elem)
        }
        None
    }
}

impl<'a, T> Iterator for IterMut<'a, T> where T: PartialEq {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next.take() {
            if Some(&node) == self.next_back.as_ref() {
                self.next = None;
                self.next_back = None;
            } else {
                self.next = unsafe { node.next.as_mut() };
            }
            return Some(&mut node.elem)
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> where T: PartialEq {

    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_back.take() {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next_back = unsafe { node.prev.as_mut() };
            }
            return Some(&mut node.elem)
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics_stack() {
        let mut list = List::new();

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        list.push_back(6);
        list.push_back(7);

        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn basics_queue() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        list.push_front(6);
        list.push_front(7);

        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();

        assert_eq!(list.peek_back(), None);
        assert_eq!(list.peek_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek_back().cloned(), Some(1));
        assert_eq!(list.peek_back().cloned(), Some(1));
        assert_eq!(list.peek_front().cloned(), Some(3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_back(1); list.push_back(2); list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_back(1); list.push_back(2); list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1).as_ref());
        assert_eq!(iter.next_back(), Some(3).as_ref());
        assert_eq!(iter.next(), Some(2).as_ref());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_back(1); list.push_back(2); list.push_back(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(1).as_mut());
        assert_eq!(iter.next_back(), Some(3).as_mut());
        assert_eq!(iter.next(), Some(2).as_mut());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

}