mod chaotic_maps;
use chaotic_maps::{ChaoticMaps};

fn main() {
    println!("Hello World");
    let henon = ChaoticMaps::Henon{name: String::from("I'm Henon")};
    println!("{}", henon.get_name());
}