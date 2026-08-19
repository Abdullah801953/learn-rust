// 1. Methods Kya Hote Hain? (Functions Se Alag Kyun Hain?)

// Functions aur Methods dono fn keyword se bante hain, lekin inme 2 bade fark hain:

//     Context: Normal functions bahar hote hain, jabki Methods impl (Implementation) block ke andar define kiye jaate hain.

//     self Parameter: Method ka pehla parameter hamesha self hota hai, jo us Struct ke instance (object) ko represent karta hai.

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("Area: {}", rect1.area());
// }

// 2. self Ke Teeno Forms (&self, &mut self, self)

// Method mein self ko 3 alag tarikon se pass kiya ja sakta hai:

//     &self (Immutable Borrow): Subse zyada use hota hai. Yeh sirf data ko read karta hai, modify nahi karta na ownership leta hai.

//     &mut self (Mutable Borrow): Jab method ke andar Struct ki kisi field ko badalna (change करना) ho.

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn scale(&mut self, factor: u32) {
//         self.width *= factor;
//         self.height *= factor;
//     }
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let mut rect = Rectangle {
//         width: 20,
//         height: 30,
//     };
//     rect.scale(2);
//     println!("area of rectangle is :{}", rect.area());
// }

// struct Rectangle {
//     width: u32,
// }
// impl Rectangle {
//     fn destroy_and_get_width(self) -> u32 {
//         self.width
//     }
// }
// fn main() {
//     let rect = Rectangle { width: 10 };

//     let w = rect.destroy_and_get_width(); // Ownership rect se method mein move ho gayi

//     println!("Width was: {}", w);
//     // println!("{}", rect.width); // ❌ ERROR: rect move ho chuka hai, ab use nahi ho sakta!
// }

// 3. Field aur Method Ka Same Naam (Getters Concept)
// Rust mein aap Field ka naam aur Method ka naam exact SAME rakh sakte hain:

// struct Rectangle {
//     width: u32,
// }
// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
// fn main(){
//     let rect1=Rectangle{width:30};
//     if rect1.width(){
//         println!("Width is nonzero: {}", rect1.width);
//     }
// }
// Getters: Kuch languages mein field values lene ke liye Getters hote hain. Rust mein hum field ko private karke method ko public rakh kar read-only access dete hain.

// 4. Automatic Referencing and Dereferencing (-> Operator Kyun Nahi Hai?)

// C/C++ mein object ke paas direct call ke liye . aur pointer ke liye -> use hota hai (object->method()).

// Rust mein -> operator NAHI hota! Rust automatic reference manage karta hai. Jab aap rect1.area() likhte hain, toh Rust background mein khud (&rect1).area() kar leta hai.

// 5. Extra Parameters Wale Methods
// Methods mein self ke alawa bhi extra parameters pass kiye ja sakte hain:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn is_hold(&self, other: &Rectangle) -> bool {
//         self.width >= other.width && self.height >= other.width
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 20,
//         height: 30,
//     };
//     let rect2 = Rectangle {
//         width: 40,
//         height: 70,
//     };
//     println!("is rect1 can hold rect2 ? {}", rect1.is_hold(&rect2));
// }

// 1. Value Pass Kaise Ho Raha Hai?Jab aap rect1.is_hold(&rect2) call karte hain, toh Rust background mein ise Rectangle::is_hold(&rect1, &rect2) ki tarah treat karta hai:rect1 $\rightarrow$ &self: Jab aap dot (.) laga kar method call karte hain (rect1.is_hold), toh rect1 automatic pehle parameter &self mein chala jata hai. Isse method ko rect1 ka data (width: 20, height: 30) mil jata hai.&rect2 $\rightarrow$ other: Jo value aap bracket ke andar pass karte hain (&rect2), wo dusre parameter other mein chali jati hai. Isse method ko rect2 ka data (width: 40, height: 70) mil jata hai.2. Calculation Ka Logic (Step-by-Step)Ab method ke andar ki values yeh hain:self (rect1): width = 20, height = 30other (rect2): width = 40, height = 70Aapka condition hai: self.width >= other.width && self.height >= self.widthPehla Check (self.width >= other.width):$20 \ge 40$ $\rightarrow$ falseDusra Check (self.height >= self.width):$30 \ge 20$ $\rightarrow$ trueFinal Result (false && true):&& (AND operator) tabhi true deta hai jab dono side true ho.false && true ka result false aayega.

// 6. Associated Functions (Bina self Wale Functions)

// impl block ke andar kuch aise functions bhi hote hain jinme pehla parameter self nahi hota. Inhe Associated Functions kehte hain.

//     Yeh kisi specific instance par nahi, balki poore Struct Type par kaam karte hain.

//     Inhe aksar Constructor (naya instance banane) ke liye use kiya jata hai.

//     Inhe call karne ke liye :: (Double Colon) syntax ka use hota hai (jaise String::from()).
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }
// fn main() {
//     let sq = Rectangle::square(20);
//     println!("square: {:#?}", sq);
// }
// Code ka Line-by-Line Breakdown

//     struct Rectangle { width: u32, height: u32, }

//         Yahan humne ek Rectangle naam ka custom data type (Struct) banaya hai.

//         Iske andar do properties hain: width aur height, dono ka type u32 (Unsigned 32-bit integer, yaani positive numbers) hai.

//     impl Rectangle {

//         impl ka matlab hai Implementation.

//         Is block ke andar hum aise functions ya methods likhte hain jo Rectangle se jude hote hain.

//     fn square(size: u32) -> Self {

//         square naam ka ek function banaya jo input mein size naam ka number leta hai.

//         Sabse important baat: Isme pehla parameter &self nahi hai. Iska matlab yeh ek Associated Function (constructor ki tarah) hai, jise bina kisi purane object ke direct call kiya ja sakta hai.

//         -> Self: Yahan Self (capital S) ka matlab hai Rectangle struct khud. Yeh function ek naya Rectangle bana kar return karega.

//     Self { width: size, height: size }

//         Yeh code ek naya Rectangle create kar raha hai.

//         Square (varg) mein width aur height barabar hoti hain, isliye width aur height dono ko humne size ke barabar set kar diya.

//         Note: Is line ke aage semicolon (;) nahi hai, kyunki Rust mein aakhri expression automatic return ho jati hai.

//     } (impl ka closing)

//         impl block yahan khatam hota hai.

// Main Function Line-by-Line

//     fn main() {

//         Program ki execution yahan se shuru hoti hai.

//     let sq = Rectangle::square(20);

//         Hum Rectangle::square(20) call kar rahe hain. Rust mein associated functions ko call karne ke liye :: (double colon) ka use hota hai.

//         Humne size = 20 pass kiya. Isne background mein { width: 20, height: 20 } wala Rectangle banaya aur usko sq variable mein store kar diya.

//     println!("square: {:#?}", sq);

//         Yeh line sq ko screen par print kar rahi hai.

// 7. Multiple impl Blocks
// Ek Struct ke liye aap ek se zyada impl blocks likh sakte hain. Yeh bilkul valid hai:

// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main(){
//     let rect1=Rectangle{width:20,height:30};
//     let rect2=Rectangle{width:40,height:50};
//     println!("is rect1 can hold rect2 ? {}",rect1.can_hold(&rect2));
//     println!("area of rectangle is: {}",rect1.area());
// }

