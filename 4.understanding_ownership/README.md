# Ownership Rules
- Each value in Rust has an owner;
- There can only be one owner at a time;
- When the owner goes out of scope, the value will be dropped.

# Ownership and Functions
- Data types that do implement Copy(integers, floats...) trait are copied
- Data types that do not implement Copy trait (String...), are moved(ownership)

```rust

fn main() {
    let s: String = String::from("Hello, World"); // s comes into scope
    
    takes_ownership(s); // s's value moves into the function...
                        // ... s is no longer valid

    let x: i32 = 5; // x comes into scope
                    
    makes_copy(x); // x would move into the function,
                    // but i32 is Copy, so it's okay to still use x afterward
} // Here, x goes out of scope, then s. But becouse s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    // ... do something
} // some_string goes out of scope and `drop` is called. memory is freed

fn makes_copy(some_integer: i32) { // some_integer coes into scope
    // ... do something
}// some_integer goes out of scope. Nothing special happens
```

