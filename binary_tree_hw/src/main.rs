struct TreeNode {
    val: i32,
    l: Option<Box<TreeNode>>,
    r: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn insert(&mut self, new_val: i32) {
        if self.val == new_val {
            return
        }

        let target_node = if new_val < self.val {
            println!("Insert {} (new value) at left of {}", new_val, self.val); 
            &mut self.l 
        } 
        else { 
            println!("Insert {} (new value) at right of {}", new_val, self.val); 
            &mut self.r 
        };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = TreeNode { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

fn main () {
    let mut x = TreeNode { val: 10, l: None, r: None };
    println!("The root value is : {}", x.val);
    x.insert(5);
    x.insert(7);
    x.insert(20);

    //Todo: print out BT node
}