/// time=O(M*N)
/// space=O(M*N)
/// where M & N are length of strings
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut text1: Vec<char> = text1.chars().collect();
    let mut text2: Vec<char> = text2.chars().collect();
    if text1.len() < text2.len() {
        std::mem::swap(&mut text1, &mut text2);
    }

    let mut previous = vec![0; text1.len()+1];
    let mut current = vec![0; text1.len()+1];

    for col in (0..text2.len()).rev(){
        for row in (0..text1.len()).rev(){
            if text2[col] == text1[row] {
                current[row] = 1 + previous[row + 1];
            } else {
                current[row] = previous[row].max(current[row+1])
            }
        }
        std::mem::swap(&mut current, &mut previous);
    }
    return previous[0];
}


pub mod test {
    #[test]
    fn unit() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let output = 3;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        let output = 3;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        let output = 0;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "bl".to_string();
        let text2 = "yby".to_string();
        let output = 1;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

    }
}
