use package_crates_modules::calculate_tax;

fn main() {
    println!("Admin Tool Running...");
    let admin_tax = calculate_tax(100000.0);
    println!("Admin Tax Report: {}", admin_tax);
}