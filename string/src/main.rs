// Rust mein Strings seekhna thoda challenging lag sakta hai kyunki baaki languages (jaise JavaScript ya Python) strings ke andar ki complexities ko chhupa deti hain, jabki Rust aapko unhe directly handle karne par majboor karta hai.Rust mein String basically Bytes ka ek Vector (Vec<u8>) hota hai, jo strictly UTF-8 Encoded hota hai.1. String vs &str Mein Kya Farak Hai?Rust mein do sabse main string types hoti hain:Property&str (String Slice)StringOwnershipBorrowed (kisi aur jagah stored data ka reference)Owned (heap par store hota hai)SizeFixed / ImmutableGrowable / MutableMemoryStack, Heap, ya Binary embeddedExclusively HeapJS AnalogyRead-only referenceStringBuilder ya Dynamic String

// fn main() {
//     // 1. String Literal (&str): Program ke binary mein hardcoded hota hai
//     let literal: &str = "Hello World";

//     // 2. Owned String: Heap par allocate hoti hai, dynamic hai
//     let owned: String = String::from("Hello World");
// }

// 2. Nayi String Kaise Banayein? (Creation)

// String banane ke 3 aam tarike hain:

// fn main(){
//     // Tarika 1: Khali String
//     // let mut s1=String::new();
//     // s1.push_str("hello");
//     // println!("{}",s1);

//     // Tarika 2: to_string() ka use karke
//     // let s2="pehle se likha text".to_string();
//     // println!("{}",s2);

//     // Tarika 3: String::from() ka use karke (Most Common)
//     // let s3=String::from("pehle se likha text");
//     // println!("{}",s3);

//     // Rust ki Strings 100% UTF-8 support karti hain:
//     // let hindi = String::from("नमस्ते");
//     // let japanese = String::from("こんにちは");
//     // println!("{}",hindi);
//     // println!("{}",japanese);

// }

// 3. String Update Karna (Appending & Concatenation)
// A. Data add karna (push_str aur push)

//     push_str(): Poori string slice append karne ke liye (ownership nahi leta).

//     push(): Single character (char) append karne ke liye.

// fn main() {
//     let mut s = String::from("foo");
//     let suffix = "bar";
//     s.push_str(suffix);
//     println!("suffix abhi bhi valid hai: {suffix}");

//     s.push('!');
//     println!("{}", s);
// }

// B. Strings ko jodna (+ Operator ka Khel)

// + operator Rust ke add function ko call karta hai: fn add(self, s: &str) -> String
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");

//     let s3 = s1 + s2;
//     println!("{}", s3);
// }

//  PS C:\Users\dell\projects\hello_world\string> cargo run

//    Compiling string v0.1.0 (C:\Users\dell\projects\hello_world\string)

// error[E0308]: mismatched types                                                                   

//   --> src\main.rs:61:19

//    |

// 61 |     let s3 = s1 + s2;

//    |                   ^^ expected `&str`, found `String`

//    |

// help: consider borrowing here

//    |

// 61 |     let s3 = s1 + &s2;

//    |                   +


// For more information about this error, try `rustc --explain E0308`.                              

// error: could not compile `string` (bin "string") due to 1 previous error                         

// PS C:\Users\dell\projects\hello_world\string> 

// Yeh error Rust mein + operator ke signature (rules) ki vajah se aa raha hai.

// Rust mein jab aap do Strings ko + operator se jodte (concatenate karte) hain, toh Rust internal level par add method ko use karta hai, jiska design aisa hota hai:

// fn add(self, s: &str) -> String

// Is rule ko samajhna bohot zaroori hai:Pehla Element (s1): Yeh String hona chahiye (bina & ke). Rust s1 ki ownership le leta hai aur naye characters ko s1 ke buffer mein hi append (jod) deta hai.Dusra Element (s2): Yeh &str (String slice/reference) hona chahiye. Yeh ek String nahi ho sakta.Aapke Code Mein Kya Galti Huyi?Aapne dono variables ko String banaya tha:s1 $\rightarrow$ Strings2 $\rightarrow$ String

// Jab aapne s1 + s2 likha, toh Rust ko dusre place par &str chahiye tha, lekin aapne use pura String de diya. Isliye Rust compiler ne error diya: expected &str, found String.

// Solution (Isse Kaise Sahi Karein?)
// Dusre variable (s2) ke aage & (reference) laga dein:

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = String::from("world");

//     // s2 ke aage `&` lagaya
//     let s3 = s1 + &s2; 

//     println!("{}", s3); // Output: helloworld
// }

