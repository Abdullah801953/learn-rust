// 1. Struct Kya Hota Hai Aur Yeh Tuples Se Alag Kyun Hai?

// Tuples aur Structs dono alag-alag types ke data ko ek saath rakh sakte hain. Lekin:

//     Tuple: Isme fields ke naam nahi hote, sirf index (.0, .1) hote hain.

//     Struct: Isme har field ka ek clear naam (Key) aur type (Value) hota hai.

// Basic Struct Definition Syntax:

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("rahul123"),
//         email: String::from("rahul@example.com"),
//         sign_in_count: 1,
//     };

//     // Value access karne ka tarika:
//     println!("Username: {}", user1.username);
//     println!("Email: {}", user1.email);
//     println!("Sign in count: {}", user1.sign_in_count);
//     println!("Active: {}", user1.active);
// }

// Important Rule: Rust mein poora instance mut (mutable) hona chahiye. Aap kisi single field ko alag se mut nahi bana sakte.

// 2. Struct Ka Instance Kaise Banate Aur Modify Karte Hain?
// Struct ka istemal karne ke liye hum uska ek instance (object) banate hain:

// fn main() {
//     // Instance Create Karna
//     let mut user1 = User {
//         active: true,
//         username: String::from("rahul123"),
//         email: String::from("rahul@example.com"),
//         sign_in_count: 1,
//     };

//     // Dot Notation (.) se Value access karna ya badalna
//     user1.email = String::from("newemail@example.com");
// }

// 3. Fields Access & Mutability RulesAccessing Value: Dot notation se access karte hain $\rightarrow$ user1.email.Modifying Value: Agar instance mutable hai (let mut user1), toh field change kar sakte hain:Rustuser1.email = String::from("another@example.com");

// 4. Do Super Shortcuts (Shorthand Tricks)
// A. Field Init Shorthand

// Jab function parameter ka naam aur struct field ka naam exact same ho, toh do baar likhne ki zaroorat nahi hoti:

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username, // `username: username` likhne ki jagah sirf `username`
//         email,    // `email: email` ki jagah sirf `email`
//         sign_in_count: 1,
//     }
// }

// B. Struct Update Syntax (..)

// Agar aapko kisi purane instance se milta-julta naya instance banana ho, toh baaki fields copy/move karne ke liye .. ka use hota hai:

// let user2=User{
//     email:String::from("another@gmail.com"),
//     ..user1,// Baaki saare fields (active, username, sign_in_count) user1 se le lo
// }

// 5. Structs Ke TypesRust mein 3 types ke structs hote hain:Struct TypeSyntax ExampleCommon Use CaseNamed Field Structstruct User { name: String }General data models (User, Product, Order).Tuple Structstruct Color(i32, i32, i32);Naming a tuple without redundant field names (RGB colors, Coordinates).Unit-Like Structstruct AlwaysEqual;Jab zero data store karna ho, par Traits (Interfaces) implement karne hon.

// Partial Move kehte hain. Jab aap ..user1 likhte hain, toh Rust har field ko alag-alag check karta hai ki use Move karna hai ya Copy.
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("rahul123"),
//         email: String::from("rahul@example.com"),
//         sign_in_count: 1,
//     };
//     let user2=User{
//         email:String::from("new_email@example.com"),
//         ..user1
//     };
//     println!("user1 email:{}",user1.email);
//     println!("user1 active:{}",user1.active);
//     println!("user1 count:{}",user1.sign_in_count);
//     println!("user1 username:{}",user1.username);

// }

// Important Move & Copy Rule in Update Syntax:

//     username (String): Yeh Heap Data hai. Updating ke waqt user1.username ka ownership Move ho jata hai user2 mein. Iske baad aap user1 ko access nahi kar sakte.

//     active (bool) & sign_in_count (u64): Yeh Stack types hain jo Copy trait implement karti hain.

//     user1.email: Yeh invalid nahi hua kyunki user2 ne nayi email provide ki thi, toh purana email move nahi hua.

// 6. Tuple Structs (Named Tuples)

// Field ke naam dene ki zaroorat na ho par ek alag Type banana ho, tab Tuple Structs kaam aate hain.

// Rust Book ka yeh Chapter 5.2 step-by-step sikhata hai ki kaise hum ek aam program ko Structs aur Debug Traits ka use karke clean, readable aur type-safe banate hain.

// Step 1: Single Variables Se Start Karna (Shuruati Tarika)
// Isme width aur height ke liye alag-alag variables use kiye gaye hain:

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
// fn area(width:u32,height:u32)->u32{
//     width*height
// }

// Kami: area function 2 alag parameters leta hai. Code padhkar yeh pata nahi chalta ki width1 aur height1 aapas mein ek hi Rectangle se jude hain.

// Step 2: Tuples Se Refactor Karna
// Group banane ke liye Tuples ka use kiya gaya:

// fn main() {
//     let rec1 = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rec1));
// }
// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }
// Kami: Parameter toh 1 ho gaya, lekin dimensions.0 aur dimensions.1 se yeh clear nahi hota ki width kaunsi hai aur height kaunsi.

// Step 3: Structs Se Refactor Karna (Final Best Code)
// Data ko meaningful naam dene ke liye Rectangle Struct banaya gaya:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1=Rectangle{
//         width:30,
//         height:50,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1) // Reference (&) pass kar rahe hain taaki ownership main() ke paas rahe
//     );
// }
// fn area(rectangle:&Rectangle)->u32{
//     rectangle.width*rectangle.height
// }
// Fayda: Code bilkul clear ho gaya—rectangle.width aur rectangle.height se seedha pata chalta hai kaunsi value kya hai.

// Borrowing (&Rectangle): Hum &rect1 pass karte hain taaki area function data borrow kare, ownership na le. Isse rect1 main() mein aage bhi use ho sakta hai.

// Step 4: Debugging Aur Printing (#[derive(Debug)] & dbg!)

// Struct ko print karne ke liye normal {} kaam nahi karta. Iske liye Rust mein Debug Trait outer attribute add kiya jata hai:

#[derive(Debug)] // Debug feature enable karne ke liye
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! value log karke return kar deta hai
        height: 50,
    };

    // Print formats:
    println!("One-line Debug: {:?}", rect1);
    println!("Pretty-print Debug:\n{:#?}", rect1);

    // dbg! Macro: File name & line number ke saath stderr par output deta hai
    dbg!(&rect1);
}