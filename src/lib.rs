fn calculate_hash<T: AsRef<[u8]>>(data: &T) -> Vec<u8> {
    use blake2::{Blake2b, Digest};

    let mut hasher = Blake2b::new();
    hasher.input(data);
    hasher.result().to_vec()
}

struct MerkleTree<T> {
    root: Node<T>
}

enum Node<T> {
    Node(MerkleNode<T>),
    Leaf(MerkleLeaf<T>),
}

struct MerkleNode<T> {
    left: Box<Node<T>>,
    right: Box<Node<T>>,
}

struct MerkleLeaf<T> {
    hash: Vec<u8>,
    data: T,
}

impl<T> From<T> for MerkleLeaf<T>
    where T: AsRef<[u8]> {
    fn from(data: T) -> Self {
        let hash = calculate_hash(&data);

        MerkleLeaf {
            data,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MerkleLeaf;

    #[test]
    fn it_hashes_leaf_data_correctly() {
        let data = "test_data".to_string();
        let merkle_leaf = MerkleLeaf::from(data.clone());

        assert_eq!(merkle_leaf.data, data);
        assert_eq!(merkle_leaf.hash, vec![249, 124, 220, 236, 144, 165, 213, 107, 109, 161, 237, 2, 189, 209, 247, 92, 37, 154, 19, 252, 148, 61, 177, 152, 191, 210, 99, 37, 220, 74, 109, 173, 226, 207, 47, 193, 127, 30, 50, 125, 215, 44, 65, 50, 171, 129, 48, 75, 122, 77, 104, 172, 67, 6, 244, 15, 43, 221, 31, 185, 131, 100, 229, 140]);
    }
}
