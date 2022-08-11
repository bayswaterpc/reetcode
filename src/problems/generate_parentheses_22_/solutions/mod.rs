fn backtrack(permutations: &mut Vec<String>, stack: &mut String, nl: i32, nr: i32, n: i32) {
    if stack.len() == (n * 2) as usize {
        permutations.push(stack.clone());
        return;
    }
    if nl < n {
        stack.push('(');
        backtrack(permutations, stack, nl + 1, nr, n);
        stack.pop();
    }
    if nr < nl {
        stack.push(')');
        backtrack(permutations, stack, nl, nr + 1, n);
        stack.pop();
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut permutations = vec![];
    let mut stack = "".to_string();

    backtrack(&mut permutations, &mut stack, 0, 0, n);

    permutations
}
pub mod test {
    #[allow(unused_imports)]
    use crate::problems::generate_parentheses_22_::test::do_unit;

    #[test]
    fn unit() {
        do_unit(
            3,
            ["((()))", "(()())", "(())()", "()(())", "()()()"],
            super::generate_parenthesis,
        );
        do_unit(1, ["()"], super::generate_parenthesis);
    }
}
