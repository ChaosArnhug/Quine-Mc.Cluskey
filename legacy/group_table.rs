pub mod group_table_0;
pub mod group_table_1;
pub mod group_table_2;
pub mod group_table_3;
pub mod group_table_4;

use group_table_0::GroupTable0;
use group_table_1::GroupTable1;
use group_table_2::GroupTable2;
use group_table_3::GroupTable3;
use group_table_4::GroupTable4;

fn test() {
    let test_minterms: [u8; 6] = [0,1,3,4,5,7];

    let mut gt0_test = GroupTable0::new(&test_minterms);
    let mut gt1_test = GroupTable1::new(&mut gt0_test);
    let mut gt2_test = GroupTable2::new(&mut gt1_test);
    let mut gt3_test = GroupTable3::new(&mut gt2_test);
    let gt4_test = GroupTable4::new(&mut gt3_test);
    println!("{}", gt0_test);
    println!("{}", gt1_test);
    println!("{}", gt2_test);
    println!("{}", gt3_test);
    println!("{}", gt4_test);
}

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
