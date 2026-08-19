// 1. Enum Kya Hota Hai? (Basic Enums)

// Enum (Enumeration) ka matlab hota hai ek aisa type jiske andar sirf kuch fixed choices (variants) ho sakti hain.

// Example:
// Maan lijiye aap ek delivery app bana rahe hain. Ek Order ka status sirf in 4 me se ek ho sakta hai:

//     Pending

//     Shipped

//     Delivered

//     Cancelled

enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}
fn print_status(status: OrderStatus) {
    match status {
        OrderStatus::Pending => println!("⏳ Order abhi Pending hai."),
        OrderStatus::Shipped => println!("🚚 Order raste me hai (Shipped)."),
        OrderStatus::Delivered => println!("✅ Order Deliver ho gaya!"),
        OrderStatus::Cancelled => println!("❌ Order Cancel ho gaya."),
    }
}
fn main(){
    let my_order=OrderStatus::Shipped;
    let old_order=OrderStatus::Delivered;

    print_status(my_order);
    print_status(old_order);
}