use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_insert(Vec::new()).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(ret) = self.store.get(&key) {
            match ret.binary_search_by(|k| k.0.cmp(&timestamp)) {
                Ok(ans) => ret[ans].1.clone(),
                Err(x) => if x == 0 {
                    String::new()
                } else {
                    ret[x - 1].1.clone()
                }
            }
        } else {
            String::new()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_981() {
        let mut map = TimeMap::new();
        map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(map.get("foo".to_string(), 3), "bar".to_string());
    }
}