struct Tree {
    root: Option<Node>
}

impl Tree {
    fn insert(&mut self, val: i32){
        match self.root {
            None => self.root = Some(Node {value:val, left:None, right:None}),
            Some(ref mut node) => node.insert(val)
        } 
    }

    fn print(&mut self) {
        match self.root {
            None => println!("Empty Tree"),
            Some(ref mut node) => node.print()
        }
    }
}

struct Node {
	value: i32,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>
}

impl Node {
	fn insert(&mut self, val: i32) {
        if val == self.value {
            return;
        }
		else if val < self.value {
			match self.left {
				None => self.left = Some(Box::new(Node {value:val, left:None, right:None})),
				Some(ref mut node) => node.insert(val)
			}
		}
		else {
			match self.right {
				None => self.right = Some(Box::new(Node {value:val, left:None, right:None})),
				Some(ref mut node) => node.insert(val)
			}
		}
	}
    fn print(&mut self) {
        let left_value = match self.left {
            None => String::from("None"),
            Some(ref mut l) => l.value.to_string()
        };
        let right_value = match self.right {
            None => String::from("None"),
            Some(ref mut r) => r.value.to_string()
        };
        println!("{} => ({}, {})", self.value, left_value, right_value);
        match self.left {
            None => return,
            Some(ref mut l) => l.print()
        };
        match self.right {
            None => return,
            Some(ref mut r) => r.print()
        };
    }
}

/*
             50
           /    \
         25      70
          \      /  \
          40    65   90
                    /
                    85
*/

fn main() {
    let mut tree = Box::new(Tree { root: None });
    
    tree.insert(50);
    tree.insert(25);
    tree.insert(40);
    tree.insert(70);
    tree.insert(65);
    tree.insert(90);
    tree.insert(85);

    tree.print();
}