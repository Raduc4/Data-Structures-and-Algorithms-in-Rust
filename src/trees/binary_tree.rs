use std::mem;

type Tree = Option<Box<Node>>;

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Tree,
    pub length: u64,
}

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(dev: IoTDevice) -> Option<Box<Self>> {
        Some(Box::new(Node {
            dev,
            left: None,
            right: None,
        }))
    }
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerial_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerial_id)
    }

    pub fn find_r(&self, node: &Tree, numerial_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerial_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerial_id {
                    self.find_r(&n.left, numerial_id)
                } else {
                    self.find_r(&n.right, numerial_id)
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinarySearchTree, IoTDevice};

    #[test]
    fn where_does_it_come() {
        let mut itt = BinarySearchTree::new();
        itt.add(IoTDevice {
            numerical_id: 2,
            address: String::from("Sad"),
        });
    }
}
