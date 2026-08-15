/*
# 1. References Aur Borrowing Ka Concept

Pichhle section me `calculate_length` function me String bhejne par ownership move ho jaati thi. Ownership wapas lene ke liye hume tuple `(String, usize)` return karna padta tha.

Rust iska solution **References (`&`)** ke through deta hai. Reference ek pointer ki tarah hota hai jo memory address ko point karta hai, lekin data ki ownership nahi leta.

```rust
fn main() {
    let str = String::from("hello");
    let len = calculate_len(&str);
    println!("the length of {str} is {len}");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
```

### Memory Picture (3 Blocks)

| Block | Variable | Kahan hai | Andar kya hai | Matlab |
|-------|----------|-----------|---------------|--------|
| **Block 1** | `s` (Reference Pointer) | `calculate_length` function ke andar | Sirf ek field `ptr` (Pointer) | `s` ka ptr direct `s1` ko point kar raha hai. Heap me rakhe text ki ownership nahi, sirf reference (address pointer) |
| **Block 2** | `s1` (Actual Owner) | `main()` function ke andar | `ptr` (heap address), `len` (5), `capacity` (5) | Real ownership `s1` ke paas hi hai |
| **Block 3** | Heap Memory Data | Computer ki Heap Memory me | Index 0 se 4 tak: `h, e, l, l, o` | Actual text contents |

## 2. Immutable References (Listing 4-6)

By default, Rust me variables ki tarah references bhi **Immutable (Read-Only)** hote hain. Agar aap borrowed value ko modify karne ki koshish karenge, toh compile error aayega:

```rust
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    // ❌ COMPILE ERROR: Cannot mutate immutable reference
    some_string.push_str(", world");
}
```

## 3. Mutable References

Borrowed data ko modify karne ke liye **Mutable Reference (`&mut`)** ka use hota hai:

```rust
fn main() {
    let mut s = String::from("hello"); // 1. Variable ko 'mut' banaya

    change(&mut s); // 2. '&mut s' pass kiya
}

fn change(some_string: &mut String) { // 3. Parameter type '&mut String' rakha
    some_string.push_str(", world"); // ✅ Works fine!
}
```

## 4. Mutable References Ki Sabse Badi Restriction

Rust me ek waqt par kisi data ka sirf **EK hi mutable reference** ho sakta hai:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // ❌ COMPILE ERROR: cannot borrow `s` as mutable more than once at a time

println!("{r1}, {r2}");
```

### Data Race Se Bachao

Rust ye restriction isliye lagata hai taaki compile time par hi **Data Races** ko roka ja sake. Data Race tab hoti hai jab:

1. Do ya do se zyada pointers ek saath same data ko access kar rahe hon.
2. Unme se kam se kam ek pointer data me write (change) kar raha ho.
3. Access ko synchronize karne ka koi mechanism na ho.

### Curly Brackets `{ }` Se Naya Scope

Aap alag-alag scope me multiple mutable references bana sakte hain, bas wo ek saath active nahi hone chahiye:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 yahan scope se bahar ho gaya

let r2 = &mut s; // ✅ No problem!
```

## 5. Combining Mutable Aur Immutable References

Aap ek hi data ke liye Immutable (`&`) aur Mutable (`&mut`) references ko ek saath mix nahi kar sakte:

```rust
let mut s = String::from("hello");

let r1 = &s; // ✅ OK
let r2 = &s; // ✅ OK
let r3 = &mut s; // ❌ COMPILE ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable

println!("{r1}, {r2}, and {r3}");
```

### Reference Scope Overlapping (NLL - Non-Lexical Lifetimes)

Reference ka scope uske declaration se lekar uske aakhri usage (last time used) tak rehta hai:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1} and {r2}");
// r1 aur r2 ka last use yahan khatam ho gaya!

let r3 = &mut s; // ✅ No problem! Kyunki r1 aur r2 ka scope ab active nahi hai.
println!("{r3}");
```
*/

fn main() {}