// 1. Module ko declare karein (taki compiler ko pata chale garden.rs file exist karti hai)
pub mod garden;

use crate::garden::vegetables::Asparagus;

fn main(){
    let plant=Asparagus{};
    println!("I am growing {:?}",plant)
}
