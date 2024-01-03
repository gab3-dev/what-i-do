impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let sl= s.to_lowercase();
        let iter = s1.chars().filter(
            |c| c.is_ascii_alphanumeric()
        );
        iter.clone().eq(iter.rev())
    }
}
