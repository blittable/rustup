use std::cmp;

fn main() {
    let mut parent = Node {
        id: 0,
        value: 0,
        left: None,
        right: None
    };
    let node1 = Node {
        id: 1,
        value: -4,
        left: None,
        right: None
    };
    let node2 = Node {
        id: 2,
        value: 4,
        left: None,
        right: None
    };
    let node3 = Node {
        id: 3,
        value: -8,
        left: None,
        right: None
    };
    let node4 = Node {
        id: 4,
        value: 8,
        left: None,
        right: None
    };
    let node5 = Node {
        id: 5,
        value: -2,
        left: None,
        right: None
    };
    let node6 = Node {
        id: 6,
        value: 2,
        left: None,
        right: None
    };
    
    parent.insert(node1);
    parent.insert(node2);
    parent.insert(node3);
    parent.insert(node4);
    parent.insert(node5);
    parent.insert(node6);
    /*parent.insert(Node {
        id: 7,
        value: 16,
        left: None,
        right: None
    });*/
}

struct Node {
    id: i32,
    value: i32,
    left: Option<*mut Node>,
    right: Option<*mut Node>
}

trait Feature {
    fn insert(&mut self, node: Node);
}

impl Feature for Node {
    fn insert(&mut self, node: Node) {
        let mut ptr_parent: Option<*mut Node>;
        println!("Parent is : {}", &self.id);
        
        let inner_insertion = |ptr_direction: Option<*mut Node>, n: Node| {
            match &ptr_direction {
                None => {
                    let ptr = Box::into_raw(Box::new(n));
                    let val = unsafe { &*ptr };
                    println!("Insert to Parent : {} Node : {}({})", self.id, val.id, val.value);
                    return Some(ptr);
                },
                Some(v) => {
                    let mut val = unsafe { Box::from_raw(*v) };
                    val.insert(n);
                    return None;
                }
            };
        };

        if &self.value < &node.value {
            match inner_insertion(self.right, node) {
                None => {},
                Some(ptr) => self.right = Some(ptr)
            }
        }
        else {
            match inner_insertion(self.left, node) {
                None => {},
                Some(ptr) => self.left = Some(ptr)
            }
        }
    }
}