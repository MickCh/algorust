use std::{cmp::PartialOrd, fmt::Debug};

struct Node<T>
where
    T: Debug,
{
    keys: Vec<T>,
    children: Vec<Option<Node<T>>>,
}

impl<T> Debug for Node<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, ch: {:?}", self.keys, self.children)
    }
}

impl<T> Node<T>
where
    T: Debug,
{
    fn new(value: Option<T>) -> Self {
        match value {
            Some(v) => Node {
                keys: vec![v],
                children: vec![None, None],
            },
            None => Node {
                keys: vec![],
                children: vec![],
            },
        }
    }
}

pub struct BTree<T>
where
    T: PartialOrd + Debug,
{
    enable_debug: bool,
    root: Option<Node<T>>,
    max_degree: usize, //max number of children (not keys)
}

impl<T> Debug for BTree<T>
where
    T: PartialOrd + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(B-Tree)")
    }
}

impl<T> BTree<T>
where
    T: PartialOrd + Debug,
{
    pub fn new(max_degree: usize) -> Self {
        BTree::<T> {
            enable_debug: false,
            root: None,
            max_degree,
        }
    }

    fn add_value(&self, node: &mut Node<T>, pos: usize, value: T) {
        node.keys.insert(pos, value);
        node.children.insert(pos, None)
    }

    fn find_pos(&self, node: &Node<T>, value: &T) -> usize {
        for i in 0..node.keys.len() {
            if *value < node.keys[i] {
                return i;
            }
        }
        node.keys.len()
    }

    fn rebalance(&self, parent: Option<&mut Node<T>>, node: &mut Node<T>) {
        if node.keys.len() < self.max_degree {
            if self.enable_debug {
                println!("Rebalance started but not needed");
            }
            return;
        }
        if self.enable_debug {
            if parent.is_some() {
                println!("Rebalance for child node");
            } else {
                println!("Split for the root node");
            }
            println!("Parent: {:?}", parent);
            println!("Node: {:?}", node);
        }

        //split
        let middle_pos = (node.keys.len() - 1) / 2; //keeps less keys on the left if length is even
                                                    // println!("Split {:?} with middle_pos: {middle_pos}", node.keys);

        match parent {
            None => {
                //this is implementation for root - current node as new root and split to left & right child
                let mut right: Node<T> = Node::new(None);
                right.keys.extend(node.keys.drain(middle_pos + 1..));
                right.children.extend(node.children.drain(middle_pos + 1..));

                let mut left: Node<T> = Node::new(None);
                left.keys.extend(node.keys.drain(..middle_pos));
                left.children.extend(node.children.drain(..=middle_pos));

                node.children.push(Some(left));
                node.children.push(Some(right));
            }
            Some(p) => {
                //rebalance one of the children
                let parent_pos = self.find_pos(p, &node.keys[middle_pos]);

                p.keys.extend(node.keys.drain(middle_pos..=middle_pos));
                let new_length = p.keys.len();
                p.keys.swap(parent_pos, new_length - 1);

                p.children
                    .extend(node.children.drain(middle_pos..=middle_pos));
                let new_length = p.children.len();
                p.children.swap(parent_pos, new_length - 1);

                //move rest of keys
                let mut new_node = Node::new(None);
                new_node.keys.extend(node.keys.drain(middle_pos..));
                new_node
                    .children
                    .extend(node.children.drain(middle_pos + 1..));
                new_node.children.push(None);

                p.children[parent_pos + 1] = Some(new_node);
            }
        }
    }

    fn push_int(&self, parent: Option<&mut Node<T>>, node: &mut Node<T>, value: T) {
        // if self.enable_debug {
        //     println!("P: {:?}", parent);
        // }
        let pos = self.find_pos(node, &value);

        if let Some(mut child) = node.children[pos].take() {
            self.push_int(Some(node), &mut child, value); //TODO: parent should be node, not parent, how can I use it?
            node.children[pos] = Some(child);
        } else {
            if self.enable_debug {
                println!("Adding value {:?}", &value);
            }
            self.add_value(node, pos, value);
        }
        self.rebalance(parent, node);
    }

    pub fn push(&mut self, value: T) {
        let root = match self.root.take() {
            Some(mut rt) => {
                self.push_int(None, &mut rt, value);
                rt
            }
            None => Node::new(Some(value)),
        };
        self.root = Some(root);
    }

    pub fn display(&self) {
        println!("B-Tree");
        for i in 1.. {
            let result = Self::display_child(&self.root, 1, i);
            if !result {
                break;
            }
        }

        println!("---------");
    }

    fn display_child(node: &Option<Node<T>>, level: usize, expected_level: usize) -> bool {
        if level == expected_level {
            println!("L{}: {:?}", level, node);
        }

        let mut result = false;

        if let Some(nn) = node {
            for i in nn.children.iter() {
                result = Self::display_child(i, level + 1, expected_level);

                if i.is_some() && level == expected_level {
                    result = true;
                }
            }
        }
        result
    }

    pub fn enable_debug(&mut self) {
        self.enable_debug = true;
    }
}

// Preemptive splitting in a B-tree is the process of splitting a full node while traversing the tree to ensure the parent node has space for a new value. This prevents the need to recursively split nodes up to the root.
// Here are some details about B-trees and preemptive splitting:

//     When a node is full
//     A node is full when it contains 2*t - 1 entries, where t is the minimum degree.

// How to split a node
// To split a node, create two nodes from the full node's keys, split around the median key. Move the median node to the parent to identify the dividing point between the two new trees.
// Benefits of preemptive splitting
// Preemptive splitting ensures that there is always space in the parent of any potentially split child node. It also avoids traversing a node twice, which can happen if a node is only split when a new key is inserted.
// Disadvantages of preemptive splitting
// Preemptive splitting may result in unnecessary splits
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_tree_0_elem() {
        let b_tree: BTree<i32> = BTree::new(4);
        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_none());
    }

    #[test]
    fn test_b_tree_1_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        b_tree.push(10);

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_empty_child(&r, &vec![10]);
        }
    }

    #[test]
    fn test_b_tree_2_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);

        for i in [10, 20] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_empty_child(&r, &vec![10, 20]);
        }
    }

    #[test]
    fn test_b_tree_3_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_empty_child(&r, &vec![10, 20, 30]);
        }
    }

    #[test]
    fn test_b_tree_4_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![20]);
            check_empty_children(&r, vec![vec![10], vec![30, 40]]);
        }
    }

    #[test]
    fn test_b_tree_5_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40, 50] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![20]);
            check_empty_children(&r, vec![vec![10], vec![30, 40, 50]]);
        }
    }

    #[test]
    fn test_b_tree_6_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40, 50, 60] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![20, 40]);
            check_empty_children(&r, vec![vec![10], vec![30], vec![50, 60]]);
        }
    }

    #[test]
    fn test_b_tree_8_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40, 50, 60, 70, 80] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![20, 40, 60]);
            check_empty_children(&r, vec![vec![10], vec![30], vec![50], vec![70, 80]]);
        }
    }

    #[test]
    fn test_b_tree_10_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![40]);

            assert_eq!(r.children.len(), 2);

            assert!(r.children[0].is_some());
            if let Some(child) = &r.children[0] {
                check_keys(child, vec![20]);
                check_empty_children(child, vec![vec![10], vec![30]]);
            }

            assert!(r.children[1].is_some());
            if let Some(child) = &r.children[1] {
                check_keys(child, vec![60, 80]);
                check_empty_children(child, vec![vec![50], vec![70], vec![90, 100]]);
            }
        }
    }

    #[test]
    fn test_b_tree_13_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 12, 14, 15] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![40]);

            assert_eq!(r.children.len(), 2);

            assert!(r.children[0].is_some());
            if let Some(child) = &r.children[0] {
                check_keys(child, vec![12, 20]);
                check_empty_children(child, vec![vec![10], vec![14, 15], vec![30]]);
            }

            assert!(r.children[1].is_some());
            if let Some(child) = &r.children[1] {
                check_keys(child, vec![60, 80]);
                check_empty_children(child, vec![vec![50], vec![70], vec![90, 100]]);
            }
        }
    }

    #[test]
    fn test_b_tree_16_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 12, 14, 15, 16, 17, 18,
        ] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![40]);

            assert_eq!(r.children.len(), 2);

            assert!(r.children[0].is_some());
            if let Some(child) = &r.children[0] {
                check_keys(child, vec![12, 15, 20]);
                check_empty_children(child, vec![vec![10], vec![14], vec![16, 17, 18], vec![31]]);
                //TODO - BAD 31, should be 30 but it isn't check
            }

            assert!(r.children[1].is_some());
            if let Some(child) = &r.children[1] {
                check_keys(child, vec![60, 80]);
                check_empty_children(child, vec![vec![50], vec![70], vec![90, 100]]);
            }
        }
    }

    #[test]
    fn test_b_tree_17_elem() {
        let mut b_tree: BTree<i32> = BTree::new(4);
        for i in [
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 12, 14, 15, 16, 17, 18, 19,
        ] {
            b_tree.push(i);
        }

        assert_eq!(b_tree.max_degree, 4);
        assert!(b_tree.root.is_some());

        if let Some(r) = b_tree.root {
            check_keys(&r, vec![15, 40]);

            assert_eq!(r.children.len(), 3);

            assert!(r.children[0].is_some());
            if let Some(child) = &r.children[0] {
                check_keys(child, vec![12]);
                check_empty_children(child, vec![vec![10], vec![14]]);
            }

            assert!(r.children[1].is_some());
            if let Some(child) = &r.children[1] {
                check_keys(child, vec![17, 20]);
                check_empty_children(child, vec![vec![16], vec![18, 19], vec![30]]);
            }

            assert!(r.children[2].is_some());
            if let Some(child) = &r.children[2] {
                check_keys(child, vec![60, 80]);
                check_empty_children(child, vec![vec![50], vec![70], vec![90, 100]]);
            }
        }
    }

    fn check_keys(node: &Node<i32>, vector: Vec<i32>) {
        assert_eq!(node.keys.len(), vector.len());

        for i in 0..vector.len() {
            assert_eq!(node.keys[i], vector[i]);
        }
    }

    fn check_empty_children(node: &Node<i32>, vector: Vec<Vec<i32>>) {
        assert_eq!(node.children.len(), vector.len());

        for i in 0..vector.len() {
            assert!(&node.children[i].is_some());

            if let Some(node) = &node.children[i] {
                check_empty_child(node, &vector[i]);
            }
        }
    }

    fn check_empty_child(node: &Node<i32>, vector: &Vec<i32>) {
        assert_eq!(node.keys.len(), vector.len());

        for i in 0..vector.len() {
            assert_eq!(node.keys[i], vector[i]);
        }

        assert_eq!(node.children.len(), vector.len() + 1);
        for ch in node.children.iter() {
            assert!(ch.is_none());
        }
    }
}
