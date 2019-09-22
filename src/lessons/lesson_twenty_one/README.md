
# Lesson Twenty-One: Async 

## Objectives 
- Understand the fundamental of the upcoming ``async``  


## Understanding Runtimes

A runtime is an abstraction for a loop for an extended process.  Rust supports pluggable runtimes with the runtime attribute.  

Note this line: `#[runtime::main]`

```rust
/// Use the default Native Runtime
#[runtime::main]
async fn main() {}

/// Use the Tokio Runtime
#[runtime::main(runtime_tokio::Tokio)]
async fn main() {}
```

## Async Essentials 

- It's not done! (Sept. 2019) - it's in nightly, but not in `stable`
- Methods must be marked by `async` keyword
- Almost always for single-threaded scenarios 
- Nothing happens if `asnyc` functions are not awaited
- await() methods return Futures


## Futures

The Tokio documentation has an excellent explanation of futures:

Trait tokio::prelude::Future

A future represents an asynchronous computation.

A future is a value that may not have finished computing yet. This kind of "asynchronous value" makes it possible for a thread to continue doing useful work while it waits for the value to become available.
The poll method

The core method of future, poll, attempts to resolve the future into a final value. This method does not block if the value is not ready. Instead, the current task is scheduled to be woken up when it's possible to make further progress by polling again. The context passed to the poll method can provide a Waker, which is a handle for waking up the current task.

When using a future, you generally won't call poll directly, but instead .await the value.


## A sample using the Tokio runtime crate

```rust, no_run
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```