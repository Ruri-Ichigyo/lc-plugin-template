/*
 * @lc app=leetcode.cn id=146 lang=rust
 * @lcpr version=30203
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::collections::{BTreeMap, HashMap};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, (i32, u64)>, // key -> (val, 访问时间戳)
    order: BTreeMap<u64, i32>,     // 时间戳 -> key（用于找最久未用的 key）
    time: u64,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: BTreeMap::new(),
            time: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&(val, old_ts)) = self.map.get(&key) {
            self.time += 1;
            self.order.remove(&old_ts);
            self.order.insert(self.time, key);
            self.map.insert(key, (val, self.time));
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // 若 key 已存在，先删除旧时间戳
        if let Some(&(_, old_ts)) = self.map.get(&key) {
            self.order.remove(&old_ts);
        } else if self.map.len() == self.capacity {
            // 淘汰最久未使用的 key
            if let Some((&lru_ts, &lru_key)) = self.order.iter().next() {
                self.order.remove(&lru_ts);
                self.map.remove(&lru_key);
            }
        }
        self.time += 1;
        self.map.insert(key, (value, self.time));
        self.order.insert(self.time, key);
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
