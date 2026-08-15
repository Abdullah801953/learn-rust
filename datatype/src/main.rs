/*
# Scalar Types (Scaler Types)

| Type | Size | Range / Example |
|------|------|-----------------|
| `u8` (unsigned) | 8-bit | `25 * 3` |
| `i8` (signed) | 8-bit | `-128` |
| `f32` (float) | 32-bit | `2.3` |
| `f64` (float) | 64-bit | `45.6` |
| `bool` | - | `true` |
| `char` | 4 bytes | `'h'` |

```rust
let x: u8 = 25 * 3;
let x: i8 = -128;
let x: f32 = 2.3;
let x: f64 = 45.6;
let x: bool = true;
let x: char = 'h';
```

# Compound Types

## 1. Tuple

- Fixed length, different types ho sakte hain
- **Destructuring:** `let (x, y, z) = tup;`
- **Index access:** `tup.0`, `tup.1`, `tup.2`

```rust
let tup: (i32, u32, f64) = (23, 45, 4.5);

// Destructuring (tod kar alag variables me daalna)
let (x, y, z) = tup;
println!("{x}");
println!("{y}");
println!("{z}");

// Index se access
let twenty_three = tup.0;
let fourty_five = tup.1;
let four_point_five = tup.2;
println!("{},{},{}", twenty_three, fourty_five, four_point_five);
```

## 2. Array

- Fixed length, same type ke elements
- Index `0` se shuru hota hai

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
println!("{},{}", first, second);
```

**Same value se initialize:** `[value; length]` — 5 elements, sabki value 0 (`[0, 0, 0, 0, 0]`):

```rust
let c = [0; 5];
let first = c[0];
let second = c[1];
println!("{},{}", first, second);
```
*/

fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{},{}", first, second);

    let c = [0; 5];
    let first = c[0];
    let second = c[1];
    println!("{},{}", first, second);
}