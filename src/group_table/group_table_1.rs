use std::collections::HashSet;

use super::{generate_combined_bin_value, group_table_0::GroupTable0, is_hm_greater_1};

#[derive(Default)]
pub struct GroupTable1 {
    pub g0_prime: Vec<(String,u8,u8,bool)>,
    pub g1_prime: Vec<(String,u8,u8,bool)>,
    pub g2_prime: Vec<(String,u8,u8,bool)>,
    pub g3_prime: Vec<(String,u8,u8,bool)>,
    pub g4_prime: Vec<(String,u8,u8,bool)>,
}

impl GroupTable1 {
    pub fn new(group_table_0: &mut GroupTable0) -> GroupTable1 {
        let mut group_table: GroupTable1 = GroupTable1::default();

        group_table.g0_prime = group_table.compare_groups(&mut group_table_0.g0, &mut group_table_0.g1);
        group_table.g1_prime = group_table.compare_groups(&mut group_table_0.g1, &mut group_table_0.g2);
        group_table.g2_prime = group_table.compare_groups(&mut group_table_0.g2, &mut group_table_0.g3);
        group_table.g3_prime = group_table.compare_groups(&mut group_table_0.g3, &mut group_table_0.g4);
        group_table.g4_prime = group_table.compare_groups(&mut group_table_0.g4, &mut group_table_0.g5);

        GroupTable1::remove_duplicate_tuples(&mut group_table.g0_prime);
        GroupTable1::remove_duplicate_tuples(&mut group_table.g1_prime);
        GroupTable1::remove_duplicate_tuples(&mut group_table.g2_prime);
        GroupTable1::remove_duplicate_tuples(&mut group_table.g3_prime);
        GroupTable1::remove_duplicate_tuples(&mut group_table.g4_prime);
        
        return group_table;
    }
    pub fn compare_groups(&mut self, gt0_vec1: &mut Vec<(u8,bool)>, gt0_vec2: &mut Vec<(u8,bool)>) -> Vec<(String,u8, u8, bool)>{
        let mut buffer: Vec<(String,u8,u8, bool)> = Vec::new();
        for term_v1 in gt0_vec1 {
            for term_v2 in &mut *gt0_vec2 {
                if !is_hm_greater_1(&term_v1.0, &term_v2.0) {
                    term_v1.1 = true;
                    term_v2.1 = true;
                    buffer.push((generate_combined_bin_value(&term_v1.0, &term_v2.0), term_v1.0, term_v2.0, false));
                }
            }
        }
        return buffer;
    }

    pub fn remove_duplicate_tuples(tuples: &mut Vec<(String,u8,u8,bool)>){
        let mut seen = HashSet::new();

        tuples.retain(|(first, _, _, _)| seen.insert(first.clone()));

    }
}

impl std::fmt::Display for GroupTable1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_table = String::new();
        display_table.push_str("Group Table 1\n=================================\n\n| Gi |  bin  | dec - dec | Used |\n---------------------------------\n");

        let groups = [&self.g0_prime, &self.g1_prime, &self.g2_prime, &self.g3_prime, &self.g4_prime];

        for (group_index, group) in groups.iter().enumerate() {
            if group.is_empty() {
                let gen_line = gt1_group_line_generator(group_index as u8, None);
                display_table.push_str(&gen_line);
            } else {
                for minterms_and_bin in *group {
                    let gen_line = gt1_group_line_generator(group_index as u8, Some(&minterms_and_bin));
                    display_table.push_str(&gen_line);
                }
            }
            display_table.push_str("---------------------------------\n");
        }

        write!(f, "{}", display_table)
    }
}

fn gt1_group_line_generator(group: u8, minterms: Option<&(String,u8, u8, bool)>) -> String {
    match minterms {
        Some(value) => format!("| G{group} | {} |  {:02} - {:02}  | {} |\n", value.0, value.1, value.2, value.3),
        None => format!("| G{group} | ///// |  // - //  | //// |\n"),
    }
}
