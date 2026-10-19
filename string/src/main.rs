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

// C. Multiple Strings ke liye format! Macro

// Jab bohot saari strings jodni ho, toh + likhna mushkil ho jata hai. Iske liye format! use karein—yeh kisi ka bhi ownership nahi leta!

// fn main(){
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

// let s=format!("{s1}-{s2}-{s3}");
//     println!("{s}");
// }

// 1. Rust Strings me Indexing (s[0]) Kyun Disallowed Hai?Dusri programming languages (jaise Python, JavaScript, ya C) me aap s[0] karke string ka pehla letter nikal lete hain. Lekin Rust me agar aap aisa likhte hain:Rustlet s1 = String::from("hi");
// let h = s1[0]; // ❌ Compiler Error!
// To compiler E0277 error deta hai. Rust aisa do mukhya kaarno se karta hai: Internal Memory Representation aur Performance Guarantee ($O(1)$).

// 2. Internal Representation (Memory me String Kaise Store Hota Hai?)

// Rust me String actually ek Vec<u8> (vector of bytes) ke upar ek wrapper hota hai jo UTF-8 encoding follow karta hai.

//     ASCII / English characters: Har character 1 byte leta hai.
//     Rust

//     let hello = String::from("Hola");
//     // Length = 4 bytes (H, o, l, a sab 1-1 byte ke hain)

//     Non-ASCII characters (Cyrillic, Hindi, Emojis, etc.): Har character 2 se 4 bytes tak le sakta hai.
//     Rust

//     let hello = String::from("Здравствуйте");
// Length = 24 bytes (Ye 12 letters ka shabd hai, lekin har character 2 bytes leta hai)

// Agar s[0] allowed hota, toh kya hota?

// Agar aap "Здравствуйте" ke index 0 ko access karte:

//     Character 'З' UTF-8 me do bytes se milkar banta hai: [208, 151].

//     Index 0 par byte value 208 hai.

//     Byte value 208 apne aap me koi valid character nahi hai.

//     Agar Rust aapko 208 return karta, toh wo galat aur unexpected hota. Isiliye Rust compile time par hi s[0] ko block kar deta hai taaki runtime par unexpected bugs na aayein.

// 3. Performance Reason: $O(1)$ vs $O(N)$ Time ComplexityComputer science me array[index] ya vector[index] access $O(1)$ (constant time) hona chahiye.UTF-8 me kyunki har character ki length (bytes me) alag ho sakti hai, Rust ko $N$-th character tak pahunchne ke liye string ke start se saare bytes count karne padenge. Ye operation $O(N)$ (linear time) ho jayega.Rust aapko ye illusion nahi dena chahta ki indexing fast hai, isiliye direct [] indexing allow nahi karta.

// 5. Slicing Strings (&s[range])

// Agar aapko byte index ke zariye string slice banana hai, toh aap Range syntax [start..end] use kar sakte hain:
// 5. Slicing Strings (&s[range])

// Agar aapko byte index ke zariye string slice banana hai, toh aap Range syntax [start..end] use kar sakte hain:
// Rust

// let hello = "Здравствуйте";
// let s = &hello[0..4]; // Pehle 4 bytes -> "Зд" (Kyunki har char 2 bytes ka hai)

// ⚠️ Common Pitfall (Runtime Panic):
// Agar aap range ko kisi character ke beech me todte hain (jaise &hello[0..1]), toh Rust program runtime panic (crash) kar jayega:
// Code ki Step-by-Step Working

// Aayein dekhte hain ki background me memory ke andar kya ho raha hai:
// Line 1: let hello = "Здравствуйте";

//     String "Здравствуйте" memory me UTF-8 format me store hoti hai.

//     Russian language ka har letter UTF-8 me 2 bytes leta hai:

//         Letter 'З' = Byte 0 aur Byte 1 ([208, 151])

//         Letter 'д' = Byte 2 aur Byte 3 ([208, 180])

//     Isliye, pehle do letters (Зд) milakar kul 4 bytes bante hain.

// Line 2: let s = &hello[0..4];

//     [0..4] ka matlab hai Byte Index 0 se lekar Byte Index 3 tak (4th byte included nahi hai).

//     Rust memory me check karta hai:

//         Byte 0..2 = 'З' (Valid character boundary)

//         Byte 2..4 = 'д' (Valid character boundary)

//     Kyunki 0 aur 4 dono sahi character boundaries par hain, Rust s ko ek string slice (&str) bana deta hai jiski value "Зд" hoti hai.

// Ek Chota Sa Experiment (Crash Test)

// Agar aap [0..4] ki jagah [0..1] likhenge:
// Rust

// fn main() {
//     let hello = "Здравствуйте";
//     let s = &hello[0..1]; // ❌ Crash ho jayega!
//     println!("{s}");
// }

// Toh Rust runtime par panic (crash) kar jayega, kyunki Index 1 par pehla letter 'З' aadha hi kat-ta hai (character boundary toot jati hai).

// 6. Iterating Over Strings (Sahi Tarika)

// String ke parts par kaam karne ke liye aapko explicit hona padta hai ki aapko characters chahiye ya bytes:
// Iterating over Characters (.chars())

// char (Unicode scalar values) par loop chalane ke liye:

// fn main() {
//     for c in "Зд".chars() {
//         println!("{c}");
//     }
// }

// Iterating over Bytes (.bytes())

// Raw byte values par loop chalane ke liye:

fn main(){
    for c in "Зд".bytes(){
        println!("{c}");
    }
}