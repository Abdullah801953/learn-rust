// 1. if Expressions
// if expression aapko conditional branches chalane ki permission deta hai.

// fn main() {
//     let number=3;
//     if number<5{
//         println!("number is lesser");
//     }
// }

// ⚠️ Strict Rule: Condition MUST be a bool

// C, C++, JavaScript ya Python mein if (number) likhna valid hota hai (jahan 0 false hota hai aur non-zero true). Lekin Rust mein yeh strict ERROR hai!

// Rust kabhi bhi numbers ya strings ko auto-convert karke bool (truthy/falsy) nahi banata. Condition ka result exact bool (true ya false) hona chahiye

// fn main() {
//     let number = 3;

//     // ❌ COMPILE ERROR! `number` integer hai, bool nahi!
//     /*
//     if number {
//         println!("Number was three");
//     }
//     */
//     // ✅ SAHI TAREEKA: Exact boolean comparison (`number != 0`)
//     if number != 0 {
//         println!("Number is not zero");
//     }
// }

// Multiple Conditions: else if
// Jab 2 se zyada conditions check karni hon:

// fn main() {
//     let number = 4;
//     if number % 2 == 0 {
//         println!("number devisiable by 2");
//     } else if number % 6 == 0 {
//         println!("number devisiable by 6")
//     } else {
//         println!("number is not devisiable by 3 and 4")
//     }
// }

// Using if in a let Statement

// Rust mein if ek expression hai (value return karta hai). Iska matlab aap ise let statement ke right side par use kar sakte ho (jaise baaki languages mein ternary operator ? : hota hai):

// fn main(){
//     let condition=true;
//     let number=if condition {5} else {6};
//     println!("the value of number is:{number}");
// }

// 2. Repetition with Loops

// A. loop (Infinite Loop)
// loop keyword code ko tab tak chalta rehta hai jab tak aap explicitly break na kar dein.

// fn main() {
//     let mut counter = 0;
//     loop {
//         counter += 1;
//         println!("counter:{counter}");
//         if counter == 3 {
//             break;
//         }
//     }
// }

// Returning Values from loop

// Aap loop ko koi value calculate karne ke liye use kar sakte ho aur break ke aage value likhkar us value ko return karva sakte ho:

// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//            break counter * 2;

//         }
//     };
//     println!("the result is: {result}");
// }

// Loop Labels (Nested Loops mein kisi specific loop ko break karna)

// Jab ek loop ke andar doosra loop hota hai, toh aap 'label lagakar specify kar sakte hain ki kaunsa loop break karna hai:

// fn main() {
//     let mut count = 0;

//     'counting_up: loop {
//         println!("count ={count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining={remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
// }

// B. while Loop (Conditional Loop
// Jab tak condition true hai, tab tak loop chalega. Jaise hi condition false hogi, loop ruk jayega.

// fn main() {
//     let mut count = 5;
//     while count != 0 {
//         println!("number:{count}");
//         count -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// C. for Loop (Sabse Safe aur Popular Loop)
// Rust mein collections (array/vectors) par iterate karne ke liye ya specific range chalaney ke liye for loop sabse best, safe aur fast hota hai.

// 1. Array Par Iterate Karna
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     for elements in a {
//         println!("values is:{elements}");
//     }
// }

// 2. Number Range Par Iterate Karna (Range)

// Range ke liye 1..4 (1 se 3 tak) ya 1..=4 (1 se 4 tak) ka use hota hai. Reverse ginti ke liye .rev() method lagate hain:

// fn main() {
//     for number in 1..4 {
//         println!("number:{number}");
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("number:{number}");
    }
}
