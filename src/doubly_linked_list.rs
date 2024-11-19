use std::ptr;

#[derive(PartialEq, Copy, Clone)]
pub struct Node<T> {
    pub elem: T,
    pub next: *mut Node<T>,
    pub prev: *mut Node<T>,
}

pub struct List<T> where T: Clone + Copy {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> List<T> where T: Clone + Copy {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn next(&self, cur: *mut Node<T>) -> Option<*mut Node<T>> {
        unsafe {
            if !cur.is_null() && !(*cur).next.is_null() {
                return Some((*cur).next)
            }
            None
        }
    }

    pub fn prev(&self, cur: *mut Node<T>) -> Option<*mut Node<T>> {
        unsafe {
            if !cur.is_null() && !(*cur).prev.is_null() {
                return Some((*cur).prev)
            }
            None
        }
    }

    pub fn head(&self) -> Option<*mut Node<T>> {
        if !self.head.is_null() {
            return Some(self.head)
        }
        None
    }

    pub fn tail(&self) -> Option<*mut Node<T>> {
        if !self.tail.is_null() {
            return Some(self.tail)
        }
        None
    }

    pub fn remove(&mut self, node: *mut Node<T>) -> bool {
        unsafe {
            if node.is_null() {
                return false
            }
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.head = (*node).next;
            }
    
            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.tail = (*node).prev;
            }
            true
        }
    }

    // this is absolutely horrendous but at least it'll be temporary
    pub fn insert_node_after(&mut self, after: *mut Node<T>, new_node: *mut Node<T>) -> bool {
        unsafe {
            if after.is_null() || new_node.is_null() {
                return false
            }
            (*new_node).next = (*after).next;
            (*new_node).prev = after;
            (*after).next = new_node;
            if (*new_node).next.is_null() {
                self.tail = new_node;
            } else {
                (*(*new_node).next).prev = new_node;
            }
            true
        }
    }

    pub fn push_back(&mut self, element: T) {
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

    pub fn pop_back(&mut self) -> Option<T> {
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

    pub fn push_front(&mut self, element: T) {
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

    pub fn pop_front(&mut self) -> Option<T> {
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

    pub fn peek_front(&self) -> Option<&T> {
        unsafe {
            if !self.head.is_null() {
                return Some(&(*self.head).elem);
            }
            None
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe {
            if !self.tail.is_null() {
                return Some(&(*self.tail).elem);
            }
            None
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter {
                next: Some(&*self.head),
                next_back: Some(&*self.tail),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut {
                next: Some(&mut *self.head),
                next_back: Some(&mut *self.tail),
            }
        }
    }
}

pub struct IntoIter<T: Copy + Clone>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    next_back: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    next_back: Option<&'a mut Node<T>>,
}

impl<T: Copy + Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T: Copy + Clone> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next = unsafe { node.next.as_ref() };
            }
            return Some(&node.elem);
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_back {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next_back = unsafe { node.prev.as_ref() };
            }
            return Some(&node.elem);
        }
        None
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: PartialEq,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next.take() {
            if Some(&node) == self.next_back.as_ref() {
                self.next = None;
                self.next_back = None;
            } else {
                self.next = unsafe { node.next.as_mut() };
            }
            return Some(&mut node.elem);
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_back.take() {
            if self.next == self.next_back {
                self.next = None;
                self.next_back = None;
            } else {
                self.next_back = unsafe { node.prev.as_mut() };
            }
            return Some(&mut node.elem);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use crate::doubly_linked_list::Node;

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
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

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
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

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
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(1).as_mut());
        assert_eq!(iter.next_back(), Some(3).as_mut());
        assert_eq!(iter.next(), Some(2).as_mut());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn insert_after_middle_case() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let head = list.head().unwrap();
        let mut node  = Node {
            elem: 4,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        list.insert_node_after(head, &mut node);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1).as_ref());
        assert_eq!(iter.next(), Some(4).as_ref());
        assert_eq!(iter.next(), Some(2).as_ref());
    }

    #[test]
    fn insert_after_edge_case() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let tail = list.tail().unwrap();
        let mut node  = Node {
            elem: 4,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        list.insert_node_after(tail, &mut node);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1).as_ref());
        assert_eq!(iter.next(), Some(2).as_ref());
        assert_eq!(iter.next(), Some(3).as_ref());
        assert_eq!(iter.next(), Some(4).as_ref());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_from_front() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let head = list.head().unwrap();
        list.remove(head);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(2).as_ref());
        assert_eq!(iter.next(), Some(3).as_ref());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_from_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let tail = list.tail().unwrap();
        list.remove(tail);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1).as_ref());
        assert_eq!(iter.next(), Some(2).as_ref());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_from_middle() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let tail = list.tail().unwrap();
        list.remove((unsafe { *tail }).prev);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1).as_ref());
        assert_eq!(iter.next(), Some(3).as_ref());
        assert_eq!(iter.next(), None);
    }
}
