use std::ptr;

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
}