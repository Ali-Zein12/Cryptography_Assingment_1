use sha2 ::{ Digest , Sha256 };
use std::collections::VecDeque;

 pub trait SumCommitment {
 fn amount (&self) -> u64;
 fn digest (&self) -> [u8; 32];
 }

#[derive(Clone)]
 struct MySumCommitment{
    amount: u64,
    digest: [u8; 32],
 }

 impl SumCommitment for MySumCommitment {
     fn amount (&self) -> u64
     {
        self.amount
     }
     fn digest (&self) -> [u8; 32]
     {
         self.digest
     }
 }

 pub trait ExclusiveAllotmentProof <C: SumCommitment > {
 fn position (&self) -> usize;
 fn sibling (&self , height: u8) -> Option <C>;
 fn verify (&self , root_commitment: &C) -> bool;
 }


 // Structure implementing the 'ExclusiveAllotmentProof' trait
struct MyExclusiveAllotmentProof {
    position: usize,
    sibling: Option<MySumCommitment>,
}

impl ExclusiveAllotmentProof<MySumCommitment> for MyExclusiveAllotmentProof {
    fn position(&self) -> usize {
        self.position
    }

    fn sibling(&self, _height: u8) -> Option<MySumCommitment> {
        self.sibling.clone()
    }

    fn verify(&self, root_commitment: &MySumCommitment) -> bool {
        // Implement your verification logic here
        // Compare the provided commitment with the calculated commitment based on the proof
        // Return true if it's valid, false otherwise
        // You need to compute the Merkle path from the leaf to the root and verify it
        unimplemented!()
    }
}

// Structure implementing the 'MerkleTree' trait
struct MyMerkleTree {
    values: Vec<u64>,
    commitments: Vec<MySumCommitment>,
}

impl MerkleTree<MySumCommitment, MyExclusiveAllotmentProof> for MyMerkleTree {
    fn new(values: Vec<u64>) -> Self {
        // Construct the Merkle tree from the given values
        // Calculate the commitments for each leaf node
        // You can use the provided hash_bytes function
        // You also need to build the tree structure
        unimplemented!()
    }

    fn commit(&self) -> MySumCommitment {
        // Return the root commitment of the Merkle tree
        // It's the last element in the commitments vector
        unimplemented!()
    }

    fn prove(&self, position: usize) -> MyExclusiveAllotmentProof {
        // Generate a proof for the element at the specified position
        // You need to return an ExclusiveAllotmentProof containing the sibling and position
        // You also need to calculate the Merkle path from the leaf to the root
        unimplemented!()
    }
}


 pub trait MerkleTree <C: SumCommitment , P: ExclusiveAllotmentProof <C>> {
 fn new(values: Vec <u64 >) -> Self;
 fn commit (&self) -> C;
 fn prove (&self , position: usize) -> P;
 }


 // struct merkle_tree
 // {
 //    root: Option<Box<merkle_node>>
 // }

 // impl MerkleTree for merkle_tree {
 //     fn new(values: Vec <u64 >) -> Self
 //     {

 //     }
 // }

 fn hash_bytes(slice: &[u8]) -> [u8; 32] {
 let mut hasher = Sha256 ::new();
 hasher.update(slice);
 hasher.finalize ().into()
 }

 fn main() {
    println!("hello world");

 }