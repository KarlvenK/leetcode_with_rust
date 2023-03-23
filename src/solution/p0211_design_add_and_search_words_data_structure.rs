struct WordDictionary {
    head: TreeNode,
}

struct TreeNode {
    is_end: bool,
    children: Vec<Option<TreeNode>>,
}

impl TreeNode {
    fn new(is_end: bool) -> Self {
        let mut ret = TreeNode {
            is_end,
            children: Vec::with_capacity(26),
        };
        for _ in 0..26 {
            ret.children.push(None);
        }
        return ret;
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        return WordDictionary {
            head: TreeNode::new(false),
        };
    }

    fn add_word(&mut self, word: String) {
        fn dfs(node: &mut TreeNode, s: &[u8], idx: usize) {
            if idx == s.len() {
                node.is_end = true;
                return;
            }
            let loc = s[idx] as usize - b'a' as usize;

            let n = if let Some(x) = node.children[loc].as_mut() {
                x
            } else {
                node.children[loc] = Some(TreeNode::new(false));
                node.children[loc].as_mut().unwrap()
            };
            dfs(n, s, idx + 1);
        }
        dfs(&mut self.head, word.as_bytes(), 0)
    }

    fn search(&mut self, word: String) -> bool {
        fn dfs(node: &TreeNode, s: &[u8], idx: usize) -> bool {
            if idx == s.len() {
                return node.is_end;
            }
            if s[idx] == b'.' {
                for son in node.children.iter() {
                    if let Some(x) = son.as_ref() {
                        if dfs(x, s, idx + 1) {
                            return true;
                        }
                    }
                }
                return false;
            } else {
                let loc = s[idx] as usize - b'a' as usize;
                if let Some(x) = node.children[loc].as_ref() {
                    return dfs(x, s, idx + 1);
                } else {
                    return false;
                }
            }
        }
        dfs(&mut self.head, word.as_bytes(), 0)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_211() {}
}
