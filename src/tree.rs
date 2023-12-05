use crate::disk::{Disk, DiskError};
use crate::node::Node;
use crate::page::{Page, PageError};

#[derive(thiserror::Error, Debug)]
pub enum TreeError {
    #[error(transparent)]
    DiskError(#[from] DiskError),

    #[error(transparent)]
    PageError(#[from] PageError),
}

pub struct Tree {
    disk: Disk,
    root: Node,
}

impl Tree {
    pub fn new(path: &str) -> Result<Tree, TreeError> {
        let mut disk = Disk::new(path)?;

        let root = match disk.size() {
            0 => {
                let node = Node::default(true);
                let page = Page::try_from(&node)?;

                disk.allocate(1)?;
                disk.write(0, &page);

                node
            }
            _ => disk.read(0)?.try_into()?,
        };

        Ok(Tree { disk, root })
    }

    pub fn search(&self, key: &[u8]) {}

    pub fn insert(&mut self, key: &[u8], value: &[u8]) {
        self.insert_recursive(&mut self.root.clone(), key, value);
    }

    fn insert_recursive(&self, node: &mut Node, key: &[u8], value: &[u8]) {
        match node {
            Node::Internal(_) => {}
            Node::Leaf(node) => node.insert(key, value),
        }
    }

    fn search_recursive(&self, node: &Node, key: &[u8]) {
        match node {
            Node::Internal(_) => {}
            Node::Leaf(node) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let temp_path = "./tmp.bin";
        let _ = std::fs::remove_file(temp_path);

        Tree::new(temp_path).expect("Failed to create Tree");

        let _ = std::fs::remove_file(temp_path);
    }
}
