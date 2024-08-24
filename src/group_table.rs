pub mod group_table_0;
pub mod group_table_1;
pub mod group_table_2;
pub mod group_table_3;
pub mod group_table_4;

fn is_hm_greater_1(a: &u8, b: &u8) -> bool {
    (a ^ b).count_ones() > 1
}

fn is_hm_greater_1_wildcards(a: &str, b: &str) -> bool {
    let mut hamming_distance: i8 = 0;

    for (bit_a, bit_b) in a.chars().zip(b.chars()) {
        if bit_a != bit_b {
            hamming_distance += 1;
        }
    }

    return hamming_distance > 1;
}

fn generate_combined_bin_value(val1: &u8, val2: &u8) -> String{
    let bin_a: String = format!("{:05b}", val1);
    let bin_b: String = format!("{:05b}", val2);
    let mut bin_result: String = String::new();

    for (bit_a, bit_b) in bin_a.chars().zip(bin_b.chars()) {
        if bit_a == bit_b {
            bin_result.push(bit_a);  //if 0 - 0 => 0 or 1 - 1 => 1
        } else {
            bin_result.push('-');     //if 0 - 1 => - or 1 - 0 => -
        }
    }
    return bin_result;
}

fn generate_combined_bin_value_wildcards(bin_a: &str, bin_b: &str) -> String{
    let mut bin_result: String = String::new();

    for (bit_a, bit_b) in bin_a.chars().zip(bin_b.chars()) {
        if bit_a == bit_b {
            bin_result.push(bit_a);  //if 0 - 0 => 0 or 1 - 1 => 1
        } else {
            bin_result.push('-');     //if 0 - 1 => - or 1 - 0 => -
        }
    }
    return bin_result;
}
