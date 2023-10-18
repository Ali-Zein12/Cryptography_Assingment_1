// hiding is ensured as only tree root is seen which refers nth about the data
// binding is ensured as changing a value of a transaction 
// would result in an incorrectly calculated tree sum

use sha2::{Digest, Sha256};

pub trait SumCommitment {
    // return the amount of the commitment
    fn amount(&self) -> u64;
    // return the hash value of the commitment
    fn digest(&self) -> [u8; 32];
}

pub trait ExclusiveAllotmentProof<C: SumCommitment> {
    // 
    fn position(&self) -> usize;
    fn sibling(&self, height: u8) -> Option<C>;
    //
    fn verify(&self, root_commitment: &C) -> bool;
}

pub trait MerkleTree<C: SumCommitment, P: ExclusiveAllotmentProof<C>> {
    fn new(values: Vec<u64>) -> Self;
    fn commit(&self) -> C;
    fn prove(&self, position: usize) -> P;
}

fn hash_bytes(slice: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(slice);
    hasher.finalize().into()
}

fn u64_to_binary_u8_array(num: u64) -> [u8; 64] {
    let mut binary_array: [u8; 64] = [0; 64];

    for i in 0..64 {
        binary_array[i] = ((num >> i) & 1) as u8;
    }

    binary_array
}

struct MerkleNode {
    hash: [u8; 32],
    value: u64,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new_leaf(value: u64) -> MerkleNode {
        MerkleNode {
            value,
            hash: hash_bytes(&u64_to_binary_u8_array(value)),
            left: None,
            right: None,
        }
    }
    fn new_parent(left: MerkleNode, right: MerkleNode) -> MerkleNode {
        MerkleNode {
            value : left.value + right.value,
            hash: hash_bytes(&concatenate_hashes(left.hash, right.hash)),
            left: None,
            right: None,
        }
    }
}

fn concatenate_hashes(hash1: [u8; 32], hash2: [u8; 32]) -> [u8; 64] {
    let mut concatenated = [0u8; 64];
    // Copy the elements from the first hash into the concatenated array
    for i in 0..32 {
        concatenated[i] = hash1[i];
    }
    // Copy the elements from the second hash into the concatenated array
    for i in 0..32 {
        concatenated[i + 32] = hash2[i];
    }
    concatenated
}





struct MerkleSumTree {
    root: Option<Box<MerkleNode>>,
}

impl MerkleSumTree{
    fn new(root: Option<Box<MerkleNode>>) -> MerkleSumTree{
        MerkleSumTree{
            root,
        }
    }

    // fn addNode(new_node_value: u64) -> MerkleSumTree
    // {
    //     let mut node_list: Vec<MerkleNode> = Vec::new();


    // }
}

impl SumCommitment for MerkleNode {
    fn amount(&self) -> u64
    {
        if self.left.is_none() && self.right.is_none()
        {
            return self.value;
        }
        if self.left.is_none()
        {
            return self.value + self.right.as_ref().unwrap().amount();
        }
        if self.right.is_none()
        {
            return self.value + self.left.as_ref().unwrap().amount();
        }
        self.value + self.left.as_ref().unwrap().amount() + self.right.as_ref().unwrap().amount()
    }
    fn digest(&self) -> [u8; 32]
    {
        unimplemented!();
    }
}



fn main() {
    let mut m = MerkleNode::new_leaf(15);
    for i in m.digest() {
        print!("{:?}", i);
    }
    println!("The Merkle node data is: {}", m.amount());
}



    