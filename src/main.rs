use sha2::{Digest, Sha256};

pub trait SumCommitment {
    fn amount(&self) -> u64;
    fn digest(&self) -> [u8; 32];
}

pub trait ExclusiveAllotmentProof<C: SumCommitment> {
    fn position(&self) -> usize;
    fn sibling(&self, height: u8) -> Option<C>;
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
    sum: u64,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(sum: u64) -> MerkleNode {
        MerkleNode {
            sum,
            hash: hash_bytes(&u64_to_binary_u8_array(sum)),
            left: None,
            right: None,
        }
    }
}

struct MerkleSumTree {
    merkle_root: Option<Box<MerkleNode>>,
}

fn main() {
    let m = MerkleNode::new(15);
    let mut count = 0;
    for i in m.hash {
        print!("{:?}", i);
        count+=1;
    }
    println!("The Merkle node data is: {}", m.sum);
    println!("count is: {}", count);
}
