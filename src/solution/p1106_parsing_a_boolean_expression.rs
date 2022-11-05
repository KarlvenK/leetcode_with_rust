use std::vec;

pub struct Solution{}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut queue: Vec<char> = vec![];
        for ch in expression.chars() {
            if ch == ')' {
                let mut list = vec![];
                while queue[queue.len() - 1] != '(' {
                    list.push(queue.pop().unwrap());
                }
                queue.pop();
                match queue.pop() {
                    Some(sign) => {
                        match sign {
                            '&' => queue.push(if list.contains(&'f') {'f'} else {'t'}),
                            '|' => queue.push(if list.contains(&'t') {'t'} else {'f'}),
                            '!' => queue.push(if list.contains(&'f') {'t'} else {'f'}),
                            _ => {}
                        }
                    }
                    None => {}
                }
            } else if ch != ',' {queue.push(ch)}
        }
        queue[queue.len() - 1] == 't'
    }
}