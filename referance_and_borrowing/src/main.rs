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



// Line 1: let mut s = String::from("hello");

//     Kya hua?

//         s naam ka mutable variable Stack par bana.

//         Heap memory par "hello" text allocate hua.

// 🎨 Memory Diagram (Step 1):
// Plaintext

//        STACK MEMORY                         HEAP MEMORY
//   +-----------------------+              +---+---+---+---+---+
//   | Name  | Field | Value |              | Index | Value     |
//   +-----------------------+              +---+---+---+---+---+
//   | s     | ptr   |0x1000 | -----------> | 0     |  'h'      |
//   | (mut) | len   | 5     |              | 1     |  'e'      |
//   |       | cap   | 5     |              | 2     |  'l'      |
//   +-----------------------+              | 3     |  'l'      |
//                                          | 4     |  'o'      |
//                                          +---+---+---+---+---+

// Line 2 & 3: let r1 = &s; let r2 = &s;

//     Kya hua?

//         r1 aur r2 dono Immutable References (&s) bane.

//         Yeh dono direct Heap ko point nahi karte, balki Stack par rakhe s ke address ko point karte hain.

//         Rust ek saath unlimited immutable references allow karta hai (kyunki read-only operations se koi hazard nahi hota).

// 🎨 Memory Diagram (Step 2):
// Plaintext

//        STACK MEMORY                         HEAP MEMORY
//   +-----------------------+              +---+---+---+---+---+
//   | Name  | Value         |              | Index | Value     |
//   +-----------------------+              +---+---+---+---+---+
//   | s     | ptr: 0x1000   | -----------> | 0     |  'h'      |
//   |       | len: 5, cap: 5|              | 1     |  'e'      |
//   +-----------------------+              | 2     |  'l'      |
//   | r1    | points to `s` | ------\      | 3     |  'l'      |
//   +-----------------------+        \     | 4     |  'o'      |
//   | r2    | points to `s` | --------> [s variable] +---+---+---+
//   +-----------------------+        /

//     Active Borrows: r1 (Immutable), r2 (Immutable).

// Line 4: println!("{r1} and {r2}");

//     Kya hua?

//         r1 aur r2 ki values read hui aur terminal par hello and hello print hua.

//     💡 Sabse Main Point (Non-Lexical Lifetimes):

//         Is line par r1 aur r2 ka aakhiri (last) use hua hai.

//         Line 4 ke khatam hote hi Rust compiler samajh jata hai ki code mein aage r1 ya r2 ki zaroorat nahi hai.

//         Isiliye r1 aur r2 ka borrowing scope Line 4 par hi KHATAM (expire) ho jata hai.

// Plaintext

//   [Line 4 execution finished] ---> `r1` aur `r2` ke references ab ACTIVE nahi hain!

// Line 5: let r3 = &mut s;

//     Kya hua?

//         s ka ek Mutable Reference (&mut s) bana.

//     Kyun Compile Error NAHI Aaya?

//         Rust ka Rule: "Aap Immutable aur Mutable reference ek saath nahi rakh sakte."

//         Lekin kyunki Line 4 ke baad r1 aur r2 ka scope khatam ho chuka tha, is waqt memory mein koi bhi doosra reference active nahi tha.

//         Isiliye r3 ko exclusive write access (Mutable Borrow) asaani se mil gaya!

// 🎨 Memory Diagram (Step 3):
// Plaintext

//        STACK MEMORY                         HEAP MEMORY
//   +-----------------------+              +---+---+---+---+---+
//   | Name  | Value         |              | Index | Value     |
//   +-----------------------+              +---+---+---+---+---+
//   | s     | ptr: 0x1000   | -----------> | 0     |  'h'      |
//   +-----------------------+              | 1     |  'e'      |
//   | r1    | [EXPIRED]     |              | 2     |  'l'      |
//   | r2    | [EXPIRED]     |              | 3     |  'l'      |
//   +-----------------------+              | 4     |  'o'      |
//   | r3    | points to `s` | -----------> [s variable] +---+---+---+
//   |(&mut) | (Exclusive!)  |
//   +-----------------------+

// Line 6: println!("{r3}");

//     Kya hua?

//         r3 reference ko read karke hello print kiya gaya.

//         Output: hello

// Summary: Purana Rust vs Naya Rust (NLL)

//     Old Rust (2018 se pehle): Scope purely {} blocks par chalta tha. Line 6 tak r1 aur r2 active maane jaate, aur Line 5 par Compile Error aata.

//     Modern Rust (NLL): Compiler dekhta hai ki variable aakhiri baar kahan use hua hai. Last use ke baad borrow release ho jata hai, isiliye Line 5 par Naya &mut reference allow ho gaya.


fn main() {}