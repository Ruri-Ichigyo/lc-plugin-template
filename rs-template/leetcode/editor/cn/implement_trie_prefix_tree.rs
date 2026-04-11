/*
 * @lc app=leetcode.cn id=208 lang=rust
 * @lcpr version=30203
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find_node(word.as_bytes())
            .map(|n| n.is_end)
            .unwrap_or(false)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find_node(prefix.as_bytes()).is_some()
    }

    fn find_node(&self, bytes: &[u8]) -> Option<&Trie> {
        let mut node = self;
        for &c in bytes {
            let idx = (c - b'a') as usize;
            node = node.children[idx].as_deref()?;
        }
        Some(node)
    }
}
// @lc code=end

fn main() {
    // your test code here
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    println!("{}", trie.search("apple".to_string()));       // true
    println!("{}", trie.search("app".to_string()));         // false
    println!("{}", trie.starts_with("app".to_string()));    // true
    trie.insert("app".to_string());
    println!("{}", trie.search("app".to_string()));         // true
}

/*
// @lcpr case=start
// ["Trie","insert","search","search","startsWith","insert","search"]\n[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]\n
// @lcpr case=end

 */
