### Proofing
- [x] Proof  README 
- [x] Proof OVERVIEW 
- [x] Proof One 
- [x] Proof Two 
- [x] Proof Three - Sparse
- [x] Proof Four
- [ ] Proof Five 
- [ ] Review Lesson 5 
- [ ] Review Lesson 6 
- [ ] Review Lesson 7 
- [x ] Review Lesson 8 - Needs detailed review and assignment config 
- [ ] Review Lesson 9 
- [ ] Review Lesson 10
- [ ] Review Lesson 11 
- [ ] Review Lesson 12 
- [ ] Review Lesson 13 
- [ ] Review Lesson 14 
- [ ] Review Lesson 15 
- [x] Review Lesson 16 - Need detailed review and material on Drop, better explanation of Clone, Copy 
- [ ] Review Lesson 17 
- [ ] Review Lesson 18 
- [ ] Review Lesson 19 
- [ ] Demo code triage/structure
- [ ] Add more constructor patterns 
- [ ] Add more iterator patterns 

### Lessons

- Add Note on closures as inline functions
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

