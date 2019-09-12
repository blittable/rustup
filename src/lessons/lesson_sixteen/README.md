
# Lesson Sixteen: Shared Data 

## Objectives 

- Look at Rc, and RefCell in the context of a problem

| Option                            | Details       | Issue             | 
| --------------------------------- | ------------- | ----------------- |
| Clone                             | Makes a Copy  | Lose shared state |
| Borrow with Lifetime Annotations  | &'a structs   |                   |
| Box<T>                            | Content Cell  | Even Better       |
| Rc<T>                             | Even Better   |                   |
 

let boxed: Box<u8> = Box::new(val);