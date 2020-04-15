use std::str::FromStr;

const DIV: &str = "/";
const SUM: &str = "+";
const MUL: &str = "*";
const SUB: &str = "-";
const ACTIONS: [&str; 4] = [SUM, DIV, MUL, SUB];

pub struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    pub fn new() -> Stack {
        let stack: Vec<i32> = Vec::new();
        Stack { stack }
    }

    fn calculate(&mut self, value: &str) {
        let first = self.stack.pop().unwrap();
        let second = self.stack.pop().unwrap();
        let result = match value {
            DIV => (second / first),
            SUM => (second + first),
            MUL => (second * first),
            SUB => (second - first),
            _ => 0
        };
        self.stack.push(result);
        println!("{} {} {} = {} \n {:?}", first, value, second, result, self.stack);
    }

    fn insert(&mut self, value: &str) {
        if ACTIONS.contains(&value) {
            self.calculate(&value);
            return;
        }
        let number = FromStr::from_str(value).unwrap();
        self.stack.push(number);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }
}

pub fn exec(arr: Vec<&str>) -> i32 {
    let mut stack = Stack::new();

    for a in arr {
        stack.insert(a);
    }

    stack.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr: Vec<&str> = vec!["4", "2", "/"];
        assert_eq!(exec(arr), 2);
    }

    #[test]
    fn supreme() {
        let arr: Vec<&str> = vec!["15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"];
        assert_eq!(exec(arr), 5);
    }
}
