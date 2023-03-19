use std::fmt;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Display + std::cmp::PartialEq> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn add(&mut self, value: T) {

        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None}));
        }else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node{value, next: None}));
        }
        self.size += 1;
    }

    pub fn insert(&mut self, value: T, position: usize) {
        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None}));
        }else {
            if position == 0 {
                self.head = Some(Box::new(Node { value, next: Some(self.head.take().unwrap())}));
            } else if position >= self.size {
                self.add(value);
                return;
            } else {
                let mut current = self.head.as_mut().unwrap();
                let mut counter = 0;
                while current.next.is_some() {
                    if counter == position {
                        current.next = Some(Box::new(Node{value, next: current.next.take()}));
                        break;
                    }
                    current = current.next.as_mut().unwrap();
                    counter += 1;
                }
            }
        }
        self.size += 1;
    }

    pub fn remove(&mut self, value: T) {
        if self.size == 0 {
            return;
        }else {
            let mut current = self.head.as_mut().unwrap();
            if current.value == value{
                if self.size > 1 {
                    self.head = self.head.as_mut().unwrap().next.take();
                }else{
                    self.head = None;
                }
                self.size -= 1;
                return;
            }

            while current.next.is_some() {
                let next = current.next.as_mut().unwrap();
                if next.value == value {
                    current.next = next.next.take();
                    self.size -= 1;
                    break;
                }else{
                    current = current.next.as_mut().unwrap();
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }
    
}


impl<T: std::fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.size == 0 {
            return write!(f, "[]");
        }
        
        let mut out = String::new();
        let mut current = self.head.as_ref().unwrap();
        out += "[";
        out += &format!("{}", &current.value);
        while current.next.is_some() {
            current = current.next.as_ref().unwrap();
            out += &format!(", {}", &current.value);
        }

        write!(f, "{}]", &out)
    }
}

