mod length_converter;
mod password_generator;
use rand::Rng;


fn main() {
    println!("{}", random!(0, 90))
}
