// Ownership Rust ka sabse unique aur sabse powerful feature hai. Isi ki wajah se Rust bina kisi Garbage Collector (GC) ke bhi 100% Memory Safe aur extremely fast hai!

// Baaki languages mein memory management do tarike se hoti hai:

//     Garbage Collection (Java, Python, JS): Program peeche se continuously check karta hai ki kaunsi memory free karni hai (jis se performance slow ho sakti hai).

//     Manual Management (C, C++): Programmer ko khud malloc/free karna padta hai (dhan na rakhne par memory leak ya crash bugs aate hain).

// Rust ka Naya Tarika: Ownership rules ka ek set! Compile time par hi compiler in rules ko check kar leta hai. Agar rules follow hue toh program compile hoga, nahi toh compiler error de dega.

// 1. Pehle Samjhein: Stack vs Heap (Memory Model)

// Ownership ko samajhne ke liye Stack aur Heap ka farak pata hona bohot zaroori hai.
// Feature	Stack 	Heap
// Data Structure	Last-In, First-Out (LIFO - Jaise plates ka dher)	Unorganized dher (Badi jagah)
// Size	Fixed / Fixed compile-time size	Dynamic / Variable size (Size badal sakta hai)
// Speed	Very Fast (Pointers ko follow nahi karna padta)	Slightly Slower (Pointers search karke jana padta hai)
// Example	i32, bool, char (Fixed size integers)	String (Jiski length user input se badal sakti hai)

// 2. Ownership Ke 3 Sone Swarnim Niyam (3 Golden Rules)

// Maan mein in 3 rules ko fit kar lijiye, poora Chapter 4 inhi par tikka hai:

//     Rule 1: Rust mein har value ka ek variable hota hai jise uska Owner kehte hain.

//     Rule 2: Ek waqt par sirf EK HI Owner ho sakta hai.

//     Rule 3: Jab Owner Scope ke bahar jata hai, toh uski value memory se automatically drop (free) ho jati hai.

// 3. Scope Aur String Type
// Variable Scope Ka Matlab

// Scope ka matlab hai code ka woh hiss jahan tak koi variable valid hai.
// fn main() {
//     // `s` abhi valid nahi hai
//     {
//         let s = "hello"; // `s` yahan se valid hai
//         println!("{s}"); // `s` ko use kar sakte hain
//     }
//     // Yahan scope khatam! `s` ab valid NAHI hai
// }

// String Literals (&str) vs String Type

//     String Literal (&str): Hardcoded text (e.g., "hello"). Fast hota hai kyunki fixed size ka hota hai (Stack par rehta hai), lekin badla nahi ja sakta.

//     String Type: Dynamic text jo Heap par store hota hai. Ise runtime par modify kiya ja sakta hai:
// fn main() {
//     // `String::from` heap par memory allocate karta hai
//     let mut s = String::from("Hello");
//     s.push_str(", World!"); // Append kar rahe hain
//     println!("{s}"); // Output: Hello, World!
// }

// 4. Memory Interaction: Move, Clone, aur Copy
// A. Move (Ownership Transfer)

// Primitive types (integers) stack par hote hain, toh unki copy asaani se ban jati hai:
// fn main() {
//     let x = 5;
//     let y = x; // `x` ki copy `y` me chali gayi. Dono valid hain!
//     println!("{},{}",x,y);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // 👈 Ownership Move ho gayi!

//     // println!("{s1}"); // ❌ COMPILE ERROR: value borrowed here after move
//     println!("{s2}");    // ✅ Valid!
// }

// Line 1: fn main() {

// Yeh Rust program ka entry point hai. Jab main function start hota hai, toh CPU par main ke liye ek Stack Frame allocate ho jata hai.
// Line 2: let s1 = String::from("hello");
// Kya hua?

//     Heap Allocation: Heap memory mein "hello" text ke liye 5 bytes allocate hote hain.

//     Stack Allocation: Stack par s1 naam ka variable banta hai jisme 3 data store hote hain:

//         ptr (Pointer): Heap ka memory address (Maan lijiye 0x1000).

//         len (Length): Kitne characters hain (5).

//         capacity (Capacity): Total reserved space (5).

// 🎨 Step 1 Memory Diagram:
// Plaintext

//        STACK MEMORY                         HEAP MEMORY
//   +-----------------------+              +---+---+---+---+---+
//   | Name  | Field | Value |              | Index | Value     |
//   +-----------------------+              +---+---+---+---+---+
//   | s1    | ptr   |0x1000 | -----------> | 0     |  'h'      |
//   |       | len   | 5     |              | 1     |  'e'      |
//   |       | cap   | 5     |              | 2     |  'l'      |
//   +-----------------------+              | 3     |  'l'      |
//                                          | 4     |  'o'      |
//                                          +---+---+---+---+---+

//     Status: s1 heap memory ka Akela Owner hai.

// Line 3: let s2 = s1;
// Kya hua?

//     Stack Metadata Copy: Rust ne s1 ka Stack metadata (ptr, len, capacity) s2 ke andar copy kar diya.

//     Heap Data NOT Copied: Heap par rakha text "hello" copy NAHI hua (Heap data bilkul waisa hi raha).

//     Move & Invalidation: Rust ne s1 ko Dead / Invalid mark kar diya.

//     Ownership Transferred: Ab "hello" ka naya owner s2 ban gaya.

// 🎨 Step 2 Memory Diagram:
// Plaintext

//        STACK MEMORY                         HEAP MEMORY
//   +-----------------------+              +---+---+---+---+---+
//   | Name  | Field | Value |              | Index | Value     |
//   +-----------------------+              +---+---+---+---+---+
//   | s1    | [INVALIDATED] |              | 0     |  'h'      |
//   |       | (Cannot use!) |              | 1     |  'e'      |
//   +-----------------------+              | 2     |  'l'      |
//   | s2    | ptr   |0x1000 | -----------> | 3     |  'l'      |
//   |       | len   | 5     |              | 4     |  'o'      |
//   |       | cap   | 5     |              +---+---+---+---+---+
//   +-----------------------+

// 💡 Rust ne s1 ko Invalid kyun kiya?

// Agar s1 aur s2 dono valid hote, toh function khatam hone par Rust dono variables ke liye 0x1000 memory ko free karne ki koshish karta. Isse Double Free Error aata, jo crash karwa sakta tha. Isiliye Rust ne s1 ko move hote hi invalid kar diya.
// Line 5: // println!("{s1}"); (Commented Line)
// Agar aap ise uncomment karte toh kya hota?

// Compiler turant compile error deta:
// Plaintext

// error[E0382]: borrow of moved value: `s1`
//  --> src/main.rs:5:22
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 | 
// 5 |     println!("{s1}");
//   |              ^^^^ value borrowed here after move

// Kyun? Kyunki Line 3 par s1 ki ownership s2 ko mil chuki hai, toh s1 ab exisiting/valid variable nahi raha.
// Line 6: println!("{s2}");
// Kya hua?

// s2 ke paas valid pointer (0x1000) hai. Rust Heap par jaakar "hello" print karta hai.

// Output:
// Plaintext

// hello

// Line 7: } (Scope Ends)
// Scope Khatam Hone Par Kya Hota Hai?

//     main function ka scope khatam hota hai.

//     Rust s2 ke liye automatically drop function call karta hai, jo Heap memory (0x1000) ko safai se free/deallocate kar deta hai.

//     s1 kyunki pehle se invalid tha, toh Rust uske liye kuch nahi karta (No double-free error!).

// B. Clone (Deep Copy)

// Agar aapko Heap data ki poori copy chahiye (pointer nahi, poora data), toh .clone() method ka use karein:
// Rust

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // 🟢 Heap ka data poora duplicate hua

//     println!("s1 = {s1}, s2 = {s2}"); // ✅ Dono valid hain!
// }

