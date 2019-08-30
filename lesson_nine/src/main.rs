#[derive(PartialEq)]
struct BinaryNode<'a> {
    value: &'a i32,
    left: Option<Box<BinaryNode<'a>>>,
    right: Option<Box<BinaryNode<'a>>>,
}

impl<'a> BinaryNode<'a> {
    pub fn insert(&mut self, new_value: &'a i32) {
        if self.value == new_value {
            return;
        }

        let target_node = if new_value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_value),
            &mut None => {
                let new_node = BinaryNode {
                    value: new_value,
                    left: None,
                    right: None,
                };

                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

fn main() {
    let mut x = BinaryNode {
        value: &5,
        left: None,
        right: None,
    };
    x.insert(&9);
    x.insert(&2);
    x.insert(&3);

    assert!(
        x == BinaryNode {
            value: &5,
            left: Some(Box::new(BinaryNode {
                value: &2,
                left: None,
                right: Some(Box::new(BinaryNode {
                    value: &3,
                    left: None,
                    right: None
                })),
            })),
            right: Some(Box::new(BinaryNode {
                value: &9,
                left: None,
                right: None
            })),
        }
    );
}
