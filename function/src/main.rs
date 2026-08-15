/*
# 1. Function Naming & Basic Declaration

Function ko call karna aur define karna:

```rust
fn main() {
    println!("Hello, world!");
    new_function();
}

fn new_function() {
    println!("this is new function");
}
```

**Error:** Naming convention snake_case nahi hone par warning aati hai:

```text
warning: function `newFunction` should have a snake case name
 --> src\main.rs:6:4
  |
6 | fn newFunction(){
  |    ^^^^^^^^^^^ help: convert the identifier to snake case: `new_function`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: `function` (bin "function") generated 1 warning
```

**Note:** Rust mein function aur variable names ke liye snake_case convention follow kiya jata hai (saare letters lowercase aur words ke beech mein underscore `_`).

**Note:** Rust ko isse koi farak nahi padta ki aapne sub-function ko `main()` ke upar define kiya hai ya neeche. Bas woh us scope mein defined hona chahiye.

# 2. Function Parameters

Rust ka ek strict rule hai: Aapko har parameter ka **Data Type explicitly batana hi padega**. Compiler khud guess (infer) nahi karega.

```rust
fn main() {
    print_labeled_measurement(5, 'h'); // function call
}

fn print_labeled_measurement(value: u8, unit_lable: char) {
    println!("the mesurement is: {value}{unit_lable}");
}
```

# 3. Statements vs. Expressions

Rust ek **expression-based language** hai. Functions ki body Statements aur Expressions se milkar banti hai:

| Type | Kya karta hai | Example |
|------|---------------|---------|
| **Statement** | Instruction jo action perform karti hai, koi value return nahi karti | `let y = 6;` |
| **Expression** | Evaluate hokar ek final value return karti hai | `5 + 6` returns `11` |

Scope block `{}` bhi ek expression hota hai!

```rust
fn main() {
    let y = {
        let x = 2;
        x + 1 // 👈 Notice: end me Semicolon (;) NAHI hai!
    };
    println!("the value of y is: {y}");
}
```

**Semicolon `;` Ka Khel:**

| Code | Type | Result |
|------|------|--------|
| `x + 1` | Expression | Value return karega |
| `x + 1;` | Statement | Unit type `()` return karta hai |

# 4. Functions with Return Values

Jab koi function value return karta hai, toh function signature mein `->` arrow ke saath uska Return Type likhte hain.

**2 Ways to Return:**

1. **Implicit Return (Recommended):** Function ke aakhir mein bina semicolon `;` ke expression likhein.
2. **Explicit Return:** `return` keyword aur `;` ka use karein (mostly early exit ke liye).

**Example 1: Implicit Return**

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("the value of x is: {x}");
}
```

**Example 2: Calculation with Implicit Return**

```rust
fn plus_one(x: u8) -> u8 {
    x + 1
}

fn main() {
    let result = plus_one(2);
    println!("add 1 in 2 is {result}");
}
```
*/

fn main() {}