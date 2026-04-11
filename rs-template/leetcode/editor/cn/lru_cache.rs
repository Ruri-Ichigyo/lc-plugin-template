/*
 * @lc app=leetcode.cn id=146 lang=rust
 * @lcpr version=30203
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,   // key -> val
    order: std::collections::VecDeque<i32>, // front = most recent
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: std::collections::VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            // 移到最近使用位置
            self.order.retain(|&k| k != key);
            self.order.push_front(key);
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.order.retain(|&k| k != key);
        } else if self.map.len() == self.capacity {
            // 淘汰最久未使用的 key
            if let Some(lru_key) = self.order.pop_back() {
                self.map.remove(&lru_key);
            }
        }
        self.map.insert(key, value);
        self.order.push_front(key);
    }
}
// @lc code=end

fn main() {
    // your test code here
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{}", cache.get(1));  // 1
    cache.put(3, 3);               // 淘汰 key=2
    println!("{}", cache.get(2));  // -1
    cache.put(4, 4);               // 淘汰 key=1
    println!("{}", cache.get(1));  // -1
    println!("{}", cache.get(3));  // 3
    println!("{}", cache.get(4));  // 4
}

/*
// @lcpr case=start
// ["LRUCache","put","put","get","put","get","put","get","get","get"]\n[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]\n
// @lcpr case=end

 */
