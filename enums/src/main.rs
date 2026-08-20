// 1. Enum Kya Hota Hai? (Basic Enums)

// Enum (Enumeration) ka matlab hota hai ek aisa type jiske andar sirf kuch fixed choices (variants) ho sakti hain.

// Example:
// Maan lijiye aap ek delivery app bana rahe hain. Ek Order ka status sirf in 4 me se ek ho sakta hai:

//     Pending

//     Shipped

//     Delivered

//     Cancelled

// enum OrderStatus {
//     Pending,
//     Shipped,
//     Delivered,
//     Cancelled,
// }
// fn print_status(status: OrderStatus) {
//     match status {
//         OrderStatus::Pending => println!("⏳ Order abhi Pending hai."),
//         OrderStatus::Shipped => println!("🚚 Order raste me hai (Shipped)."),
//         OrderStatus::Delivered => println!("✅ Order Deliver ho gaya!"),
//         OrderStatus::Cancelled => println!("❌ Order Cancel ho gaya."),
//     }
// }
// fn main(){
//     let my_order=OrderStatus::Shipped;
//     let old_order=OrderStatus::Delivered;

//     print_status(my_order);
//     print_status(old_order);
// }

// 2. Enum Ke Andar Data Store Karna (Enums with Data)
// Rust ke Enums ki sabse badi taakat yeh hai ki aap har variant ke andar alag-alag data attach kar sakte hain.

// Example:

//     Quit: Isme koi data nahi hai.

//     Move: Isme { x, y } coordinates hain.

//     Write: Isme ek String message hai.

//     ChangeColor: Isme 3 numbers (RGB) hain.

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn process_message(msg: Message) {
//     match msg {
//         Message::Quit => {
//             println!("Door band ho raha hai... Quit!");
//         }
//         Message::Move { x, y } => {
//             println!("Player position change hui: x={}, y={}", x, y);
//         }
//         Message::Write(text) => {
//             println!("Naya text message: {}", text);
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Color badal gaya: Red={}, Green={}, Blue={}", r, g, b);
//         }
//     }
// }
// fn main(){
//     let msg1=Message::Write(String::from("Hello Rust!"));
//     let msg2=Message::Move { x: 10, y: 20 };
//     let msg3=Message::ChangeColor(225, 0, 0);
//     let msg4=Message::Quit;

//     process_message(msg1);
//     process_message(msg2);
//     process_message(msg3);
//     process_message(msg4);
// }

// 3. Option<T> Enum (Null/Empty Values Ka Ilaaj)

// Doosri programming languages (like Java, C++, Python) me null ya None hota hai, jisse aksar Null Pointer Exception / Crash ho jata hai.

// Rust me null hota hi nahi hai! uske jagah standard library me Option<T> enum use hota hai:
// enum Option<T> {
//     Some(T), // Value present hai (Value T ke andar hai)
//     None,    // Value absent hai (Kuch nahi hai)
// }

// enum MyOption<T> {
//     Some(T),
//     None,
// }

// fn divide(numerator: f64, denominator: f64) -> MyOption<f64> {
//     if denominator == 0.0 {
//         MyOption::None
//     } else {
//         MyOption::Some(numerator / denominator)
//     }
// }

// fn main() {
//     let result1 = divide(10.0, 2.0);
//     let result2 = divide(10.0, 0.0);

//     match result1 {
//         MyOption::Some(val) => println!("10 / 2 ka answer hai: {}", val),
//         MyOption::None => println!("Zero se divide nahi kar sakte!"),
//     }

//     match result2 {
//         MyOption::Some(val) => println!("10 / 0 ka answer hai: {}", val),
//         MyOption::None => println!("Error: Zero se divide nahi kar sakte!"),
//     }
// }

// 4. Pattern Matching (match aur Catch-all _)

// match aapko har ek case ko check karne par majboor karta hai. Isme aap _ (Catch-all) ka use karke baaki bache saare cases ko handle kar sakte hain.

// fn main() {
//     let dice_roll = 4;

//     match dice_roll {
//         1 => println!("Aapko 1 mila! Ek step piche jao."),
//         6 => println!("Aapko 6 mila! Ek baar fir se dice ghumaao!"),
//         // '_' ka matlab hai: "1 aur 6 ke alawa baaki koi bhi number"
//         _ => println!("Aapko {} mila. Normal move karo.", dice_roll),
//     }
// }

// 5. if let (Jab Sirf Ek Hi Variant Check Karna Ho)

// Agar aapko Enum ke 10 variants me se sirf 1 variant me interest hai, toh match lamba lagta hai. Iske liye if let shortcut hai.

//  fn main() {
//     let config_max: Option<u8> = Some(100);

//     // OPTION 1: Match se likhna (Thoda Lamba code)
//     println!("--- Match Se ---");
//     match config_max {
//         Some(max) => println!("Max value match se: {}", max),
//         _ => (), // Baaki sab ignore kar do
//     }

//     // OPTION 2: 'if let' se likhna (Short & Clean)
//     println!("--- If Let Se ---");
//     if let Some(max) = config_max {
//         println!("Max value if-let se: {}", max);
//     }
// }

