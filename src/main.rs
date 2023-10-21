use sha2 ::{ Digest , Sha256 };

 pub trait SumCommitment {
    fn amount (&self) -> u64;
    fn digest (&self) -> [u8; 32];
 }

// To be able to create clones of the object to get siblings
// but ensure they are never affected.
#[derive(Clone)]
// To be able to compare root commitment and calculated commitment
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


struct MyExclusiveAllotmentProof {
    // to be able to actually prove it
    commitment_to_proof:MySumCommitment,
    position: usize,
    // contains all the siblings along the path to 
    // prove that this commitment belongs to the tree
    path: Vec<Option<MySumCommitment>>,
}



impl ExclusiveAllotmentProof<MySumCommitment> for MyExclusiveAllotmentProof 
{
    fn position(&self) -> usize {
        self.position
    }

    // height parameter here is the sibling at a certain height for the proof
    fn sibling(&self, height: u8) -> Option<MySumCommitment> 
    {
        self.path[height as usize].clone()
    }

    // verify that a previously generated proof was generated from 
    // a tree having this root commitment
    fn verify(&self, root_commitment: &MySumCommitment) -> bool {
        // initialize the current commitment with the commitment to proof
        let mut current_commitment = self.commitment_to_proof.clone();
        let mut current_position = self.position;

        // keep hashing with siblings till root is reached
        for commitment_option in &self.path 
        {
            if let Some(sibling_commitment) = commitment_option {
                // Determine which side of the parent commitment to choose
                let mut combined_bytes = Vec::new();
                if current_position % 2 == 0
                {
                    combined_bytes.extend_from_slice(&current_commitment.digest());
                    combined_bytes.extend_from_slice(&sibling_commitment.digest());
                }
                else
                {
                    combined_bytes.extend_from_slice(&sibling_commitment.digest());
                    combined_bytes.extend_from_slice(&current_commitment.digest());
                }

                // calculate the parent commitment by hashing bytes of left and right
                let parent_digest = hash_bytes(&combined_bytes);

                // create the parent commitment
                let parent_commitment = MySumCommitment {
                    amount: current_commitment.amount + sibling_commitment.amount,
                    digest: parent_digest,
                };


                // move up the tree for the parent commitment to be the current node
                current_commitment = parent_commitment;
                current_position /= 2;
            }
        }

        // we reach root and current commitment should equal root commitment.
        // otherwise this transaction wasn't generated from this tree
        &current_commitment == root_commitment
    }

}


pub trait MerkleTree <C: SumCommitment , P: ExclusiveAllotmentProof <C>> 
{
    fn new(values: Vec <u64 >) -> Self;
    fn commit (&self) -> C;
    fn prove (&self , position: usize) -> P;
}




struct MyMerkleTree {
    root_commitment: Option<MySumCommitment>, // Add a field to store the root commitment
    tree: Vec<Vec<MySumCommitment>>,
}

impl MerkleTree<MySumCommitment, MyExclusiveAllotmentProof> for MyMerkleTree 
{
    fn new(values: Vec<u64>) -> Self 
    {
        // tree is a vector of vectors(2D array) such that the inner arrays represent different heights
        // and values inside these arrays represent sum commitments at this height
        let mut tree: Vec<Vec<MySumCommitment>> = Vec::new();
        // covert all these values into sum commitments as these are the leaf nodes
        // so store them in an array called current level (leaf level)
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
            // define a new level to place the parent commitments in it
            // hashes of the children
            let mut next_level: Vec<MySumCommitment> = Vec::new();

            // take 2 by 2 elements from the array (assuming array size 2n so
            // every element will have a pair)
            for chunk in current_level.chunks(2) 
            {
                let left = chunk[0].clone();
                // handles if the array length was odd too (extra)
                // by making it equal left if there was only 1 element in the chunk
                let right = chunk.get(1).cloned().unwrap_or(left.clone());
                // array to cooncatenate the hashes of the left and right
                let mut combined_bytes = Vec::new();
                combined_bytes.extend_from_slice(&left.digest());
                combined_bytes.extend_from_slice(&right.digest());

                // hash the concatenated hash to get the parent hash
                let parent_digest = hash_bytes(&combined_bytes);

                let parent_commitment = MySumCommitment {
                    amount: left.amount() + right.amount(),
                    digest: parent_digest,
                };
                // push this commitment into the higher level array of the tree
                next_level.push(parent_commitment);
            }
            // push this height array into the tree
            tree.push(current_level.clone());
            // make the current level be the next level to keep hashing up till
            // the root is reached (aka current_level.len() = 1)
            current_level = next_level;
        }
        tree.push(current_level.clone());


        MyMerkleTree {
            // set root commitment to be the only remaining element in the
            // current level array
            root_commitment: Some(current_level[0].clone()),
            tree,
        }
    }


    fn commit(&self) -> MySumCommitment 
    {
        // return the root commitment of the tree
        // and expect this value to not be present as
        // someone could call the new method with and
        // empty array of values
        self.root_commitment.clone().expect("Root commitment is not set")
    }




    fn prove(&self, position: usize) -> MyExclusiveAllotmentProof 
    {
        // initialize a new array called path to store the siblings
        // of a certain transaction at all heights
        let mut path: Vec<Option<MySumCommitment>> = Vec::new();
        // get the tree height by the number or arrays in the 
        // tree 2D array
        let tree_height = self.tree.len();
        let mut current_position = position;
        // iterate on each level of the tree
        for level in 0..tree_height - 1 
        {
            // if position is even, then we need the node on its right
            // so we get element at position + 1
            let sibling_index = if current_position % 2 == 0 {
                current_position + 1
            } else {
                // if position is odd, then we need the node on its left
                // element at position - 1
                current_position - 1
            };
            // if it is within the tree range then push it into the siblings path
            if sibling_index < self.tree[level].len() {
                path.push(Some(self.tree[level][sibling_index].clone()));
            } else {
                path.push(None);
            }
            // move up the tree
            current_position /= 2;
        }

        println!("Proof path (siblings up the tree):");
        for (level, commitment) in path.iter().enumerate() 
        {
            print!("Level {}: ", level);
            if let Some(commitment) = commitment
            {
                print!("{}  ", commitment.amount());
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


// a function to print the tree so that we are able to
// visualize it better
fn print_tree(merkle_tree: &MyMerkleTree)
{
    for level in &merkle_tree.tree 
    {
        for commitment in level 
        {
            print!("{} ",&commitment.amount());
        }
        print!("\n");
    }
    println!("---------------------------------------");
}

// function to print a commitment
fn print_commitment(commitment: &MySumCommitment) 
{
    println!("---------------------------------------");
    println!("Commitment amount: {}", commitment.amount());

    let concatenated_string = commitment.digest()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    println!("Commitment digest: {}", concatenated_string);
    println!("---------------------------------------");
}

fn print_root_commitment(commitment: &MySumCommitment) 
{
    println!("Root commitment amount: {}", commitment.amount());

    let concatenated_string = commitment.digest()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    println!("Root commitment digest: {}", concatenated_string);
    println!("---------------------------------------");
}

fn print_sibling_commitment(commitment: &MySumCommitment) 
{
    println!("Getting a sibling using a certain height: \n");
    println!("Sibling commitment amount: {}", commitment.amount());

    let concatenated_string = commitment.digest()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    println!("Sibling commitment digest: {}", concatenated_string);
    println!("---------------------------------------");
}



fn main() 
{
    // test creating commitments for individual values
    let commitment: MySumCommitment = MySumCommitment {
        amount: 10,
        digest: hash_bytes(&10u64.to_be_bytes()), // Assuming hash_bytes is a valid function
    };
    println!("Testing commitment digest and amount: ");
    print_commitment(&commitment);

    let values = vec![10, 20, 90, 40, 50, 60, 86, 66];
    let values2 = vec![6, 9, 30, 100, 52, 26, 45, 11];
    let merkle_tree = MyMerkleTree::new(values.clone());
    let merkle_tree2 = MyMerkleTree::new(values2.clone());

    // Choose a position for which you want to generate the proof
    let position = 3; // Change this to your desired position


    print_tree(&merkle_tree);
    print_tree(&merkle_tree2);


    print_root_commitment(&merkle_tree.commit());

    let proof = merkle_tree.prove(position);
    let proof2 = merkle_tree2.prove(position);

    println!("Proof 1 position: {}", proof.position());
    println!("---------------------------------------");

    if let Some(sibling) = proof.sibling(2) 
    {
        print_sibling_commitment(&sibling);
    }
    else 
    {
        println!("Sibling: None");
    }
    println!("Testing proofs on their actual trees: ");
    println!("Proof generated by tree 1 on tree 1: ");
    println!("Valid proof: {}", proof.verify(&merkle_tree.commit()));
    println!("Proof generated by tree 2 on tree 2: ");
    println!("Valid proof: {}", proof2.verify(&merkle_tree2.commit()));
    println!("\n-------------------true----------------\n");
    println!("Testing proofs on different trees: ");
    println!("Proof generated by tree 2 on tree 1: ");
    println!("Valid proof: {}", proof2.verify(&merkle_tree.commit()));
    println!("Proof generated by tree 1 on tree 2: ");
    println!("Valid proof: {}", proof.verify(&merkle_tree2.commit()));
    println!("\n-------------------false----------------");


}