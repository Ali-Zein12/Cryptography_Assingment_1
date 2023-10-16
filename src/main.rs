/*    
Commitments: To ensure that Alice's ledger is accurate and that individual client balances
are exclusively allotted and not double-counted in the total Bt, cryptographic commitments are used.
A commitment is a cryptographic representation of a value or set of values that hides the
actual data but allows for later verification.
In this project, commitments are used to commit to the ledger,
ensuring that it cannot be altered without detection.
*/

use sha2 ::{ Digest , Sha256 };

// not 64 as max number represented by 32 bits is 2,147,483,647
const MAXIMUM_SUPPLY: u32 = 1000000000;


/* SumCommitment is like an interface such that whatever function implements that
 interface, it has to implement its unimplemented methods*/
pub trait SumCommitment 
{
    // self is the instance of the struct that will implement this trait
    fn amount (& self) -> u64; 
    fn digest (& self) -> [u8; 32]; 
}

/* basically what this "<C: SumCommitment >" is is that
 whatever structure is implementing the ExclusiveAllotmentProof
 needs to implement the SumCommitment trait*/
pub trait ExclusiveAllotmentProof <C: SumCommitment > 
{ 
    fn position (& self) -> usize;
    fn sibling (&self , height: u8) -> Option <C>;
    fn verify (&self , root_commitment : &C) -> bool;
}


pub trait MerkleTree <C: SumCommitment , P: ExclusiveAllotmentProof <C>> 
{
    fn new( values: Vec <u64 >) -> Self;
    fn commit (& self) -> C;
    fn prove (&self , position : usize) -> P;
}


// this function returns an array of 32 unsigned 8 bit items ( -> [u8; 32])
 fn hash_bytes (slice: &[u8]) -> [u8; 32] 
 {
    let mut hasher = Sha256 :: new ();
    hasher . update(slice); hasher . finalize ().into ()
 }


 // Define a struct for a node in the Merkle-Sum tree.
 // option as this value is optional (leaf nodes don't have children for example)
/*
     trees can be of huge sizes. Thus, we use the box to specify that this is going to
     be allocated in the heap of the RAM as its length is not fixed and it varies according
     to the number of children(depth of the tree)
 */
struct MerkleNode 
{
    value: u64,               // The client's balance.
    hash: [u8; 32],           // Hash of the 2 children.
    total_balance: u64,       // The total sum of the two subtrees of 2 children
    left: Option<Box<MerkleNode>>,  // Left child node.
    right: Option<Box<MerkleNode>>, // Right child node.
}

struct MerkleSumTree 
{
    root: Option<Box<MerkleNode>>
}
 
impl SumCommitment for MerkleSumTree 
{
    fn amount(&self) -> u64 
    {
        unimplemented!();
        // let total_sum = calc_sum(&self.root); // Assuming `root` is of type `Option<Box<MerkleNode>>`
        // total_sum
    }
    fn digest(&self) -> [u8; 32] 
    {
        
        
    }
}



fn main()
{
    // not 64 as max number represented by 32 bits is 2,147,483,647
    print!("The MAXIMUM SUPPLY is {}", MAXIMUM_SUPPLY);

}



// fn calc_sum(curr_node: &Option<Box<MerkleNode>>) -> u64 {
//     match curr_node {
//         Some(node) => {
//             let left_sum = calc_sum(&node.left);
//             let right_sum = calc_sum(&node.right);
//             node.value + left_sum + right_sum
//         }
//         None => 0,
//     }
    
// }



    




