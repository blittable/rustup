#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    //Add root node
    //Add a node to the left

    const ROOT_ID: i32 = 2000;

    let mut tree: Tree = Tree { nodes: Vec::new() };

    let root: Node = Node {
        id: ROOT_ID,
        left_node: None,
        right_node: None,
        parent: None,
    };

    let child: Node = Node {
        id: 2000,
        left_node: None,
        right_node: None,
        parent: None,
    };

    tree.add_node(&root);
}

pub struct Tree<'a> {
    nodes: Vec<Option<&'a Node>>,
}

pub struct Node {
    id: i32,
    parent: Option<Box<Node>>,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl<'a> Tree<'a> {
    pub fn add_node(&mut self, node: &'a Node) {
        self.nodes.push(Some(&node));
    }
    pub fn node_count(&self) -> i32 {
        &self.nodes.iter().filter_map(|&x| x) Some(x).collect().len()
    }
}
