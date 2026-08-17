// fn main() {
//     let mut str = String::from("hello world");
//     let word = first_word(&str);
//     str.clear();
//     println!("in {str} space is present at {word}");
// }
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &items) in bytes.iter().enumerate() {
//         if items == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// 1. Slices Bina Code Ki Problem (Raw Index Bug)

// Pehle text mein ek problem statement di gayi hai: String me se pehla word dhoondna.

// Agar hum slices ka use kiye bina function likhte hain:
// Rust

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

//     Problem: Yeh function sirf ek number (usize) return karta hai (jaise 5).

//     Bug Scenario:
//     Rust

//     let mut s = String::from("hello world");
//     let word = first_word(&s); // word = 5
//     s.clear(); // s ab empty ("") ho gayi!
//     // word me abhi bhi 5 pada hai, par s khali hai -> Data out-of-sync ho gaya!

//     Raw index (usize) return karne par String aur returned index ke beech koi link nahi rehta, jisse runtime bugs aate hain.

// 2. String Slices (&str) Solution & Internal Layout

// Slice ek collection ke continuous part ka reference hota hai jiske paas ownership nahi hoti.

//     Syntax: &s[start..end] (jahan start inclusive hai aur end exclusive).

//     Internal Structure: Memory mein slice ke paas 2 items hote hain:

//        1. Pointer: Sub-sequence ke pehle byte ka address.

//        2. Length: Sub-sequence ki kitni length hai (end - start).

fn main() {
    let mut s = String::from("hello world");

    // 1. Direct slice banana
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {hello}, world: {world}");

    // 2. Function se slice Lena
    let word = first_word(&s);
    println!("First word: {word}");
    
    s.clear(); // ❌ Agar ise uncomment karenge toh COMPILE ERROR aayega
}

// Return type `usize` se badal kar `&str` kar diya
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[0..i]; // Space tak ka slice return kiya
        }
    }

    &s[..] // Agar space na mile toh poori string ka slice
}