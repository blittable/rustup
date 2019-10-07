use std::iter;

fn main() {}

use std::fmt;
use std::iter;

#[derive(Debug, PartialEq)]
struct BooleanVector(Vec<bool>);

//        BooleanVector(lhs.iter().zip(rhs.iter()).map(|(x, y)| *x || *y).collect())

struct PMask {
    sys: u8,
    org: u8,
    group: u8,
    user: u8,
}

impl PMask {
    //new() returns with all values unitialized
    pub fn new() -> Self {
        PMask {
            sys: 0b00000000,
            org: 0b00000000,
            group: 0b00000000,
            user: 0b00000000,
        }
    }

    fn apply(&self) -> impl Iterator<Item = u8> {
        let s = iter::once(self.sys);
        let o = iter::once(self.org);
        let g = iter::once(self.group);
        let u = iter::once(self.user);
        s.chain(o).chain(g).chain(u)
    }

    //by convention `active` is iff sys, org, group, and user have
    //little-endian lead bit as 1
    pub fn is_active(&self) -> bool {
        let result =
            &self.sys << 7 & &self.org << 7 & &self.group << 7 & &self.user << 7 & &self.sys;

        println!("Is active: {:b}", result);

        result == 1_u8
    }

    //Set the minimum permissions for a Permissions object
    pub fn activate(&self) -> bool {
        let result =
            &self.sys << 7 & &self.org << 7 & &self.group << 7 & &self.user << 7 & &self.sys;

        println!("Is active: {:b}", result);

        result == 1_u8
    }
}

#[test]
fn floop() {
    let mask = PMask::new();

    for i in mask.apply() {
        println!("i: {:?}", i);
    }

    assert_eq!(mask.is_active(), false);
}
