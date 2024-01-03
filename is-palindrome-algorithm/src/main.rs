struct Solution{}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s1 = s.to_lowercase();
        let iter = s1.chars().filter(
            |c| c.is_ascii_alphanumeric()
        );
        iter.clone().eq(iter.rev())
    }
}

fn main() {
    Solution::is_palindrome("anna".to_string());
}
