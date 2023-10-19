use sha2 ::{ Digest , Sha256 };
use std::collections::VecDeque;

 pub trait SumCommitment {
    fn amount (&self) -> u64;
    fn digest (&self) -> [u8; 32];
 }

#[derive(Clone)]
#[derive(Debug)]
 struct MySumCommitment{
    amount: u64,
    digest: [u8; 32],
 }

 impl SumCommitment for MySumCommitment {
    fn amount (&self) -> u64
    {
       self.amount
    }
    // it is not mutable... so assuming it just returns the hash
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
#[derive(Debug)]
struct MyExclusiveAllotmentProof {
    position: usize,
    sibling: Option<MySumCommitment>,
}

impl ExclusiveAllotmentProof<MySumCommitment> for MyExclusiveAllotmentProof {
    fn position(&self) -> usize {
        self.position
    }

    fn sibling(&self, _height: u8) -> Option<MySumCommitment> {
        // clone to ensure that the sibling doesn't get affected in any way
        self.sibling.clone()
    }

    fn verify(&self, root_commitment: &MySumCommitment) -> bool {
        // Verify exclusive allotment by checking if the provided commitment is consistent with the proof.
        // This involves reconstructing the Merkle path from the leaf to the root and comparing it to the
        // provided root commitment.

        // Start with the provided leaf commitment
        let mut current_commitment = match self.sibling.clone() {
            Some(sibling_commitment) => sibling_commitment,
            None => return false, // A leaf node should have a sibling commitment
        };

        let mut position = self.position;

        // Traverse the Merkle path from the leaf to the root
        while position > 0 {
            // Calculate the sibling position at the current level
            let sibling_position = if position % 2 == 0 { position - 1 } else { position + 1 };

            // Retrieve the sibling commitment
            let sibling_commitment = match self.sibling(sibling_position.trailing_zeros() as u8) {
                Some(commitment) => commitment,
                None => return false, // Sibling not found in the proof
            };

            // Combine the current commitment and the sibling commitment to calculate the parent commitment
            let mut combined_bytes = Vec::new();
            combined_bytes.extend_from_slice(&current_commitment.digest);
            combined_bytes.extend_from_slice(&sibling_commitment.digest);
            let parent_digest = hash_bytes(&combined_bytes);

            // Move up the tree
            current_commitment = MySumCommitment {
                amount: current_commitment.amount + sibling_commitment.amount,
                digest: parent_digest,
            };

            position /= 2;
        }

        // Verify that the computed root commitment matches the provided root commitment
        current_commitment.digest == root_commitment.digest
    }
}


 pub trait MerkleTree <C: SumCommitment , P: ExclusiveAllotmentProof <C>> {
     fn new(values: Vec <u64 >) -> Self;
     fn commit (&self) -> C;
     fn prove (&self , position: usize) -> P;
 }


// Structure implementing the 'MerkleTree' trait
struct MyMerkleTree {
    commitments: Vec<MySumCommitment>,
    root_commitment: MySumCommitment,
}

impl MerkleTree<MySumCommitment, MyExclusiveAllotmentProof> for MyMerkleTree 
{
    fn new(values: Vec<u64>) -> Self 
    {
        let mut commitments: Vec<MySumCommitment> = vec![];

        for &value in &values 
        {
            let leaf_commitment = MySumCommitment{
                amount: value,
                digest: hash_bytes(&value.to_be_bytes()),
            };
            print!("{}  ", leaf_commitment.amount);
            commitments.push(leaf_commitment);
        }

        let mut next_level: Vec<MySumCommitment> = commitments.clone();
        let mut current_level: Vec<MySumCommitment> = vec![];

        while next_level.len() > 1 {
            println!("\n");
            for chunk in next_level.chunks(2) {
                let left = chunk[0].clone();
                let right = chunk.get(1).cloned().unwrap_or(left.clone());
                let mut combined_bytes = Vec::new();
                combined_bytes.extend_from_slice(&left.digest);
                let mut amount_odd = left.amount;
                if left.digest != right.digest 
                {
                    combined_bytes.extend_from_slice(&right.digest);
                    amount_odd += right.amount;
                }

                let parent_digest = hash_bytes(&combined_bytes);

                let parent_commitment = MySumCommitment{
                    amount: amount_odd,
                    digest: parent_digest,
                };
                print!("{:?} ", parent_commitment.amount);



                current_level.push(parent_commitment);
            }

            next_level = current_level.clone();
            current_level.clear();
        }

        MyMerkleTree {
            commitments: next_level.clone(),
            root_commitment: next_level[0].clone(),
        }
    }

    fn commit(&self) -> MySumCommitment {
        self.root_commitment.clone()
    }

    fn prove(&self, position: usize) -> MyExclusiveAllotmentProof {
        // Generate a proof for the element at the specified position
        // You need to return an ExclusiveAllotmentProof containing the sibling and position
        // You also need to calculate the Merkle path from the leaf to the root
        let mut path = VecDeque::new();
        let mut position = position;
        let mut height = 0;
        while position > 0 {
            let sibling_position = if position % 2 == 0 { position - 1 } else { position + 1 };
            let sibling = if sibling_position < self.commitments.len() {
                Some(self.commitments[sibling_position].clone())
            } else {
                None
            };


            path.push_back(sibling);
            position = position / 2;
            height += 1;
        }
        MyExclusiveAllotmentProof {
            position,
            sibling: path.pop_front().unwrap(),
        }
    }
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
    // Example usage:
    let values = vec![10, 20, 30, 40];
    let merkle_tree = MyMerkleTree::new(values.clone());


}





// fn main() 
// {
//     // Test creating commitments for individual values
//     let values: Vec<u64> = vec![10, 20, 30, 40];

//     for value in values {
//         // Calculate the hash of the value
//         let value_bytes = value.to_be_bytes();
//         let hash = hash_bytes(&value_bytes);

//         // Convert the hash to a hexadecimal string
//         let hash_hex = hex::encode(&hash);

//         // Create a MySumCommitment
//         let commitment = MySumCommitment {
//             amount: value,
//             digest: hash,
//         };

//         println!("Value: {}, Hash (Hex): {}", value, hash_hex);
//     }
//  }