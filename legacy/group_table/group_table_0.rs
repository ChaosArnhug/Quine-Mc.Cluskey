#[derive(Default)]
pub struct GroupTable0 {
    pub g0: Vec<(u8,bool)>,
    pub g1: Vec<(u8,bool)>,
    pub g2: Vec<(u8,bool)>,
    pub g3: Vec<(u8,bool)>,
    pub g4: Vec<(u8,bool)>,
    pub g5: Vec<(u8,bool)>
}

impl GroupTable0 {
    pub fn new(minterms: &[u8]) -> GroupTable0{
        let mut group_table: GroupTable0 = GroupTable0::default();
        for minterm in minterms {
            group_table.add_minterm_to_group(minterm)
        }

        return group_table;
    }

    pub fn add_minterm_to_group(&mut self, minterm: &u8) {
        match minterm {
            0 => self.g0.push((*minterm, false)),
            1 | 2 | 4 | 8 | 16 => self.g1.push((*minterm, false)),
            3 | 5 | 6 | 9 |10 | 12 | 17 | 18 | 20 | 24 => self.g2.push((*minterm, false)),
            7 | 11 | 13 | 14 | 19 | 21 | 22 | 25 | 26 | 28 => self.g3.push((*minterm, false)),
            15 | 23 | 27 | 29 | 30 => self.g4.push((*minterm, false)),
            31 => self.g5.push((*minterm, false)),
            _ => {}
        }
    }
}

impl std::fmt::Display for GroupTable0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_table = String::new();
        display_table.push_str("Group Table 0\n===========================\n\n| Gi |  bin  | dec | Used |\n---------------------------\n");

        let groups = [&self.g0, &self.g1, &self.g2, &self.g3, &self.g4, &self.g5];

        for (group_index, group) in groups.iter().enumerate() {
            if group.is_empty() {
                let gen_line = gt0_group_line_generator(group_index as u8, None);
                display_table.push_str(&gen_line);
            } else {
                for minterm in *group {
                    let gen_line = gt0_group_line_generator(group_index as u8, Some(minterm));
                    display_table.push_str(&gen_line);
                }
            }
            display_table.push_str("---------------------------\n");
        }

        write!(f, "{}", display_table)
    }
}

fn gt0_group_line_generator(group: u8, minterm: Option<&(u8,bool)>) -> String {
    match minterm {
        Some(value) => format!("| G{group} | {:05b} |  {:02} | {} |\n", value.0, value.0, value.1),
        None => format!("| G{group} | ///// |  // | //// |\n"),
    }
}
