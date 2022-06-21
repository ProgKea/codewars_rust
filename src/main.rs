#![allow(dead_code)]

pub struct Solution;

impl Solution {
    fn gimme(input_array: [i32; 3]) -> usize {
        let mut sorted_array = input_array.clone();
        sorted_array.sort();
        let target = sorted_array[1];

        for (i, value) in input_array.iter().enumerate() {
            if value == &target {
                return i;
            }
        }

        return 1;
    }

    fn give_me_a_diamond(n: i32) -> Option<String> {
        if n < 0 || n % 2 == 0 {
            return None;
        }

        let n = n as usize;
        let diamond = (1..=n)
            .chain((1..n).rev())
            .step_by(2)
            .map(|x| format!("{}{}\n", " ".repeat((n - x) / 2), "*".repeat(x)))
            .collect();

        return Some(diamond);
    }

    fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
        let mut solution: Vec<String> = Vec::new();

        for a in arr_a {
            for b in arr_b {
                if b.contains(a) && !solution.contains(&a.to_string()) {
                    solution.push(a.to_string());
                }
            }
        }

        solution.sort();
        return solution;
    }

    fn round_to_next_5(n: i32) -> i32 {
        n + (5 - n % 5) % 5
    }

    fn accum(s: &str) -> String {
        let mut solution = String::new();

        for (i, char) in s.chars().enumerate() {
            solution.push_str(&char.to_uppercase().to_string());
            solution.push_str(&char.to_string().repeat(i).to_lowercase());
            solution.push('-');
        }
        solution.pop();

        return solution;
    }

    fn alphabet_position(text: &str) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut solution = String::new();

        for char in text.chars() {
            if char.is_alphabetic() {
                solution.push_str(
                    &alphabet
                        .find(char.to_ascii_lowercase())
                        .unwrap()
                        .checked_add(1)
                        .unwrap()
                        .to_string(),
                );
                solution.push(' ');
            }
        }
        solution.pop();

        return solution;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(Solution::gimme([2, 3, 1]), 0);
        assert_eq!(Solution::gimme([-2, -3, -1]), 0);
        assert_eq!(Solution::gimme([5, 10, 14]), 1);
    }

    #[test]
    fn test_give_me_a_diamond() {
        assert_eq!(
            Solution::give_me_a_diamond(3),
            Some(" *\n***\n *\n".to_string())
        );
        assert_eq!(
            Solution::give_me_a_diamond(5),
            Some("  *\n ***\n*****\n ***\n  *\n".to_string())
        );
        assert_eq!(Solution::give_me_a_diamond(-3), None);
        assert_eq!(Solution::give_me_a_diamond(2), None);
        assert_eq!(Solution::give_me_a_diamond(0), None);
        assert_eq!(Solution::give_me_a_diamond(1), Some("*\n".to_string()));
    }

    #[test]
    fn test_in_array() {
        assert_eq!(
            Solution::in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            Solution::in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            Solution::in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            Solution::in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }

    #[test]
    fn test_round_to_next_5() {
        let tests = [
            [0, 0],
            [1, 5],
            [-1, 0],
            [-5, -5],
            [3, 5],
            [5, 5],
            [7, 10],
            [20, 20],
            [39, 40],
            [990, 990],
            [121, 125],
            [555, 555],
        ];

        for [x, out] in tests.iter() {
            assert_eq!(Solution::round_to_next_5(*x), *out);
        }
    }

    #[test]
    fn basic_tests() {
        assert_eq!(
            Solution::accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            Solution::accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            Solution::accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            Solution::accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            Solution::accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }

    #[test]
    fn test_alphabet_position() {
        assert_eq!(
            Solution::alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
        );
        assert_eq!(
            Solution::alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
        );
    }
}

fn main() {}
