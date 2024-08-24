use std::collections::HashSet;

use super::group_table_2::GroupTable2;
use super::{generate_combined_bin_value_wildcards, is_hm_greater_1_wildcards};

#[derive(Default)]
pub struct GroupTable3 {
    pub g0_prime_3: Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)>,
    pub g1_prime_3: Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)>,
    pub g2_prime_3: Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)>,
}

impl GroupTable3 {
    pub fn new(group_table_2: &mut GroupTable2) -> GroupTable3 {
        let mut group_table: GroupTable3 = GroupTable3::default();

        group_table.g0_prime_3 = group_table.compare_groups(&mut group_table_2.g0_prime_2, &mut group_table_2.g1_prime_2);
        group_table.g1_prime_3 = group_table.compare_groups(&mut group_table_2.g1_prime_2, &mut group_table_2.g2_prime_2);
        group_table.g2_prime_3 = group_table.compare_groups(&mut group_table_2.g2_prime_2, &mut group_table_2.g3_prime_2);

        GroupTable3::remove_duplicate_tuples(&mut group_table.g0_prime_3);
        GroupTable3::remove_duplicate_tuples(&mut group_table.g1_prime_3);
        GroupTable3::remove_duplicate_tuples(&mut group_table.g2_prime_3);

        return group_table;
    }

    pub fn compare_groups(&mut self, gt1_vec1: &mut Vec<(String,u8,u8,u8,u8,bool)>, gt1_vec2: &mut Vec<(String,u8,u8,u8,u8,bool)>) -> Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)> {
        let mut buffer: Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)> = Vec::new();
        for term_v1 in gt1_vec1 {
            for term_v2 in &mut *gt1_vec2 {
                if !is_hm_greater_1_wildcards(&term_v1.0, &term_v2.0) {
                    term_v1.5 = true;
                    term_v2.5 = true;
                    buffer.push((generate_combined_bin_value_wildcards(&term_v1.0, &term_v2.0), term_v1.1, term_v1.2,term_v1.3, term_v1.4, term_v2.1, term_v2.2, term_v2.3, term_v2.4, false));
                }
            }
        }
        
        return buffer;
    }

    pub fn remove_duplicate_tuples(tuples: &mut Vec<(String,u8,u8,u8,u8,u8,u8,u8,u8,bool)>){
        let mut seen = HashSet::new();

        tuples.retain(|(first, _, _, _, _, _, _, _, _, _)| seen.insert(first.clone()));

    }
}

impl std::fmt::Display for GroupTable3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_table = String::new();
        display_table.push_str("Group Table 3\n=====================================================================\n\n| Gi |  bin  | dec - dec - dec - dec - dec - dec - dec - dec | Used |\n---------------------------------------------------------------------\n");

        let groups = [&self.g0_prime_3, &self.g1_prime_3, &self.g2_prime_3];

        for (group_index, group) in groups.iter().enumerate() {
            if group.is_empty() {
                let gen_line = gt3_group_line_generator(group_index as u8, None);
                display_table.push_str(&gen_line);
            } else {
                for minterms_and_bin in *group {
                    let gen_line = gt3_group_line_generator(group_index as u8, Some(&minterms_and_bin));
                    display_table.push_str(&gen_line);
                }
            }
            display_table.push_str("---------------------------------------------------------------------\n");
        }

        write!(f, "{}", display_table)
    }
}

fn gt3_group_line_generator(group: u8, minterms: Option<&(String,u8, u8, u8, u8, u8, u8, u8, u8, bool)>) -> String {
    match minterms {
        Some(value) => format!("| G{group} | {} |  {:02} - {:02}  -  {:02} - {:02}  -  {:02} - {:02}  -  {:02} - {:02}  | {} |\n", value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9),
        None => format!("| G{group} | ///// |  // - //  -  // - //  -  // - //  -  // - //  | //// |\n"),
    }
}