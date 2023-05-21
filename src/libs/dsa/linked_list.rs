#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct DoubleLinkedList<T: std::fmt::Debug> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T: std::fmt::Debug> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Add a value to the front of the linked list
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let raw_node: *mut _ = Box::into_raw(new_node);

        unsafe {
            (*raw_node).next = self.head.take();
            (*raw_node).prev = None;

            if let Some(node) = &mut (*raw_node).next {
                node.prev = Some(raw_node);
            } else {
                self.tail = Some(raw_node);
            }

            self.head = Some(Box::from_raw(raw_node));
        }
    }

    /// Add a value to the back of the linked list
    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let raw_node: *mut _ = Box::into_raw(new_node);

        unsafe {
            (*raw_node).prev = self.tail;

            if let Some(node) = self.tail {
                (*node).next = Some(Box::from_raw(raw_node));
            } else {
                self.head = Some(Box::from_raw(raw_node));
            }

            self.tail = Some(raw_node);
        }
    }

    /// Remove and return the value from the front of the linked list
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
    
            if let Some(next_node) = &mut node.next {
                next_node.prev = node.prev;
            } else {
                self.tail = None;
            }
    
            let value: T = node.value; // Fix: Extract the value correctly
            value
        })
    }

    /// Remove and return the value from the back of the linked list
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|raw_node| {
            unsafe {
                let node = Box::from_raw(raw_node);

                if let Some(prev_node) = node.prev {
                    (*prev_node).next = None;
                    self.tail = Some(prev_node);
                } else {
                    self.head = None;
                    self.tail = None;
                }

                node.value
            }
        })
    }

    /// Check if the linked list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn print_values(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{:?} <-> ", node.value);
            current = &node.next;
        }
        print!("NULL");
        println!("");
    }
}

// impl<T: std::fmt::Debug> Drop for DoubleLinkedList<T> {
//     fn drop(&mut self) {
//         while self.pop_front().is_some() {}
//     }
// }