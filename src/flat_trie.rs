struct Trie {
    offset: Vec<u32>,
    dictionary: String,
    dictionary_lookup: Vec<u32>,
}

impl Trie {
    pub fn new(mut paths: Vec<Vec<String>>) -> Result<Trie, String> {
        paths.sort();

        let mut dictionary: String = String::new();
        let mut dictionary_lookup: Vec<usize> = vec![0];
        let mut offset: Vec<usize> = vec![0, 1];

        let mut rcu: usize = 0;
        let mut ccu: usize = 0;
        let mut nullctr = 0;

        let mut next_i: usize = 0;

        while nullctr < paths.len() - 1 {
            if let (Some(cur_path)) = &paths.get(rcu) {
                if let Some(cur_node) = cur_path.get(ccu) {
                    nullctr = 0;

                    //count how many paths share the same node
                    let same_count = paths
                        .iter()
                        .skip(rcu)
                        .map(|path| path.get(ccu))
                        .take_while(|node| node == &Some(cur_node))
                        .count();

                    //using some rolling null counter/dynamic programming would be much faster here, just haven't wrapped my head around it yet
                    //count how many null values occur in this nodes siblings before its first child
                    let mut null_count = paths
                        .iter()
                        .skip(rcu + same_count)
                        .filter_map(|path| path.get(ccu))
                        .count();
                    //count how many null values occur before this nodes first child in the childs siblings
                    null_count += paths
                        .iter()
                        .take(rcu + 1)
                        .filter_map(|path| path.get(ccu))
                        .count();

                    //this nodes first child will occur (total number of paths - number of null nodes - number of paths using this node) after this node
                    offset.push(rcu + paths.len() - null_count - same_count);

                    dictionary_lookup.push(dictionary.len());
                    dictionary.push_str(cur_node);

                    //increment past paths sharing this node
                    rcu += same_count % paths.len();
                    ccu += same_count / paths.len();
                } else {
                    nullctr += 1;
                }
            } else {
                nullctr += 1;
            }
        }
        /*
        a, b, c, d
        a, b, e, f
        a, g, h, i
        o, l, m
        */

        Err("test".to_owned())
    }
}
