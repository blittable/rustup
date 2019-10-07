# Lesson Fourteen: Iterator Patterns 

## Objectives 
- Understand Some of the main patterns used for iteration 


{{#playpen src/main.rs}}


enum Permissions {
    Sys(u8),
    Org(u8),
    Group(u8),
    User(u8),
}

impl Permissions {
    pub fn new() -> Self {
        PMask {
            sys: 0b00000000,
            org: 0b00000000,
            group: 0b00000000,
            user: 0b00000000,
        }
    }
}

    fn apply(&self) -> impl Iterator<Item = u8> {
        let s = iter::once(self.sys);
        let o = iter::once(self.org);
        let g = iter::once(self.group);
        let u = iter::once(self.user);
        s.chain(o).chain(g).chain(u)
    }
}



