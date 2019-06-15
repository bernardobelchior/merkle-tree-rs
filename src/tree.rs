use crate::calculate_hash;

type HashFn = Fn(&[u8]) -> Vec<u8>;

#[derive(Debug, Clone)]
pub struct MerkleTree<T> {
    pub root: Node<T>,
}

impl<T> MerkleTree<T> where T: AsRef<[u8]> {
    pub fn new(root: Node<T>) -> MerkleTree<T> {
        MerkleTree {
            root,
        }
    }

    /// Creates a MerkleTree from a vector.
    /// The vector is assumed to have an even number of data points.
    pub fn from_vec(data: Vec<T>) -> MerkleTree<T> {
        let nodes = data.into_iter().map(|d| Node::Leaf(MerkleLeaf {
            hash: calculate_hash(d.as_ref()),
            data: d,
        }));

        let root = MerkleTree::build_until_root(nodes.collect());

        MerkleTree {
            root
        }
    }

    fn build_until_root(mut nodes: Vec<Node<T>>) -> Node<T> {
        if nodes.len() == 1 {
            return nodes.remove(0);
        }

        let mut iter = nodes.into_iter();
        let (mut left, mut right) = (iter.next(), iter.next());
        let mut nodes: Vec<Node<T>> = Vec::new();

        while left.is_some() {
            nodes.push(Node::Node(MerkleNode::new(&calculate_hash, left.unwrap(), right.unwrap())));

            left = iter.next();
            right = iter.next();
        }

        MerkleTree::build_until_root(nodes)
    }
}

impl<T> PartialEq for MerkleTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.root.hash().eq(other.root.hash())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node<T> {
    Node(MerkleNode<T>),
    Leaf(MerkleLeaf<T>),
}

impl<T> Node<T> {
    fn hash(&self) -> &Vec<u8> {
        match self {
            Node::Node(n) => &n.hash,
            Node::Leaf(l) => &l.hash
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MerkleNode<T> {
    pub hash: Vec<u8>,
    pub left: Box<Node<T>>,
    pub right: Box<Node<T>>,
}

impl<T> MerkleNode<T> where T: AsRef<[u8]> {
    pub fn new<'a>(hash_fn: &HashFn, left: Node<T>, right: Node<T>) -> MerkleNode<T> {
        let mut concat = left.hash().clone();
        concat.extend_from_slice(right.hash());

        MerkleNode {
            left: Box::new(left),
            right: Box::new(right),
            hash: hash_fn(concat.as_slice()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MerkleLeaf<T> {
    pub hash: Vec<u8>,
    pub data: T,
}

impl<'a, T> From<T> for MerkleLeaf<T>
    where T: AsRef<[u8]> {
    fn from(data: T) -> Self {
        let hash = calculate_hash(data.as_ref());

        MerkleLeaf {
            data,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hashes_leaf_data_correctly() {
        let data = "test_data".as_bytes();
        let merkle_leaf = MerkleLeaf::from(data);

        assert_eq!(merkle_leaf.data, data);
        assert_eq!(merkle_leaf.hash, vec![249, 124, 220, 236, 144, 165, 213, 107, 109, 161, 237, 2, 189, 209, 247, 92, 37, 154, 19, 252, 148, 61, 177, 152, 191, 210, 99, 37, 220, 74, 109, 173, 226, 207, 47, 193, 127, 30, 50, 125, 215, 44, 65, 50, 171, 129, 48, 75, 122, 77, 104, 172, 67, 6, 244, 15, 43, 221, 31, 185, 131, 100, 229, 140]);
    }

    #[test]
    fn it_hashes_node_from_leaves() {
        let merkle_leaf = Node::Leaf(MerkleLeaf::from("test_data".as_bytes()));
        let merkle_leaf_2 = Node::Leaf(MerkleLeaf::from("test_data".as_bytes()));

        let merkle_node = MerkleNode::new(&calculate_hash, merkle_leaf.clone(), merkle_leaf_2.clone());

        assert_eq!(*merkle_node.left, merkle_leaf);
        assert_eq!(*merkle_node.right, merkle_leaf_2);
        assert_eq!(merkle_node.hash, vec![86, 212, 98, 60, 48, 40, 135, 164, 65, 171, 245, 66, 46, 100, 170, 222, 5, 167, 146, 71, 129, 154, 168, 28, 81, 169, 177, 176, 169, 44, 219, 22, 192, 226, 56, 186, 175, 151, 152, 182, 105, 166, 119, 22, 182, 40, 139, 10, 210, 153, 159, 114, 133, 194, 226, 99, 178, 148, 175, 2, 224, 65, 189, 34]);
    }

    #[test]
    fn it_builds_merkle_tree_from_vec() {
        let merkle_tree = MerkleTree::from_vec(vec!["a", "b", "c", "d"]);

        assert_eq!(merkle_tree.root,
                   Node::Node(MerkleNode::new(
                       &calculate_hash,
                       Node::Node(MerkleNode::new(&calculate_hash, Node::Leaf(MerkleLeaf::from("a")), Node::Leaf(MerkleLeaf::from("b")))),
                       Node::Node(MerkleNode::new(&calculate_hash, Node::Leaf(MerkleLeaf::from("c")), Node::Leaf(MerkleLeaf::from("d"))),
                       ))));
    }

    #[test]
    #[should_panic]
    fn it_panics_when_building_merkle_tree_from_odd_len_vec() {
        let merkle_tree = MerkleTree::from_vec(vec!["a", "b", "c"]);
    }


    #[test]
    fn it_compares_merkle_trees() {
        let merkle_tree = MerkleTree::from_vec(vec!["a", "b", "c", "d"]);
        let eq_tree = MerkleTree::from_vec(vec!["a", "b", "c", "d"]);
        let diff_tree = MerkleTree::from_vec(vec!["d", "b", "c", "d"]);

        assert_eq!(merkle_tree, eq_tree);
        assert_ne!(merkle_tree, diff_tree);
    }
}
