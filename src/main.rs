use sha2 ::{ Digest , Sha256 };

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
struct MyExclusiveAllotmentProof {
    position: usize,
    sibling: Option<MySumCommitment>,
    path: Vec<Option<MySumCommitment>>,
}



impl ExclusiveAllotmentProof<MySumCommitment> for MyExclusiveAllotmentProof {
    fn position(&self) -> usize {
        self.position
    }

    fn sibling(&self, _height: u8) -> Option<MySumCommitment> {
        // clone to ensure that the sibling doesn't get affected in any way
        self.sibling.clone()
    }

    fn verify(&self, root_commitment: &MySumCommitment) -> bool
    {
        // Verify exclusive allotment by checking if the provided commitment is consistent with the proof.
        // This involves reconstructing the Merkle path from the leaf to the root and comparing it to the
        // provided root commitment.

        let mut current_commitment = match self.sibling.clone() {
            Some(sibling_commitment) => sibling_commitment,
            None => return false,  // A leaf node should have a sibling commitment
        };

        let mut position = self.position;

        // Traverse the Merkle path from the leaf to the root
        for sibling_option in &self.path 

        {
            let sibling_commitment = match sibling_option {
                Some(commitment) => commitment.clone(),
                None => return false,  // Sibling not found in the proof
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


// Structure implementing the MerkleTree trait with the modified structure
struct MyMerkleTree {
    root_commitment: Option<MySumCommitment>, // Add a field to store the root commitment
    tree: Vec<Vec<MySumCommitment>>,
}

impl MerkleTree<MySumCommitment, MyExclusiveAllotmentProof> for MyMerkleTree 
{
    fn new(values: Vec<u64>) -> Self 
    {
        let mut tree: Vec<Vec<MySumCommitment>> = Vec::new();
        let mut current_level: Vec<MySumCommitment> = values
            .iter()
            .map(|&value| {
                MySumCommitment {
                    amount: value,
                    digest: hash_bytes(&value.to_be_bytes()),
                }
            })
            .collect();

        while current_level.len() > 1 {
            let mut next_level: Vec<MySumCommitment> = Vec::new();

            for chunk in current_level.chunks(2) {
                let left = chunk[0].clone();
                let right = chunk.get(1).cloned().unwrap_or(left.clone());

                let mut combined_bytes = Vec::new();
                let mut amount = left.amount;
                combined_bytes.extend_from_slice(&left.digest);
                if left.digest != right.digest {
                    combined_bytes.extend_from_slice(&right.digest);
                    amount += right.amount
                }

                let parent_digest = hash_bytes(&combined_bytes);

                let parent_commitment = MySumCommitment {
                    amount,
                    digest: parent_digest,
                };

                next_level.push(parent_commitment);
            }

            tree.push(current_level.clone());
            current_level = next_level;
        }
        tree.push(current_level.clone());


        MyMerkleTree {
            root_commitment: Some(current_level[0].clone()), // Set root commitment to the final element
            tree,
        }
    }


    fn commit(&self) -> MySumCommitment 
    {
        self.root_commitment.clone().expect("Root commitment is not set")
    }




    fn prove(&self, position: usize) -> MyExclusiveAllotmentProof 
    {
        let mut path: Vec<Option<MySumCommitment>> = Vec::new();
        let tree_height = self.tree.len();
        let mut current_position = position;

        for level in 0..tree_height - 1 
        {
            let sibling_index = if current_position % 2 == 0 {
                current_position + 1
            } else {
                current_position - 1
            };

            if sibling_index < self.tree[level].len() {
                path.push(Some(self.tree[level][sibling_index].clone()));
            } else {
                path.push(None);
            }

            current_position /= 2;
        }

        println!("Proof path:");
        for (level, commitment) in path.iter().enumerate() {
            print!("Level {}: ", level);
            if let Some(commitment) = commitment {
                print!("{}  ", commitment.amount);
            } else {
                print!("None  ");
            }
            println!();
        }

        MyExclusiveAllotmentProof {
            position,
            sibling: path.first().cloned().unwrap_or(None),
            path: path.clone(),
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
    let values = vec![10, 20, 30, 40, 50, 60, 70];
    let merkle_tree = MyMerkleTree::new(values.clone());

    // Choose a position for which you want to generate the proof
    let position = 6; // Change this to your desired position

    // Iterate over the merkle_tree.tree by reference
    for level in &merkle_tree.tree {
        for commitment in level {
            print!("{:?}  ", commitment.amount);
        }
        print!("\n");
    }

    // Call the prove method
    let proof = merkle_tree.prove(position);

    // Now you can work with the `proof` variable as needed
    // For example, you can print information about the proof:

    // ... (Print the details of the generated proof)

    
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


