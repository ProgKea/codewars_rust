#![allow(dead_code)]

pub struct Solution;
use rand::Rng;
use std::collections::BTreeMap;

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

    fn printer_error(s: &str) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut errors = 0;

        for char in s.chars() {
            if alphabet.find(char.to_ascii_lowercase()).unwrap() > 12 {
                errors += 1;
            }
        }

        return format!("{}/{}", errors, s.len());
    }

    fn expanded_form(n: u64) -> String {
        let ohs = n.to_string().len() - 1;
        let mut solution = String::new();

        for (i, num) in n.to_string().chars().enumerate() {
            if num != '0' {
                solution.push_str(&(num.to_string() + &"0".repeat(ohs - i) + " + "));
            }
        }

        return solution.split_at(solution.len() - 3).0.to_string();
    }

    fn spin_words(words: &str) -> String {
        let mut solution = String::new();

        for word in words.split_whitespace() {
            if word.len() >= 5 {
                solution.push_str(&word.chars().rev().collect::<String>());
                solution.push(' ');
            } else {
                solution.push_str(word);
                solution.push(' ');
            }
        }

        solution.pop();
        return solution;
    }

    fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
        let mut a_squared: Vec<i64> = a.iter().map(|x| x * x).collect();
        let mut b_sorted: Vec<i64> = b.clone();

        a_squared.sort();
        b_sorted.sort();

        return a_squared == b_sorted;
    }

    fn break_camelcase(s: &str) -> String {
        let mut solution = String::new();

        for char in s.chars() {
            if char.is_uppercase() {
                solution.push(' ');
            }
            solution.push(char);
        }

        return solution;
    }

    fn pyramid(n: usize) -> Vec<Vec<u32>> {
        if n == 0 {
            return vec![];
        }
        let mut solution: Vec<Vec<u32>> = vec![vec![1]];

        for _ in 0..n - 1 {
            let mut new_vec = solution.last().unwrap().to_vec();
            new_vec.push(1);
            solution.push(new_vec);
        }

        return solution;
    }

    fn dead_fish(code: &str) -> Vec<i32> {
        let mut solution: Vec<i32> = Vec::new();
        let mut n = 0;

        for ch in code.chars() {
            match ch {
                'i' => n += 1,
                'd' => n -= 1,
                's' => n *= n,
                'o' => solution.push(n),
                _ => {}
            };
        }

        return solution;
    }

    fn digital_root(n: i64) -> i64 {
        let mut solution: i64 = n;

        while solution >= 10 {
            solution = solution.to_string().chars().fold(0 as i64, |acc, x| {
                acc + x.to_string().parse::<i64>().unwrap()
            });
        }

        return solution;
    }

    fn parts_sums(ls: &[u64]) -> Vec<u64> {
        if ls.is_empty() {
            return vec![0];
        }
        let mut ls1 = ls.to_vec();
        let mut solution: Vec<u64> = Vec::new();

        while ls1.len() != 0 {
            solution.push(ls1.iter().fold(0 as u64, |a, x| a + x));
            ls1.remove(0);
        }

        solution.push(0);
        return solution;
    }

    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for a in 2..n {
            if n % a == 0 {
                return false;
            }
        }

        return true;
    }

    fn find_smallest_prime(n: i64) -> i64 {
        let mut smallest_prime = 2;

        while n % smallest_prime != 0 && !Self::is_prime(n) {
            smallest_prime += 1;
        }

        return smallest_prime;
    }

    fn prime_factors(n: i64) -> String {
        if Self::is_prime(n) {
            return "(".to_string() + &n.to_string() + &")".to_string();
        }

        let mut primes: BTreeMap<i64, u64> = BTreeMap::new();
        let mut n1 = n;
        let mut smallest_prime = Self::find_smallest_prime(n1);
        let mut formula = String::new();
        primes.insert(smallest_prime, 1);
        dbg!(n);

        while !Self::is_prime(n1 / smallest_prime) {
            n1 = n1 / smallest_prime;
            smallest_prime = Self::find_smallest_prime(n1);
            primes
                .entry(smallest_prime)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        primes
            .entry(n1 / smallest_prime)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        for prime in primes {
            formula.push('(');
            if prime.1 == 1 {
                formula.push_str(&prime.0.to_string());
            } else {
                formula.push_str(&prime.0.to_string());
                formula.push_str("**");
                formula.push_str(&prime.1.to_string());
            }
            formula.push(')');
        }

        return formula;
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

    #[test]
    fn test_printer_error() {
        assert_eq!(
            &Solution::printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &Solution::printer_error(
                "kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"
            ),
            "6/60"
        );
        assert_eq!(
            &Solution::printer_error(
                "kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"
            ),
            "11/65"
        );
    }

    #[test]
    fn test_expanded_form() {
        assert_eq!(Solution::expanded_form(12), "10 + 2");
        assert_eq!(Solution::expanded_form(42), "40 + 2");
        assert_eq!(Solution::expanded_form(70304), "70000 + 300 + 4");
    }

    #[test]
    fn test_spin_words() {
        assert_eq!(Solution::spin_words("Welcome"), "emocleW");
        assert_eq!(
            Solution::spin_words("Hey fellow warriors"),
            "Hey wollef sroirraw"
        );
        assert_eq!(Solution::spin_words("This is a test"), "This is a test");
        assert_eq!(
            Solution::spin_words("This is another test"),
            "This is rehtona test"
        );
        assert_eq!(
            Solution::spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            Solution::spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            Solution::spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }

    fn testing_comp(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(Solution::comp(a, b), exp)
    }

    #[test]
    fn test_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 11,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing_comp(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 21,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing_comp(a1, a2, false);
    }

    #[test]
    fn test_break_camelcase() {
        assert_eq!(Solution::break_camelcase("camelCasing"), "camel Casing");
        assert_eq!(
            Solution::break_camelcase("camelCasingTest"),
            "camel Casing Test"
        );
    }

    #[test]
    fn test_pyramid() {
        assert_eq!(Solution::pyramid(0), vec![] as Vec<Vec<u32>>);
        assert_eq!(Solution::pyramid(1), vec![vec![1]]);
        assert_eq!(Solution::pyramid(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(
            Solution::pyramid(3),
            vec![vec![1], vec![1, 1], vec![1, 1, 1]]
        );
    }

    #[test]
    fn test_deadfish() {
        assert_eq!(Solution::dead_fish("iiisdoso"), vec![8, 64]);
        assert_eq!(Solution::dead_fish("iiisdosodddddiso"), vec![8, 64, 3600]);
    }

    #[test]
    fn test_digital_root() {
        assert_eq!(Solution::digital_root(16), 7);
    }

    fn dotest(ls: Vec<u64>, expect: Vec<u64>) {
        let actual = Solution::parts_sums(&ls);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parts_sums() {
        dotest(vec![], vec![0]);
        dotest(vec![0, 1, 3, 6, 10], vec![20, 20, 19, 16, 10, 0]);
        dotest(vec![1, 2, 3, 4, 5, 6], vec![21, 20, 18, 15, 11, 6, 0]);
        dotest(
            vec![
                744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358,
            ],
            vec![
                10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057,
                2580168, 2579358, 0,
            ],
        );
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(Solution::prime_factors(75), "(3)(5**2)");
        assert_eq!(
            Solution::prime_factors(7775460),
            "(2**2)(3**3)(5)(7)(11**2)(17)"
        );
        assert_eq!(
            Solution::prime_factors(17 * 17 * 93 * 677),
            "(3)(17**2)(31)(677)"
        );
    }
}

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=9000000);
    println!("{}", Solution::prime_factors(random_number));
}
