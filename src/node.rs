use bincode::{Decode, Encode};

const K: usize = 127;

#[derive(Decode, Encode, Clone, Debug, PartialEq)]
pub enum Node {
    Internal(InternalNode),
    Leaf(LeafNode),
}

#[derive(Decode, Encode, Clone, Debug, PartialEq)]
pub struct InternalNode {
    parent: u64,
    keys: [Vec<u8>; K],
    children: [u64; K - 1],
}

#[derive(Decode, Encode, Clone, Debug, PartialEq)]
pub struct LeafNode {
    parent: u64,
    next: u64,
    keys: [Vec<u8>; K],
    values: [Vec<u8>; K],
}

impl Node {
    pub fn default(leaf: bool) -> Self {
        match leaf {
            true => Node::Leaf(LeafNode::default()),
            false => Node::Internal(InternalNode::default()),
        }
    }
}

impl LeafNode {
    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        let index = self.find_insert_index(key);

        self.shift_elements(index);

        self.keys[index] = key.to_vec();
        self.values[index] = value.to_vec();
    }

    fn find_insert_index(&self, key: &[u8]) -> usize {
        self.keys
            .iter()
            .position(|k| k.is_empty() || key <= k)
            .unwrap_or(self.keys.len())
    }

    fn shift_elements(&mut self, index: usize) {
        for i in (index..self.keys.len()).rev().skip(1) {
            self.keys[i] = self.keys[i - 1].clone();
            self.values[i] = self.values[i - 1].clone();
        }
    }
}

impl Default for InternalNode {
    fn default() -> Self {
        return InternalNode {
            parent: 0,
            keys: vec![Vec::new(); K].try_into().unwrap(),
            children: [0; K - 1],
        };
    }
}

impl Default for LeafNode {
    fn default() -> Self {
        return LeafNode {
            parent: 0,
            next: 0,
            keys: vec![Vec::new(); K].try_into().unwrap(),
            values: vec![Vec::new(); K].try_into().unwrap(),
        };
    }
}
