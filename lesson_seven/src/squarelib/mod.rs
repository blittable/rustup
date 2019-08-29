extern crate rand;

pub mod square {
    pub fn generatedimention() -> (u16, u16, u16) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let (h, w, d) = (rng.gen_range(0, 10), rng.gen_range(0, 10), rng.gen_range(0, 10));
        return (h, w, d);
    }
}

