#[derive(PartialEq)]
struct Node<'a> {
    value: &'a i32,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn insert(&mut self, new_value: &'a i32) {
        if self.value == new_value {
            println!("Already have node {:?} in tree.", new_value);
            return
        }
        let next_node = if new_value < self.value { &mut self.left } else { &mut self.right };
        match next_node {
            Some(ref mut child_node) => child_node.insert(new_value),
            None => {
                let new_node = Node { value: new_value, left: None, right: None };
                let box_node = Some(Box::new(new_node));
                *next_node = box_node;
            }
        }
    }

    //////////////////
    //Extra for late//
    //////////////////
    pub fn fine_node(&mut self, find_value: &'a i32)
    {
        if self.value == find_value {
            println!("Found node {:?}.", self.value);
            return
        }
        if self.value < find_value {
            match self.right {
                Some(ref mut child_node) => {
                    println!("Current node is {:?} then go right.", self.value);
                    child_node.fine_node(find_value)
                },
                None => { println!("Not found node {:?} in tree.", find_value); }
            } 
        }
        else {
            match self.left {
                Some(ref mut child_node) => {
                    println!("Current node is {:?} then go left.", self.value);
                    child_node.fine_node(find_value)
                },
                None => { println!("Not found node {:?} in tree.", find_value); }
            } 
        }
    }
}

fn main () {
    let mut tree = Node { value: &4, left: None, right: None };
    tree.insert(&8);
    tree.insert(&6);
    tree.insert(&2);
    tree.insert(&10);
    tree.insert(&6); //Exist Node
    
    assert!(tree == Node {
        value: &4,
        left: Some(Box::new(Node { value: &2, left: None, right: None })),
        right: Some(Box::new(Node { 
            value: &8, 
            left: Some(Box::new(Node { value: &6, left: None, right: None })), 
            right: Some(Box::new(Node { value: &10, left: None, right: None })) })),
    });

    /////////////
    //Find node//
    /////////////
    println!("");
    tree.fine_node(&6);
    println!("");
    tree.fine_node(&12); //Not Found
}
