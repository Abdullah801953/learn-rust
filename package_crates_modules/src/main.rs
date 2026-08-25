// 1. Crate Kya Hai? (Code ke saath)Crate Rust ka sabse chota code unit hai jo compile hota hai.A. Binary Crate (Executable Code)Yeh wo code hota hai jisme fn main() hota hai aur isse ek .exe ya runnable file banti hai.File: src/main.rsRust// Binary Crate ka entry point
// fn main() {
//     println!("Hello, main binary crate se!");
// }
// B. Library Crate (Reusable Code)Isme main() function nahi hota. Yeh sirf functions, structs, aur enums ko hold karta hai taaki doosre projects ise import karke use kar sakein.File: src/lib.rsRust// Library Crate: Yeh sirf functions aur logic provide karta hai
// pub fn add_numbers(a: i32, b: i32) -> i32 {
//     a + b
// }
// 2. Package Kya Hai? (Folder structure ke saath)Package ek aisa Container (Folder) hota hai jisme aapka Cargo.toml hota hai aur ek se zyada Crates (Binary + Library) ek saath pack hote hain.Real Package Folder StructureAap apne terminal par yeh command chalayein:Bash cargo new my_app
// Cargo aapke liye ek Package create kar deta hai jiska structure aisa dikhta hai:Plaintextmy_app/                          <--- YEH POORA PACKAGE HAI
// ├── Cargo.toml                   <--- Package ki Metadata file
// └── src/
//     ├── lib.rs                   <--- 1st Crate (Library Crate)
//     ├── main.rs                  <--- 2nd Crate (Main Binary Crate)
//     └── bin/
//         └── admin_tool.rs        <--- 3rd Crate (Another Binary Crate)
// 3. Ek Hi Package Mein Crates Aapas Mein Kaise Baat Karte Hain? (Real Code)Aaiye ek poora working project dekhte hain jisme 1 Package hai aur uske andar 1 Library Crate aur 2 Binary Crates aapas mein juda hue hain.📄 File 1: Cargo.toml (Package Config)Ini, TOML[package]
// name = "my_app"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// 📄 File 2: src/lib.rs (Library Crate)Rust// Yeh Library Crate ka core function hai
// pub fn calculate_tax(salary: f64) -> f64 {
//     salary * 0.10 // 10% tax
// }
// 📄 File 3: src/main.rs (Main Binary Crate)Yeh binary crate package ki library ko use karega:Rust// Package name ('my_app') se library crate ko import kar rahe hain
// use my_app::calculate_tax;

// fn main() {
//     let salary = 50000.0;
//     let tax = calculate_tax(salary);
//     println!("Main App: Salary {} par Tax hai: {}", salary, tax);
// }
// 📄 File 4: src/bin/admin_tool.rs (Second Binary Crate)Yeh ek alag executable tool hai jo src/bin/ folder ke andar hai:Rustuse my_app::calculate_tax;

// fn main() {
//     println!("Admin Tool Running...");
//     let admin_tax = calculate_tax(100000.0);
//     println!("Admin Tax Report: {}", admin_tax);
// }
// Inhe Run Kaise Karein?Terminal par jaakar aap alag-alag crates ko ek hi package se run kar sakte hain:Main Binary ko run karne ke liye:Bashcargo run
// Output: Main App: Salary 50000 par Tax hai: 5000Admin Binary ko run karne ke liye:Bashcargo run --bin admin_tool
// Output: Admin Tool Running... Admin Tax Report: 10000Summary TableCheezDescriptionReal-world AnalogyCrateEk single .rs file ya module tree (e.g., main.rs ya lib.rs).Ek Single BookPackageComplete folder jisme Cargo.toml aur 1 ya usse zyada Crates hote hain.Ek Box set jisme multiple books ho sakti hain

// code 
// Binary Crate ka entry point
use package_crates_modules::calculate_tax;

fn main(){
    let salary=50000.0;
    let tax=calculate_tax(salary);
    println!("Main App: Salary {} par Tax hai: {}",salary,tax);
}