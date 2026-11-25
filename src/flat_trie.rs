use std::{collections::VecDeque, ops::Range};

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

        //processing queue for nodes in the trie
        let mut children: VecDeque<(usize, Range<usize>)> = VecDeque::new();

        //push the root node to the queue
        children.push_back((0, (0..paths.len())));
        let mut total_children: usize = 0;

        while !children.is_empty() {
            let (child_depth, child_range) = children.pop_front().unwrap();

            let mut last = None;
            let mut contig = 0;
            for (i, child) in (child_range.start..=child_range.end).zip(
                paths[child_range]
                    .iter()
                    .map(|path| path.get(child_depth).copied())
                    .chain(std::iter::once(None)),
            ) {
                if child == last {
                    contig += 1;
                } else {
                    if let Some(last_str) = last {
                        dictionary_lookup.push(dictionary.len() as u32);
                        dictionary.push_str(last_str);

                        //it is okay to push the path based ranged (which includes nulls and duplicates) since we build the try based on total_children, which ignores nulls and is deduplicated
                        children.push_back(((child_depth + 1), ((i - contig)..i)));
                        total_children += 1;
                    }

                    contig = 1;
                    last = child;
                }
            }

            // we only push a new node once we have finished processing its children (and hence know the next offset)
            offset.push(total_children as u32);
        }

        dictionary_lookup.push(dictionary.len() as u32);

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
        let start = self.offset[index];
        let end = self.offset[index + 1];

        if start == end {
            return Ok((vec![], self.get_label(index as u32).to_owned()));
        }

        let mut ahead_results: Vec<(Vec<String>, String)> = Vec::new();
        for i in start..end {
            ahead_results.push(self.recurse_tree_lines((i + 1) as usize)?);
        }

        let this_prefix: String = format!("{}", self.get_label(index as u32));

        let mut lines: Vec<String> = Vec::new();

        let max_prefix = ahead_results
            .iter()
            .map(|r| r.1.chars().count())
            .max()
            .expect("nodes must be labelled")
            + 3; //so that every node has at least -|

        let space = (0..max_prefix - 1).map(|_| ' ').collect::<String>() + "| ";

        for result in ahead_results {
            if result.0.is_empty() {
                lines.push(result.1);
            } else {
                let gap = (0..(max_prefix - result.1.chars().count() - 2))
                    .map(|_| '-')
                    .collect::<String>();
                let spaced_prefix = result.1 + &gap + "-| ";

                lines.push(spaced_prefix + &result.0[0]);
            }

            result
                .0
                .iter()
                .skip(1)
                .for_each(|l| lines.push(space.clone() + &l));

            lines.push("".to_owned());
        }

        lines.pop();

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
