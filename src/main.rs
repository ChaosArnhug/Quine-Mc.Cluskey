mod quine_cluskey;

use quine_cluskey::QuineCluskey;

fn main() {
    let data: Vec<u8> = vec![0,1,3,4,5,7];
    let test: QuineCluskey = QuineCluskey::new(data);
    println!("{}", test);
}