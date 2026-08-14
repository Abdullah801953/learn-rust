// 1. Function Naming & Basic Declaration

// fn main() {
//     println!("Hello, world!");
//     newFunction();
// }

// fn newFunction(){
//     println!("this is new function");
// }
// got this error when naming convantion is not snacke case
// cargo run
//    Compiling function v0.1.0 (C:\Users\dell\projects\hello_world\function)
// warning: function `newFunction` should have a snake case name
//  --> src\main.rs:6:4
//   |
// 6 | fn newFunction(){
//   |    ^^^^^^^^^^^ help: convert the identifier to snake case: `new_function`
//   |
//   = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

// warning: `function` (bin "function") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
//      Running `target\debug\function.exe`
// Hello, world!
// this is new function

// Rust mein function aur variable names ke liye snake_case convention follow kiya jata hai (saare letters lowercase aur words ke beech mein underscore _).

// Note: Rust ko isse koi farak nahi padta ki aapne sub-function ko main() ke upar define kiya hai ya neeche. Bas woh us scope mein defined hona chahiye.

// fn main() {
//     another_function();//function call
// }

// Sub-function definition
// fn another_function() {
//     println!("another function executed!");
// }

// 2. Function Parameters

// Rust ka ek strict rule hai: Aapko har parameter ka Data Type explicitly batana hi padega. Compiler khud guess (infer) nahi karega.

// fn main(){
// print_labeled_measurement(5,'h');
// }

// fn print_labeled_measurement(value:u8,unit_lable:char){
//     println!("the mesurement is: {value}{unit_lable}");
// }

// 3. Statements vs. Expressions

// Rust ek expression-based language hai. Functions ki body Statements aur Expressions se milkar banti hai:

// Statements : Instruction jo koi action perform karti hai lekin koi value return nahi karti.
// Expressions : Jo evaluate hokar ek final value return karti hai.

// Example: Statements
// let y = 6; // Yeh ek Statement hai. Yeh koi value return nahi karta.

// Example: Expressions
// Ek math operation 5 + 6 ek expression hai jo 11 return karta hai. Scope block {} bhi ek expression hota hai!

// fn main() {
//     let y = {
//         let x = 2;
//         x + 1 // 👈 Notice: Iske end me Semicolon (;) NAHI hai!
//     };
//     println!("the value of y is:{y}");
// }

// Semicolon ; Ka Khel:x + 1 (Bina ;) $\rightarrow$ Expression (Value 4 return karega).x + 1; (Semicolon ke saath) $\rightarrow$ Statement ban jata hai (aur yeh Unit type () return karta hai).

// 4. Functions with Return Values

// Jab koi function value return karta hai, toh function signature mein -> arrow ke sath uska Return Type likhte hain.

// 2 Ways to Return:

// Implicit Return (Recommended): Function ke aakhir mein bina semicolon ; ke expression likhein.
// Explicit Return: return keyword aur ; ka use karein (mostly early exit ke liye use hota hai).

// Example 1: Implicit Return
// fn five()->i32{
//     5
// }

// fn main(){
//     let x=five();
//     println!("the value of x is:{x}");
// }

// Example 2: Calculation with Implicit Return
// fn plus_one(x:u8)->u8{
//     x+1
// }

// fn main(){
//     let result=plus_one(2);
//     println!("add 1 in 2 is {result}");
// }

