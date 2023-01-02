pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let (first, rest) = word.split_at(1);

        rest.chars().all(|c| c.is_lowercase())
            || first.chars().chain(rest.chars()).all(|c| c.is_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        assert!(Solution::detect_capital_use("USA".to_owned()))
    }

    #[test]
    fn it_works2() {
        assert!(Solution::detect_capital_use("lowercase".to_owned()))
    }

    #[test]
    fn it_works3() {
        assert!(Solution::detect_capital_use("Capital".to_owned()))
    }
}
