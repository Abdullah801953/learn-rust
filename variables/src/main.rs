/*
# 1. Mutability

Immutable variable ko `mut` keyword se mutable banaya ja sakta hai:

```rust
fn main() {
    let mut x = 4;
    println!("{x}");
    x = 5;
    println!("{x}");
}
```

# 2. Constants

`const` se constant banate hain — value kabhi change nahi hoti, aur type batana zaroori hai:

```rust
fn main() {
    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
    {
        println!("{THREE_HOUR_IN_SECONDS}");
    }

    println!("{THREE_HOUR_IN_SECONDS}");
}
```

# 3. Shadowing

Same naam se naya variable banaya jata hai — purani value ko "shadow" kar deta hai:

```rust
fn main() {
    let x = 2;
    let x = x + 3;
    {
        let x = x + 1;
        println!("block scope {x}");
    }
    println!("outer scope {x}");
}
```

# 4. const vs let (Global Scope)

- `const` ko main function ke bahar (global scope) use kar sakte hain
- `let` ko global scope me use nahi kar sakte:

```rust
const X: u32 = 5;

fn main() {
    println!("{X}");
}
```

`let` ko global use karne par yeh error aata hai:

```text
error: expected item, found keyword `let`
  --> src\main.rs:36:1
   |
36 | let x = 5;
   | ^^^
   | |
   | `let` cannot be used for global variables
   | help: consider using `static` or `const` instead of `let`
   |
   = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>

error: could not compile `variables` (bin "variables") due to 1 previous error
```
*/

fn main() {}