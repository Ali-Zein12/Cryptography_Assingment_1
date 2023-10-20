use sha2 ::{ Digest , Sha256 };

 pub trait SumCommitment {
    fn amount (&self) -> u64;
    fn digest (&self) -> [u8; 32];
 }

#[derive(Clone)]
#[derive(PartialEq)]
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
    commitment_to_proof:MySumCommitment,
    position: usize,
    path: Vec<Option<MySumCommitment>>,
}



impl ExclusiveAllotmentProof<MySumCommitment> for MyExclusiveAllotmentProof {
    fn position(&self) -> usize {
        self.position
    }

    fn sibling(&self, height: u8) -> Option<MySumCommitment> {
        self.path[height as usize].clone()
    }


    fn verify(&self, root_commitment: &MySumCommitment) -> bool {
    // Initialize the current commitment with the commitment to proof
    let mut current_commitment = self.commitment_to_proof.clone();

    // Iterate over the path from leaf to root
    for commitment_option in &self.path {
        if let Some(sibling_commitment) = commitment_option {
            // Determine which side of the parent commitment to choose
            let mut combined_bytes = [0u8; 64]; // Fixed-size array

            // Copy the digests into combined_bytes
            combined_bytes[0..32].copy_from_slice(&current_commitment.digest);
            combined_bytes[32..64].copy_from_slice(&sibling_commitment.digest);

            // Calculate the parent commitment
            let parent_digest = hash_bytes(&combined_bytes);

            // Create the parent commitment
            let parent_commitment = MySumCommitment {
                amount: current_commitment.amount + sibling_commitment.amount,
                digest: parent_digest,
            };

            // Update the current commitment to be the parent commitment
            current_commitment = parent_commitment;
        }
    }

    // At this point, current_commitment should be the root commitment
    &current_commitment == root_commitment
}

}


pub trait MerkleTree <C: SumCommitment , P: ExclusiveAllotmentProof <C>> 
{
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

        while current_level.len() > 1 
        {
            let mut next_level: Vec<MySumCommitment> = Vec::new();

            for chunk in current_level.chunks(2) 
            {
                let left = chunk[0].clone();
                let right = chunk.get(1).cloned().unwrap_or(left.clone());

                let mut combined_bytes = Vec::new();
                let mut amount = left.amount;
                combined_bytes.extend_from_slice(&left.digest);
                if left.digest != right.digest 
                {
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
        for (level, commitment) in path.iter().enumerate() 
        {
            print!("Level {}: ", level);
            if let Some(commitment) = commitment
            {
                print!("{}  ", commitment.amount);
            }
            else 
            {
                print!("None  ");
            }
            println!();
        }
        println!("---------------------------------------");

        MyExclusiveAllotmentProof {
            commitment_to_proof:self.tree[0][position].clone(),
            position,
            path: path.clone(),
        }
    }
}


 fn hash_bytes(slice: &[u8]) -> [u8; 32] 
 {
    let mut hasher = Sha256 ::new();
    hasher.update(slice);
    hasher.finalize ().into()
 }


fn printTree(merkleTree: &MyMerkleTree)
{
    for level in &merkleTree.tree 
    {
        for commitment in level 
        {
            print!("{:?}  ", commitment.amount);
        }
        print!("\n");
    }
    println!("---------------------------------------");
}



fn main() 
{
    // // Test creating commitments for individual values
    // let values: Vec<u64> = vec![10, 20, 30, 40];

    // for value in values 
    // {
    //     // Calculate the hash of the value
    //     let value_bytes = value.to_be_bytes();
    //     let hash = hash_bytes(&value_bytes);

    //     // Convert the hash to a hexadecimal string
    //     let hash_hex = hex::encode(&hash);

    //     // Create a MySumCommitment
    //     let commitment = MySumCommitment {
    //         amount: value,
    //         digest: hash,
    //     };

    //     println!("Value: {}, Hash (Hex): {}", value, hash_hex);
    // }

    let values = vec![10, 20, 30, 40, 50, 60];
    let values2 = vec![1, 2, 3, 4, 5, 6];
    let merkle_tree = MyMerkleTree::new(values.clone());
    let merkle_tree2 = MyMerkleTree::new(values2.clone());

    // Choose a position for which you want to generate the proof
    let position = 0; // Change this to your desired position


    printTree(&merkle_tree);
    printTree(&merkle_tree2);

    // Call the prove method
    let proof = merkle_tree.prove(position);
    let proof2 = merkle_tree2.prove(position);
    // if let Some(sibling) = proof.sibling(2) 
    // {
    //     println!("Sibling Amount: {}", sibling.amount);
    // } 
    // else 
    // {
    //     println!("Sibling: None");
    // }

    println!("Valid transaction: {}", proof.verify(merkle_tree.root_commitment.as_ref().unwrap()));
    println!("Valid transaction: {}", proof2.verify(merkle_tree.root_commitment.as_ref().unwrap()));
}