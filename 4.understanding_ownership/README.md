# Table of Contents
- [Ownership Rules](#ownership-rules)
- [Ownership and Functions](#ownership-and-functions)
- [References and Borrowing](#references-and-borrowing)
- [Mutable References](#mutable-references)
- [Dangling References](#dangling-references)
- [The Rules of References](#the-rule-of-references)

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

# References and Borrowing
***A reference is like a pointer in that it's an addres we can fellow to acces the data stored at that addres***.  
that data is owned by other variable. Unlike pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust

fn main() {
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);

    println!("The length of '{}' is {}. ", s1, len);
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}
```
& -> represents *references*, and they allow us to refer to some value without taking ownership of it.  
So, s references s1, and s1 point to a value into the heap. s -> s1 -> heap(hello)

# Mutable References
If we want to change a variable through its reference, the variable must be mutable as well.  

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Mutable references have one big restriction: if you have a mutable reference to a value,
you can have no other references to that value**

# Dangling References
***if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.***   

The code above wont compile, and will recieve the fellow error:
***[...]this function's return type contains a borrowed value, but there is no value for it to be borrowed from...***  
This occour, becouse s goes out of scope at the and of the dangle function.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // returns a reference to a String
    let s = String::from("String"); // s is a new string

    &s // we retur a reference to the String, s
} // s goes out of scope, and is dropped, its memory goes away. DANGER!!!!!!!!
```

to fix it, we would return the String directly

```rust
fn no_dangle() -> String {
    let s = String::from("String");
    s
}

```
# The Rules of References
- At any given time, we can have either one mutable reference or any number of immutable references.  
- References must always be valid.  
