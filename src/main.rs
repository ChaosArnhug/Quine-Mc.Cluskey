mod quine_cluskey;

use std::io;

use quine_cluskey::QuineCluskey;

fn main() {
    let data = get_user_input();
    let quine: QuineCluskey = QuineCluskey::new(data.0, Some(data.1));
    println!("{}", quine);
}

fn get_user_input () -> (Vec<u8>, Vec<u8>) {
    let mut input_minterms: String = String::new();
    let mut input_dont_care: String = String::new();
    
    println!("Enter minterms between 0 and 255 separated by a coma (',')");
    io::stdin().read_line(&mut input_minterms).expect("Failed to read minterms input");

    println!("Enter the don't care terms between 0 and 255 separated by a coma (',')\nNote that if a term is both a minterm and a don't care, the term will be considered a minterm");
    io::stdin().read_line(&mut input_dont_care).expect("Failed to read don't care input");

    return (convert_input_into_minterms(input_minterms), convert_input_into_minterms(input_dont_care));
}

fn convert_input_into_minterms(input: String) -> Vec<u8> {
    
    input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .collect()
}