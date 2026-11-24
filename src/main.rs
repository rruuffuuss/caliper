mod flat_trie;

use flat_trie::Trie;

fn main() {
    println!("Hello, world!");

    let path1 = vec!["hello", "test1"];
    let path2 = vec!["hello", "test2", "leaf1"];
    let path3 = vec!["greeting", "test3"];

    let paths = vec![path1, path2, path3];

    let trie = Trie::new(paths).unwrap();

    print!("{:?}", trie);
}
