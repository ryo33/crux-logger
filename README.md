# crux_logger
Logger middleware

## Usage

Implement the `Log` trait.  
```rust
extern crate crux_logger;
use crux_logger::{Log, Logger};

impl Log for YourState {
    fn log(&self) -> String {
        format!("{:?}", self)
    }
}
```

Create the logger middleware and add it to your store.  
```rust
let logger: Logger<YourState> = Logger::new();
your_store.add_middleware(logger);
```

Run with `RUST_LOG=crux_logger=debug`.  
```bash
$ RUST_LOG=crux_logger=debug cargo run
```
