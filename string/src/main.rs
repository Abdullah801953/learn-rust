// Rust mein Strings seekhna thoda challenging lag sakta hai kyunki baaki languages (jaise JavaScript ya Python) strings ke andar ki complexities ko chhupa deti hain, jabki Rust aapko unhe directly handle karne par majboor karta hai.Rust mein String basically Bytes ka ek Vector (Vec<u8>) hota hai, jo strictly UTF-8 Encoded hota hai.1. String vs &str Mein Kya Farak Hai?Rust mein do sabse main string types hoti hain:Property&str (String Slice)StringOwnershipBorrowed (kisi aur jagah stored data ka reference)Owned (heap par store hota hai)SizeFixed / ImmutableGrowable / MutableMemoryStack, Heap, ya Binary embeddedExclusively HeapJS AnalogyRead-only referenceStringBuilder ya Dynamic String

fn main() {
    // 1. String Literal (&str): Program ke binary mein hardcoded hota hai
    let literal: &str = "Hello World";
    println!("{literal}");
}
