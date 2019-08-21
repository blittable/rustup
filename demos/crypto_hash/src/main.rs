use blake2::{Blake2b, Digest};

fn main() {
    let mut j = Blake2b::new();
    let hash = Hash::hash(&mut j, "mycos");
    println!("Hashed {:?}", hash);
}

pub trait Hash {
    fn hash(&mut self, input: &str) -> Vec<u8>;
}

impl Hash for Blake2b {
    fn hash(&mut self, hash_input: &str) -> Vec<u8> {
        let mut j = Blake2b::new();
        &j.input(hash_input);
        let hash = &j.result();
        hash.to_vec()
    }
}
