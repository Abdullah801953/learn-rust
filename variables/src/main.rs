// making immutable variable mutable
// fn main() {
// let mut x=4;
// println!("{x}");
// x=5;
// println!("{x}");

// }

// making constant variable
// fn main() {
//     let THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
//     {
//         println!("{THREE_HOUR_IN_SECONDS}");
//     }

//     println!("{THREE_HOUR_IN_SECONDS}");
// }

// shadowing
// fn main(){
//     let x=2;
//     let x=x+3;
//     {
//         let x=x+1;
//         println!("block scope {x}");
//     }
//     println!("outer scope {x}");
// }

// cont used in outside main function but let not used outside main function
// const X: u32=5;
// fn main(){
// println!("{X}");
// }

// let x = 5;
// fn main() {
//     println!("{x}");
// }
// cargo run
//    Compiling variables v0.1.0 (C:\Users\dell\projects\hello_world\variables)
// error: expected item, found keyword `let`                                                         
//   --> src\main.rs:36:1
//    |
// 36 | let x = 5;
//    | ^^^
//    | |
//    | `let` cannot be used for global variables
//    | help: consider using `static` or `const` instead of `let`
//    |
//    = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>

// error: could not compile `variables` (bin "variables") due to 1 previous error  
fn main(){
    
}

