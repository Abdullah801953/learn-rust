// Key Takeaway: Vectors vs JS Arrays

// JS mein Array [1, "hello", true] me aap kuch bhi daal sakte hain kyunki wo dynamic hoti hai. Lekin Rust me:

//     Heap Memory: Vectors ka data Heap par store hota hai, isiliye iska size runtime par grow ya shrink ho sakta hai.

//     Same Type Only: Ek Vector me sirf ek hi type ka data ho sakta hai (e.g., saare i32 ya saare String).

// 1. Vector Kaise Banayein? (Creation)

// Vector banane ke do tareeqe hain:

// fn main() {
//     // Tareeqa 1: Khali Vector (Explicit Type Dena Padega)
//     // JS equivalent: const v = [];
//     let v1: Vec<i32> = Vec::new();

//     // Tareeqa 2: Initial values ke sath (`vec!` macro)
//     // Rust khud hi type guess kar leta hai (i32)
//     // JS equivalent: const v = [1, 2, 3];
//     let v2 = vec![1, 2, 3];

//     println!("v1: {:?}, v2: {:?}", v1, v2);
// }

// 2. Vector Mein Data Add Karna (Updating)

// Vector me data push karne ke liye use mut (mutable) banana zaroori hai.

// fn main(){
//     let mut numbers=Vec::new();

//     numbers.push(5);
//     numbers.push(6);
//     numbers.push(7);
//     numbers.push(8);

//     println!("updated vector:{:?}",numbers);
// }

// 3. Vector Se Data Read Karna (Indexing vs .get())
// Rust me element access karne ke 2 tarike hain. Dono me bohot bada farak hai:

// Method	Syntax	Agar Index Out of Bounds ho toh?	kab Use karein?
// Indexing	&v[index]	Crash (Panic)! Program stop ho jayega.	Jab aap 100% sure hon ki index exist karta hai.
// Get Method	v.get(index)	Safe (None return karega).	Jab index user input ya dynamic ho.

// fn main() {
//     let v = vec![10, 20, 30, 40, 50];

//     // Method 1: Direct Indexing (Crash ho sakta hai)
//     let third: &i32 = &v[2];
//     println!("Third element is: {}", third);

//     // Method 2: Safe Reading with .get()
//     let index_to_find = 100; // Yeh index exist nahi karta!

//     match v.get(index_to_find) {
//         Some(value) => println!("Value found: {}", value),
//         None => println!("❌ Index {} exist nahi karta! Safe handled.", index_to_find),
//     }
// }

// 4. Borrow Checker & Memory reallocation Rule ⚠️

// Yeh sabse zaroori rule hai! Agar aapne Vector ke kisi element ka Immutable reference (&) liya hai, toh aap usi scope mein Vector ko Modify (push/pop) nahi kar sakte.

