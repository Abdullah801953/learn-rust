fn main() {
    // ---------------scaler type data type---------------------
    // unsigned integer data type
    // let x: u8=25*3;
    // println!("{x}");

    // signed integer data type
    // let x: i8=-128;
    // println!("{x}");

    // float 32 data type
    // let x: f32 = 2.3;
    // println!("{x}");

    // float 64 data type
    // let x: f64 =45.6;
    // println!("{x}");

    // bool data type
    // let x: bool = true;
    // println!("{x}");

    // character type
    // let x: char = 'h';
    // println!("{x}");
    // --------------------------------------------------------------

    // ---------------------------Compound type data type------------------------------

    // tuple data type
    // let tup: (i32, u32, f64) = (23, 45, 4.5);
    // let (x, y, z) = tup;
    // println!("{y}");
    // println!("{x}");
    // println!("{z}");

    // let twenty_three=tup.0;
    // let fourty_five=tup.1;
    // let four_point_five=tup.2;
    // println!("{},{},{}",twenty_three,fourty_five,four_point_five);

    // Array data type
    let a=[1,2,3,4,5];
    let first=a[0];
    let second=a[1];
    println!("{},{}",first,second);

    // Same value se initialize karna: [value; length]
    // Iska matlab hai: 5 elements, sabki value 0 ([0, 0, 0, 0, 0])
    let c=[0;5];
    let first=c[0];
    let second=c[1];
    println!("{},{}",first,second)
}
