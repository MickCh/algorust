#[derive(Debug)]
struct Node<T>
where
    T: std::fmt::Debug,
{
    keys: Vec<T>,
    children: Vec<Option<Node<T>>>,
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
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
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    root: Option<Node<T>>,
    max_degree: usize,
}

impl<T> BTree<T>
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    pub fn new(max_degree: usize) -> Self {
        BTree::<T> {
            root: None,
            max_degree,
        }
    }

    fn add_value(&self, node: &mut Node<T>, pos: usize, value: T) {
        match pos == node.keys.len() {
            false => {
                node.keys.insert(pos, value);
                node.children.insert(pos, None)
            }
            true => {
                node.keys.push(value);
                node.children.push(None)
            }
        }
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
            return;
        }
        println!("Parent: {:?}", parent);
        println!(
            "Children number: {}, keys number: {}",
            node.children.len(),
            node.keys.len()
        );

        //split
        let middle_pos = (node.keys.len() - 1) / 2; //keeps less keys on the left if length is even
        println!("Split {:?} with middle_pos: {middle_pos}", node.keys);

        match parent {
            Some(p) => {
                println!("Ops, something new - I need to implement it");
                //middle goes to parent
                p.keys.extend(node.keys.drain(middle_pos..=middle_pos));
                p.children
                    .extend(node.children.drain(middle_pos..=middle_pos));

                //etc
            }
            None => {
                println!("Split for the root node");

                //this is implementation for root (parent is None)
                let mut left: Node<T> = Node::new(None);
                let mut right: Node<T> = Node::new(None);

                right.keys.extend(node.keys.drain((middle_pos + 1)..));
                right
                    .children
                    .extend(node.children.drain((middle_pos + 1)..));

                left.keys.extend(node.keys.drain(..middle_pos));
                left.children.extend(node.children.drain(..middle_pos));
                left.children.push(None);

                node.children[0] = Some(left);
                node.children.push(Some(right));
            }
        }
    }

    fn push_int(&self, parent: Option<&mut Node<T>>, node: &mut Node<T>, value: T) {
        println!("P: {:?}", parent);
        let pos = self.find_pos(node, &value);

        if let Some(mut child) = node.children[pos].take() {
            self.push_int(Some(node), &mut child, value); //TODO: parent should be node, not parent, how can I use it?
            node.children[pos] = Some(child);
        } else {
            println!("Adding value {:?}", &value);
            self.add_value(node, pos, value);
            self.rebalance(parent, node);
        }
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
        // println!("---------");
        // return;
        println!("B-Tree");
        Self::display_child(&self.root, 1);
        println!("---------");
    }

    fn display_child(node: &Option<Node<T>>, level: usize) {
        println!("L{}: {:?}", level, node);

        if let Some(nn) = node {
            for i in nn.children.iter() {
                Self::display_child(i, level + 1);
            }
        }
    }
}
