use std::{cell::RefCell, rc::Rc};

// type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}
struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    length: i32,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next)
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wring")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_the_transaction_log() {
        let log = TransactionLog::new_empty();

        assert_eq!(log.length, 0);
        assert_eq!(log.head, None);
        assert_eq!(log.tail, None);
    }

    #[test]
    fn it_intrements_the_len() {
        let mut log = TransactionLog::new_empty();

        log.append("String Value".to_string());

        assert_eq!(log.length, 1);
    }

    #[test]
    fn it_returns_the_first_node() {
        let mut log = TransactionLog::new_empty();

        log.append("String Value".to_string());

        assert_eq!(log.head.unwrap(), Node::new("String Value".to_string()));
    }

    #[test]
    fn length_more_than_one() {
        let mut log = TransactionLog::new_empty();

        let mut i = 0;
        while i < 5 {
            log.append(format!("String Value {}", i));
            i += 1;
            println!("{}", log.length)
        }

        assert_eq!(log.length, 5);
    }

    #[test]
    fn pop_one() {
        let mut log = TransactionLog::new_empty();

        let mut i = 0;
        while i < 5 {
            log.append(format!("String Value {}", i));
            i += 1;
        }

        log.pop();
        assert_eq!(log.length, 4);
    }
}
