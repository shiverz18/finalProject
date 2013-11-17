use std::rand;
use std::rand::RngUtil;

fn main() {
    let mut rng = rand::rng();
    let x = [rand::Weighted {weight: 4, item: 'a'},
             rand::Weighted {weight: 2, item: 'b'},
             rand::Weighted {weight: 2, item: 'c'}];
    printfln!("%c", rng.choose_weighted(x));
}
