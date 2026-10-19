// 1. What is a Hash Map & Basic Concept

// Other programming languages me Hash Maps ko Dictionary (Python), Object / Map (JavaScript), ya Associative Array (PHP) kaha jata hai.

//     Vectors vs Hash Maps: Vector me hum data index numbers (0, 1, 2...) se dhoondhte hain. Hash Map me hum data kisi bhi Key (jaise Name, ID, String) se dhoondhte hain.

//     Storage: Ye data ko Heap memory me store karta hai.

//     Homogeneous: Hash Map me saari Keys ka type same (jaise String) aur saari Values ka type same (jaise i32) hona chahiye.

// 2. Creating a New Hash Map & Inserting Data

// Hash Map Rust ke standard library ka hissa hai, lekin ye prelude me nahi hota. Isiliye ise use karne se pehle import (use std::collections::HashMap;) karna padta hai.

// use std::collections::HashMap;

// fn main() {
//     // HashMap banana
//     let mut scores = HashMap::new();

//     // Data insert karna (Key: String, Value: i32)
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     println!("{scores:?}"); // Output: {"Blue": 10, "Yellow": 50}
// }

// 3. Accessing Values in a Hash Map

// Hash Map se value nikalne ke liye .get() method ka use hota hai.

//     .get(&key) hamesha ek Option<&V> return karta hai (Some(&value) agar key mil gayi, ya None agar key nahi mili).

//     .copied() ka use karke hum Option<&i32> ko Option<i32> me convert karte hain.

//     .unwrap_or(0) ka matlab hai: Agar value mile toh wo de do, nahi toh default 0 maano.

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     let team_name = String::from("Blue");
//     // .get() me Key ka reference (&team_name) dete hain
//     let score = scores.get("&team_name").copied().unwrap_or(0);

//     println!("Blue team score: {score}");
// }

// 4. Iterating Over a Hash Map

// Aap for loop ka use karke saare Key-Value pairs par iterate (loop) kar sakte hain.

//     Note: Hash Map me elements kisi order me store nahi hote (Arbitrary Order), isliye output ka order fix nahi rehta.

// use std::collections::HashMap;

// fn main(){
//     let mut scores=HashMap::new();
//     scores.insert(String::from("Blue"),10);
//     scores.insert(String::from("Yellow"),50);

//     for (key, value) in &scores{
//         println!("{key}: {value}");
//     }
//     // Output (Order alag ho sakta hai):
//     // Yellow: 50
//     // Blue: 10
// }

// 5. Ownership in Hash Maps

// Rust ke Borrow Checker ke niyam Hash Maps par bhi laagu hote hain:

//     Copy Types (i32, bool, f64): Unki values Copy ho jati hain.

//     Owned Types (String): Unki ownership Move hokar Hash Map ke paas chali jaati hai. Insert karne ke baad aap un variables ko use nahi kar sakte.

use std::collections::HashMap;

// fn main() {
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);

//     // ❌ Error: field_name aur field_value aage use nahi ho sakte!
//     // println!("{field_name}"); // Ye compile time error dega.
// }

// fn main() {
//     let field_name = 32;
//     let field_value = 30;

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);

//     println!("{field_name}{field_value}");

//     // ye error nahi de ga ku k ye string k jaise iss ke ownership move nahi hoti hai
// }