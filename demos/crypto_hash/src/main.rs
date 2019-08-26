use blake2::{Blake2b, Digest};

fn main() {
    let mut j = Blake2b::new();
    let hash = Hash::hash(&mut j, "mycos");
    println!("Hashed {:?}", hash);

    let mut n = Node {
        id: "initial",
        hasher: Blake2b::new(),
        parent: None,
        left_node: None,
        right_node: None,
    };

    let mut m = Node {
        id: "initial",
        hasher: Blake2b::new(),
        parent: None,
        left_node: None,
        right_node: None,
    };

    let mut t = Tree {
        hash: Blake2b::new(),
        members: Vec::new(),
    };

    t.add(m);

    n.hasher.hash(&"foobar");
    t.add(n);

    t.print();
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

struct Tree<'a, T>
where
    T: Hash,
{
    hash: T,
    members: Vec<Node<'a, T>>,
}

impl<'a, T> Tree<'a, T>
where
    T: Hash,
{
    pub fn add(&mut self, node: Node<'a, T>) {
        //generate hash on add
        self.members.push(node);
    }
    pub fn print(&mut self) {
        self.members
            .iter()
            .for_each(|x: &Node<T>| println!("tree node with id {:?}", x.id));
    }
}

struct MerkleTree {}

#[derive(Debug)]
struct Node<'a, V>
where
    V: Hash,
{
    hasher: V,
    id: &'a str,
    parent: Option<Box<Node<'a, V>>>,
    left_node: Option<Box<Node<'a, V>>>,
    right_node: Option<Box<Node<'a, V>>>,
}
