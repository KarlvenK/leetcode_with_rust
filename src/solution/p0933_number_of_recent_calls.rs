pub struct RecentCounter {
    calls_cnt: i32,
    calls: Queue<Node>,
}

struct Node {
    time_stamp: i32,
    idx: i32,
}

struct Queue<T> {
    que: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            que: Vec::<T>::new(),
        }
    }
    fn pop_front(&mut self) -> T {
        self.que.remove(0)
    }
    fn push(&mut self, v: T) {
        self.que.push(v);
    }
    fn front(&self) -> &T {
        self.que.first().unwrap()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            calls_cnt: 0,
            calls: Queue::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let new_node = Node {
            time_stamp: t,
            idx: self.calls_cnt,
        };
        self.calls.push(new_node);
        self.calls_cnt += 1;
        let mut left = t - 3000;
        if left < 0 {
            left = 0;
        }
        while self.calls.front().time_stamp < left {
            self.calls.pop_front();
        }
        self.calls_cnt - self.calls.front().idx
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = RecentCounter::new();
        let ret = obj.ping(1);
        assert_eq!(1, ret);
        let ret = obj.ping(100);
        assert_eq!(2, ret);
        let ret = obj.ping(3001);
        assert_eq!(3, ret);
        let ret = obj.ping(3002);
        assert_eq!(3, ret);
    }
}
