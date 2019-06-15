use tree::{MerkleLeaf, MerkleNode, MerkleTree, Node};

mod tree;

fn calculate_hash(data: &[u8]) -> Vec<u8> {
    use blake2::{Blake2b, Digest};

    let mut hasher = Blake2b::new();
    hasher.input(data);
    hasher.result().to_vec()
}
