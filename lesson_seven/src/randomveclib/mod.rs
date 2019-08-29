extern crate rand;

pub mod ramdomvec {
    pub fn generatevec() -> (Vec<u16>) {
        use rand::Rng;
        let mut stack = Vec::new();
        let mut rng = rand::thread_rng();
        let deep = rng.gen_range(0, 10);

        for _x in 0..deep {
            stack.push(rng.gen_range(0, 10));
        }
        return stack;
    }
}

