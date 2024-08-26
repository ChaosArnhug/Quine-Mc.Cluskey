mod quine_cluskey;

use quine_cluskey::QuineCluskey;

fn main() {
    let data: Vec<u8> = vec![2,3,4,10,12,13];
    let test: QuineCluskey = QuineCluskey::new(data, Some(vec![11,14,15]));
    println!("{}", test);
}