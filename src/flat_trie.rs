pub struct Trie {
    offset: Vec<u32>,
    dictionary: String,
    dictionary_lookup: Vec<u32>,
}

impl Trie {
    pub fn new(mut paths: Vec<Vec<&str>>) -> Result<Trie, String> {
        paths.sort();

        let mut dictionary: String = String::new();
        let mut dictionary_lookup: Vec<u32> = vec![0];
        let mut offset: Vec<u32> = vec![0];

        let mut rcu: usize = 0;
        let mut ccu: usize = 0;
        let mut nullctr = 0;

        let dedup_count = |(count, last): (usize, String), n: &&str| {
            if &last != *n {
                (count + 1, (*n).to_owned())
            } else {
                (count, last)
            }
        };

        let unique: usize = paths
            .iter()
            .filter_map(|path| path.get(0))
            .fold((0_usize, "".to_owned()), dedup_count)
            .0;

        let mut next_child_index: usize = unique; //index to the first child of the next node
        offset.push(next_child_index as u32);

        while nullctr < paths.len() - 1 {
            if let Some(cur_path) = &paths.get(rcu) {
                if let Some(cur_node) = cur_path.get(ccu) {
                    nullctr = 0;

                    //count how many paths share the same node
                    let same_count = paths
                        .iter()
                        .skip(rcu)
                        .map(|path| path.get(ccu))
                        .take_while(|node| node == &Some(cur_node))
                        .count();

                    // the index of the next nodes first child is equal to the index of this nodes first child + this nodes number of unique, non-null children
                    next_child_index += paths
                        .iter()
                        .skip(rcu)
                        .take(same_count)
                        .filter_map(|path| path.get(ccu + 1))
                        .fold((0_usize, "".to_owned()), dedup_count)
                        .0;

                    //push the first child of the next node
                    offset.push(next_child_index as u32);

                    dictionary_lookup.push(dictionary.len() as u32);
                    dictionary.push_str(cur_node);

                    //increment past paths sharing this node
                    rcu += same_count;
                } else {
                    nullctr += 1;
                    rcu += 1;
                }

                ccu += rcu / paths.len();
                rcu %= paths.len();
            } else {
                panic!("path out of bounds")
            }
        }

        dictionary_lookup.push(dictionary.len() as u32);
        /*
        a, b, c, d
        a, b, e, f
        a, g, h, i
        o, l, m


        -- |
           |
        |
        |
        |
        --
        */

        Ok(Trie {
            offset,
            dictionary,
            dictionary_lookup,
        })
    }

    #[inline(always)]
    fn get_label(&self, index: u32) -> &str {
        return &self.dictionary[(self.dictionary_lookup[index as usize] as usize)
            ..(self.dictionary_lookup[(index + 1) as usize] as usize)];
    }

    fn recurse_tree_lines(&self, index: usize) -> Result<(Vec<String>, String), String> {
        /*if index + 1 >= self.offset.len() {
            return Err("index out of bounds".to_owned());
        }*/

        let start = self.offset[index];
        let end = self.offset[index + 1];

        if (start == end) {
            return Ok((vec!["".to_owned()], self.get_label(index as u32).to_owned()));
        }

        let mut ahead_results: Vec<(Vec<String>, String)> = Vec::new();
        for i in start..end {
            ahead_results.push(self.recurse_tree_lines((i + 1) as usize)?);
        }

        let this_prefix: String = format!("{} --| ", self.get_label(index as u32));

        let mut lines: Vec<String> = Vec::new();

        let max_prefix = ahead_results
            .iter()
            .map(|r| r.1.chars().count())
            .max()
            .expect("nodes must be labelled");

        let space = (0..max_prefix - 1).map(|_| ' ').collect::<String>() + "| ";

        for result in ahead_results {
            let gap = (0..(max_prefix - result.1.chars().count()))
                .map(|_| ' ')
                .collect::<String>();
            let spaced_prefix = result.1 + &gap;

            lines.push(spaced_prefix + &result.0[0]);

            result
                .0
                .iter()
                .skip(1)
                .for_each(|l| lines.push(space.clone() + &l));

            lines.push("".to_owned());
        }

        return Ok((lines, this_prefix));
    }
}

impl ToString for Trie {
    fn to_string(&self) -> String {
        let lines = self.recurse_tree_lines(0);

        if let Ok(lines) = lines {
            let mut s = String::new();

            lines.0.iter().for_each(|l| {
                s.push_str(l);
                s.push('\n');
            });

            s
        } else {
            "Error generating Trie string representation".to_owned()
        }
    }
}

impl std::fmt::Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
