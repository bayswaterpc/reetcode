/// Solution to [Min Stack](https://leetcode.com/problems/min-stack/)
#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    min_indx_stack: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![],
            min_indx_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.min_indx_stack.is_empty() {
            self.min_indx_stack.push(0);
        } else if val < self.stack[*self.min_indx_stack.last().unwrap()] {
            self.min_indx_stack.push(self.stack.len());
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        self.stack.pop().unwrap();
        if self.stack.len() == *self.min_indx_stack.last().unwrap() {
            self.min_indx_stack.pop();
        }
    }

    pub fn top(&self) -> i32 {
        if self.stack.is_empty() {
            panic!("Cannot call top on empty ")
        }
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        if self.stack.is_empty() {
            panic!("Cannot call top on empty ")
        }
        self.stack[*self.min_indx_stack.last().unwrap()]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

pub mod test {
    #[allow(unused_imports)]
    use super::MinStack;

    pub fn test_min_stack_commands(commands: &[&str], values: &[i32], outputs: &[i32]) {
        if commands[0] != "MinStack" {
            panic!("Must start by initializing")
        }
        let mut min_stack = MinStack::new();

        for ii in 1..commands.len() {
            match commands[ii] {
                "push" => {
                    min_stack.push(values[ii]);
                }
                "pop" => {
                    min_stack.pop();
                }
                "top" => {
                    assert_eq!(outputs[ii], min_stack.top())
                }
                "getMin" => {
                    assert_eq!(outputs[ii], min_stack.get_min())
                }
                _ => panic!("invalid command"),
            }
        }
    }

    #[test]
    fn unit() {
        // using i32::MIN as null value
        let commands = [
            "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
        ];
        let values = [i32::MIN, -2, 0, -3, i32::MIN, i32::MIN, i32::MIN, i32::MIN];
        let outputs = [0, i32::MIN, i32::MIN, i32::MIN, -3, i32::MIN, 0, -2];
        test_min_stack_commands(&commands, &values, &outputs);
    }
}
