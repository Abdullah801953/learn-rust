/*
# Ownership

Ownership Rust ka sabse unique aur sabse powerful feature hai. Isi ki wajah se Rust bina kisi Garbage Collector (GC) ke bhi 100% Memory Safe aur extremely fast hai!

Baaki languages mein memory management do tarike se hoti hai:

- **Garbage Collection (Java, Python, JS):** Program peeche se continuously check karta hai ki kaunsi memory free karni hai (jis se performance slow ho sakti hai).
- **Manual Management (C, C++):** Programmer ko khud malloc/free karna padta hai (dhyan na rakhne par memory leak ya crash bugs aate hain).

Rust ka Naya Tarika: **Ownership rules ka ek set!** Compile time par hi compiler in rules ko check kar leta hai. Agar rules follow hue toh program compile hoga, nahi toh compiler error de dega.

## 1. Pehle Samjhein: Stack vs Heap (Memory Model)

Ownership ko samajhne ke liye Stack aur Heap ka farak pata hona bohot zaroori hai.

| Feature | Stack | Heap |
|---------|-------|------|
| Data Structure | Last-In, First-Out (LIFO - jaise plates ka dher) | Unorganized dher (badi jagah) |
| Size | Fixed / compile-time size | Dynamic / variable size (badal sakta hai) |
| Speed | Very fast (pointers follow nahi karna padta) | Slightly slower (pointers search karke jana padta hai) |
| Example | `i32`, `bool`, `char` (fixed size integers) | `String` (jiski length user input se badal sakti hai) |

## 2. Ownership Ke 3 Golden Rules

Maan mein in 3 rules ko fit kar lijiye, poora Chapter 4 inhi par tikka hai:

1. **Rule 1:** Rust mein har value ka ek variable hota hai jise uska **Owner** kehte hain.
2. **Rule 2:** Ek waqt par sirf **EK HI Owner** ho sakta hai.
3. **Rule 3:** Jab Owner scope ke bahar jata hai, toh uski value memory se automatically **drop** (free) ho jati hai.

## 3. Scope Aur String Type

### Variable Scope Ka Matlab

Scope ka matlab hai code ka woh hissa jahan tak koi variable valid hai.

```rust
fn main() {
    // `s` abhi valid nahi hai
    {
        let s = "hello"; // `s` yahan se valid hai
        println!("{s}"); // `s` ko use kar sakte hain
    }
    // Yahan scope khatam! `s` ab valid NAHI hai
}
```

### String Literals (&str) vs String Type

| Type | Kya hai | Example |
|------|---------|---------|
| **String Literal (`&str`)** | Hardcoded text. Fast, fixed size (stack par rehta hai), lekin badla nahi ja sakta | `"hello"` |
| **String Type** | Dynamic text jo heap par store hota hai. Runtime par modify kiya ja sakta hai | `String::from("hello")` |

```rust
fn main() {
    // `String::from` heap par memory allocate karta hai
    let mut s = String::from("Hello");
    s.push_str(", World!"); // Append kar rahe hain
    println!("{s}"); // Output: Hello, World!
}
```

## 4. Memory Interaction: Move, Clone, aur Copy

### A. Move (Ownership Transfer)

**Primitive types (integers)** stack par hote hain, toh unki copy asaani se ban jati hai:

```rust
fn main() {
    let x = 5;
    let y = x; // `x` ki copy `y` me chali gayi. Dono valid hain!
    println!("{},{},", x, y);
}
```

**Heap types (String)** par ownership MOVE ho jati hai:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 👈 Ownership Move ho gayi!

    // println!("{s1}"); // ❌ COMPILE ERROR: value borrowed here after move
    println!("{s2}");    // ✅ Valid!
}
```

#### Step-by-Step Memory Analysis

**Line 1: `fn main() {`** — Yeh Rust program ka entry point hai. Main function start hote hi CPU par main ke liye ek Stack Frame allocate ho jata hai.

**Line 2: `let s1 = String::from("hello");`** — Kya hua?

- **Heap Allocation:** Heap memory mein `"hello"` text ke liye 5 bytes allocate hote hain.
- **Stack Allocation:** Stack par `s1` naam ka variable banta hai jisme 3 data store hote hain:
  - `ptr` (Pointer): Heap ka memory address (maan lijiye `0x1000`)
  - `len` (Length): Kitne characters hain (5)
  - `capacity` (Capacity): Total reserved space (5)

🎨 **Step 1 Memory Diagram:**

```text
       STACK MEMORY                         HEAP MEMORY
  +-----------------------+              +---+---+---+---+---+
  | Name  | Field | Value |              | Index | Value     |
  +-----------------------+              +---+---+---+---+---+
  | s1    | ptr   | 0x1000| -----------> | 0     |  'h'      |
  |       | len   | 5     |              | 1     |  'e'      |
  |       | cap   | 5     |              | 2     |  'l'      |
  +-----------------------+              | 3     |  'l'      |
                                         | 4     |  'o'      |
                                         +---+---+---+---+---+
```

Status: `s1` heap memory ka **Akela Owner** hai.

**Line 3: `let s2 = s1;`** — Kya hua?

- **Stack Metadata Copy:** Rust ne `s1` ka Stack metadata (ptr, len, capacity) `s2` ke andar copy kar diya.
- **Heap Data NOT Copied:** Heap par rakha text `"hello"` copy NAHI hua (bilkul waisa hi raha).
- **Move & Invalidation:** Rust ne `s1` ko Dead / Invalid mark kar diya.
- **Ownership Transferred:** Ab `"hello"` ka naya owner `s2` ban gaya.

🎨 **Step 2 Memory Diagram:**

```text
       STACK MEMORY                         HEAP MEMORY
  +-----------------------+              +---+---+---+---+---+
  | Name  | Field | Value |              | Index | Value     |
  +-----------------------+              +---+---+---+---+---+
  | s1    | [INVALIDATED] |              | 0     |  'h'      |
  |       | (Cannot use!) |              | 1     |  'e'      |
  +-----------------------+              | 2     |  'l'      |
  | s2    | ptr   | 0x1000| -----------> | 3     |  'l'      |
  |       | len   | 5     |              | 4     |  'o'      |
  |       | cap   | 5     |              +---+---+---+---+---+
  +-----------------------+
```

💡 **Rust ne `s1` ko Invalid kyun kiya?**

Agar `s1` aur `s2` dono valid hote, toh function khatam hone par Rust dono variables ke liye `0x1000` memory ko free karne ki koshish karta. Isse **Double Free Error** aata, jo crash karwa sakta tha. Isiliye Rust ne `s1` ko move hote hi invalid kar diya.

**Line 5: `// println!("{s1}");`** — Agar aap ise uncomment karte toh kya hota?

Compiler turant compile error deta:

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:22
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}");
  |              ^^^^ value borrowed here after move
```

Kyun? Kyunki Line 3 par `s1` ki ownership `s2` ko mil chuki hai, toh `s1` ab existing/valid variable nahi raha.

**Line 6: `println!("{s2}");`** — `s2` ke paas valid pointer (`0x1000`) hai. Rust heap par jaakar `"hello"` print karta hai.

Output:

```text
hello
```

**Line 7: `}` (Scope Ends)** — Scope khatam hone par kya hota hai?

- `main` function ka scope khatam hota hai.
- Rust `s2` ke liye automatically `drop` function call karta hai, jo Heap memory (`0x1000`) ko safai se free/deallocate kar deta hai.
- `s1` kyunki pehle se invalid tha, toh Rust uske liye kuch nahi karta (No double-free error!).

### B. Clone (Deep Copy)

Agar aapko Heap data ki **poori copy** chahiye (pointer nahi, poora data), toh `.clone()` method ka use karein:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 🟢 Heap ka data poora duplicate hua

    println!("s1 = {s1}, s2 = {s2}"); // ✅ Dono valid hain!
}
```

## 5. Functions and Ownership

```rust
fn main() {
    let s = String::from("hello");
    // Line 1: 's' variable create hua. "hello" string Heap memory par allocate hui.

    takes_ownership(s);
    // Line 2: 's' ko 'takes_ownership' function me pass kiya.
    // Heap data hone ki wajah se 's' ki ownership 'some_string' ko MOVE ho gayi.
    // Is line ke BAAD main() ke andar 's' INVALID ban chuka hai.

    let x = 5;
    // Line 3: 'x' variable bana. '5' ek integer hai jo Stack memory par hai.

    makes_copy(x);
    // Line 4: 'x' ko 'makes_copy' function me pass kiya.
    // Integer 'Copy' trait implement karta hai, isliye ownership Move NAHI hui,
    // balki '5' ki ek duplicate COPY function ko mili.
    // println!("{s}"); ye error karega kyunki 's' ki ownership move ho gayi hai
    // takes_ownership(s) function ko
}

fn takes_ownership(some_string: String) {
    // Line 6: 'some_string' parameter ne 's' ki ownership receive ki.
    println!("{some_string}");
    // Line 7: "hello" print hoga.
}
// Line 8: takes_ownership function ka scope khatam!
// Rust yahan automatically 'drop' function call karega aur Heap memory me se
// "hello" ko FREE (clean) kar dega.

fn makes_copy(some_integer: i32) {
    // Line 9: 'some_integer' ko 'x' ki copy mili.
    println!("{some_integer}");
    // Line 10: "5" print hoga.
}
// Line 11: makes_copy scope end. Memory me koi special heap cleanup nahi hota
// kyunki ye stack standard data tha.
```

## 6. Return Values and Scope

```rust
fn main() {
    let s1 = gives_ownership();
    // Line 1: 'gives_ownership' call hua. Function ne jo String return ki,
    // uski ownership 's1' ko mil gayi.

    let s2 = String::from("hello");
    // Line 2: 's2' bana aur Heap me "hello" store hua.

    let s3 = takes_and_gives_back(s2);
    // Line 3: 's2' ki ownership 'takes_and_gives_back' function ko move hui.
    // Fir function ne wapas value return ki, toh ownership 's3' ko mil gayi.
    // Ab 's2' INVALID ho gaya, aur 's3' VALID ho gaya.
}
// Line 4: main() scope exit.
// 1. 's3' scope se bahar -> Memory drop hui.
// 2. 's2' already moved tha -> Kuch nahi hua.
// 3. 's1' scope se bahar -> Memory drop hui.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // Line 5: 'some_string' bana.

    some_string
    // Line 6: Without semicolon (expression).
    // 'some_string' return hua aur ownership bahar caller (s1) ko MOVE ho gayi.
}

fn takes_and_gives_back(a_string: String) -> String {
    // Line 7: 'a_string' ne parameter ke taur par ownership receive ki.

    a_string
    // Line 8: 'a_string' return hua aur ownership wapas bahar caller (s3) ko MOVE ho gayi.
}
```

## 7. Tuples ke saath Ownership Return karna

```rust
fn main() {
    let s1 = String::from("hello");
    // Line 1: 's1' create hua.

    let (s2, len) = calculate_length(s1);
    // Line 2: 's1' ko pass kiya. Function ne Tuple (String, usize) return kiya.
    // Destructuring karke: 's2' ko String ki ownership wapas mili aur 'len' ko length (5) mila.

    println!("The length of '{s2}' is {len}.");
    // Line 3: Output: "The length of 'hello' is 5."
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // Line 4: String ki length nikali (5).
    (s, length) // Line 5: Tuple return kiya jisme String 's' (ownership) aur 'length' dono hain.
}
```

### Step-by-Step Breakdown

**1. `main()` Function Shuru Hua**

```rust
let s1 = String::from("hello");
```

Memory Status: `s1` ke paas `"hello"` String ki ownership hai.

**2. `calculate_length(s1)` Call Hua**

```rust
let (s2, len) = calculate_length(s1);
```

- `s1` function ke andar gaya. Iska matlab `"hello"` ki ownership `s1` se chheen kar `calculate_length` function ke parameter `s` ko de di gayi.
- **Important:** Ab `main()` ke paas `s1` VALID NAHI raha.

**3. `calculate_length` Inside Code**

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // length = 5 ho gaya
    (s, length) // 📦 PACKET (Tuple) banakar return kiya
}
```

- Function ka return type `-> (String, usize)` bolta hai: *"Main ek packet return karunga jisme pehli value String hogi aur doosri value usize number hogi."*
- `(s, length)` line me:
  - `s` ki ownership ko packet me pack kiya.
  - `length` (5) ko packet me pack kiya.
  - Ye poora packet return kar diya.

**4. Wapas `main()` Me Result Receive Hua (Destructuring)**

```rust
let (s2, len) = calculate_length(s1);
```

Ye line packet ko unpacking (kholne) ka kaam karti hai:

- Packet me se pehli value (`s`) nikal kar `s2` me chali gayi. (Toh `"hello"` ki ownership ab `s2` ko mil gayi).
- Packet me se doosri value (`length`) nikal kar `len` me chali gayi. (Toh `len` ko 5 mil gaya).
*/

fn main() {}