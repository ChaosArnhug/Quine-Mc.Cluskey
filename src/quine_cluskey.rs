use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/*
Hashmap => the group tables, group_table_u8
    Hashmap => the groups in a table, ex: gu8, gu8_prime ...
        Vec => list of the combination 
            () :    - bool: used in a further table
                    - string : binary représentation
                    - Vec(u8) : All number responsible for the combinaison

*/ 
/*
@params minterms    Vec<u8> List of all minterms required
@params tables      HashMap<u8, HashMap<u8, RefCell<Vec<(bool, String, Vec<u8>)>>>> Group Table -> group -> liste of all combination possible + bool if used + binary representation
*/
#[derive(Default, Debug)]
pub struct QuineCluskey {
    pub minterms: Vec<u8>,
    pub tables: HashMap<u8, HashMap<u8, RefCell<Vec<(bool, String, Vec<u8>)>>>>,
    pub prime_implicants: Vec<(String, String, Vec<u8>)>,
    nb_bits_needed: u8,
}

impl QuineCluskey {

    /*
    Create and fill the tables based on the minterms in params
     */
    pub fn new (minterms: Vec<u8>) -> QuineCluskey {
        let mut default_structure: QuineCluskey = QuineCluskey {
            minterms: minterms,
            ..Default::default()
        };

        default_structure.set_max_bit();
        default_structure.generate_tables();
        default_structure.get_prime_implicants();

        return default_structure;
    }

    /**
     * Calculate the number of bit needed for the binary representation
     */
    fn set_max_bit(&mut self) {
        if let Some(biggest_minterms) = self.minterms.iter().max() {
            self.nb_bits_needed = 8 - biggest_minterms.leading_zeros() as u8;
        } else {
            self.nb_bits_needed = 0;
        }
    }

    /**
     * Generate the QuineCluskey table
     */
    fn generate_tables (&mut self) {
    
        for group_table_number in 0..(self.nb_bits_needed) {
            let mut groups: HashMap<u8, RefCell<Vec<(bool, String, Vec<u8>)>>> = HashMap::new();
    
            for group_number in 0..(self.nb_bits_needed - group_table_number + 1) {
                let mut group_combinations: Vec<(bool, String, Vec<u8>)> = Vec::new();
    
                // Inital Group table, no combination needed, just split in groups based on number of one in binary representation
                if group_table_number == 0 {
                    for minterm in &self.minterms {
                        if minterm.count_ones() as u8 == group_number {
                            group_combinations.push((false, format!("{:0width$b}", minterm, width=self.nb_bits_needed as usize), vec![*minterm]));
                        }
                    }

                } else {
                    //Retrive the previous group table as you need to combine its group to generate the next group table
                    let previous_gt: &mut HashMap<u8, RefCell<Vec<(bool, String, Vec<u8>)>>> = self.tables.get_mut(&(group_table_number - 1)).unwrap();

                    //Ex: combinations_x = G0 and combinations_y = G1
                    let mut combinations_x: std::cell::RefMut<Vec<(bool, String, Vec<u8>)>> = previous_gt.get(&group_number).unwrap().borrow_mut();
                    let mut combinations_y: std::cell::RefMut<Vec<(bool, String, Vec<u8>)>> = previous_gt.get(&(group_number + 1)).unwrap().borrow_mut();

                    /*
                     * New value in the group of the new group table is a value comming from the cartesian product of 2 groups from the previous table
                     * that has a hamming distance not greater than 1 
                     */
                    for combination_x in combinations_x.iter_mut() {
                        for combination_y in combinations_y.iter_mut() {
                            if !Self::is_hm_greater_1_wildcards(&combination_x.1, &combination_y.1) {
                                combination_x.0 = true;
                                combination_y.0 = true;
                                
                                let mut new_combination: Vec<u8> = Vec::new();
                                new_combination.extend(&combination_x.2);
                                new_combination.extend(&combination_y.2);
                                group_combinations.push((false, Self::generate_combined_bin_value_wildcards(&combination_x.1, &combination_y.1), new_combination));
                            }
                        }
                    }

                }
                Self::remove_duplicate_tuples(&mut group_combinations);
                groups.insert(group_number, RefCell::new(group_combinations));
            }
    
            self.tables.insert(group_table_number, groups);
        }
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

    fn remove_duplicate_tuples(tuples: &mut Vec<(bool, String, Vec<u8>)>){
        let mut seen: HashSet<String> = HashSet::new();

        tuples.retain(|(_, binary_string, _)| seen.insert(binary_string.clone()));

    }

    fn get_prime_implicants(&mut self) {
        let mut prime_implicants: Vec<(String, String, Vec<u8>)> = Vec::new();

        for group_table in self.tables.values() {
            for group in group_table.values() {
                for combination in group.borrow().iter() {
                    if !combination.0 {
                        prime_implicants.push((Self::tanslate_binary_into_variable_letters(&combination.1), combination.1.clone(), combination.2.clone()));
                    } 
                }
            }
        }

        self.prime_implicants = prime_implicants;
    }

    fn tanslate_binary_into_variable_letters (binary: &String) -> String {
        let positive_letters: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let negative_letters: [&str; 8] = ["A\'", "B\'", "C\'", "D\'" , "E\'", "F\'", "G\'", "H\'"];
        let mut translation: String = String::new();

        for (i, bit) in binary.chars().enumerate() {
            if bit == '0' {
                translation.push_str(negative_letters[i]);
            }

            if bit == '1' {
                translation.push(positive_letters[i]);
            }
        }
        return translation;
    }
}

impl std::fmt::Display for QuineCluskey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_table = String::new();

        for group_table_number in 0..(self.nb_bits_needed) {
            display_table.push_str(&format!("Group Table {}\n=============================\n| Gi |{:^width_bool$}|{:^width_bin$}| dec |\n-----------------------------\n", group_table_number, "Used", "bin", width_bin=8 as usize, width_bool=7 as usize));
            
            for group_number in 0..(self.nb_bits_needed - group_table_number + 1) {
                let combinations: std::cell::Ref<Vec<(bool, String, Vec<u8>)>> = self.tables.get(&group_table_number).unwrap().get(&group_number).unwrap().borrow();

                if combinations.len() == 0 {
                    display_table.push_str(&format!("| G{group_number} | ///// | ////// | /// |\n"));

                } else {
                    for combination in combinations.iter() {
                        let mut decimal_representation: String = format!("-");
                        for number in &combination.2 {
                            decimal_representation.push_str(&format!(" {:02} -", number));
                        }
                        display_table.push_str(&format!("| G{} |{:^width_bool$}|{:^width_bin$}| {} |\n", group_number, combination.0, combination.1, decimal_representation, width_bin=8 as usize, width_bool=7 as usize));
                    }   
                }
                display_table.push_str("-----------------------------\n");
            }
            display_table.push_str("\n\n");
        }
        
        display_table.push_str("Prime Implicants\n================\n");
        for i in 0..self.prime_implicants.len() {
            display_table.push_str(&format!("PI{} : {}\n", i + 1, &self.prime_implicants[i].0));
        }

        display_table.push_str("\n\n");

        write!(f, "{}", display_table)
    }
}