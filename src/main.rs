mod group_table;
use group_table::group_table_0::GroupTable0;
use group_table::group_table_1::GroupTable1;
use group_table::group_table_2::GroupTable2;
use group_table::group_table_3::GroupTable3;
use group_table::group_table_4::GroupTable4;

fn main() {
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