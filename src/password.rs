pub struct PwGen {
    seed: String,
}

impl PwGen {
    pub fn new(seed: &str) -> PwGen {
        PwGen { seed: seed.to_string() }
    }

    fn s_to_v(s: &str) -> Vec<u32> {
        let mut v = Vec::new();
        for c in s.chars() {
            v.push(c as u32 - ('a' as u32));
        }
        return v;
    }

    fn v_to_s(v: &Vec<u32>) -> String {
        let mut s: String = String::new();

        for i in v {
            s.push(std::char::from_u32(i+('a' as u32)).unwrap());
        }

        s
    }

    fn next(n: u32) -> u32 {
        (n + 1) % 26
    }

    fn is_valid(pw_vec: &Vec<u32>) -> bool {
        let mut runs = 0;
        let mut pairs = 0;
        let mut potential_pair = true;
        let mut valid_chars = true;

        for n in 0..pw_vec.len() {
            // check for invalid chars: i, o, l  (8, 14, 11)
            if [8, 14, 11].contains(&pw_vec[n])  {
                valid_chars = false;
                break;
            }

            // check for pairs
            if (n > 0) && potential_pair {
                if pw_vec[n] == pw_vec[n - 1] {
                    pairs += 1;
                    potential_pair = false;
                }
            }
            else {
                // This can be the first char for next pair.
                potential_pair = true;
            }

            // check for runs
            if n > 1 {
                if pw_vec[n] == pw_vec[n-1]+1 &&
                    pw_vec[n-1] == pw_vec[n-2]+1 {
                    runs += 1;
                }
            }
        }

        // println!("{:?}: valid_chars:{}, pairs:{}, runs:{}", pw_vec, valid_chars, pairs, runs);

        return valid_chars && (pairs >= 2) && (runs >= 1);
    }

    fn next_pw(pw: &mut Vec<u32>, debug: bool) {
        let mut valid = false;
        while !valid {
            let mut place: usize = pw.len();
            let mut n = 0;

            while (n == 0) && (place > 0) {
                place -= 1;
                n = PwGen::next(pw[place]);
                pw[place] = n;
            }

            if debug {
                println!("next: {}", PwGen::v_to_s(pw));
            }

            valid = PwGen::is_valid(pw);
        }
    }
}

impl super::Day for PwGen {
    fn part1(&self) -> Result<i64, &str> {
        let mut v = PwGen::s_to_v(&self.seed);
        PwGen::next_pw(&mut v, false);
        println!("Next password: {}", PwGen::v_to_s(&v));
        return Ok(0 as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut v = PwGen::s_to_v(&self.seed);
        PwGen::next_pw(&mut v, false);
        println!("next password: {}", PwGen::v_to_s(&v));
        PwGen::next_pw(&mut v, false);
        println!("Next next password: {}", PwGen::v_to_s(&v));
        return Ok(0 as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_is_valid() {
        assert!(!PwGen::is_valid(&PwGen::s_to_v("hijklmmn")));
        assert!(!PwGen::is_valid(&PwGen::s_to_v("abbceffg")));
        assert!(!PwGen::is_valid(&PwGen::s_to_v("abbcegjk")));
        assert!(PwGen::is_valid(&PwGen::s_to_v("abcdffaa")));
        assert!(PwGen::is_valid(&PwGen::s_to_v("ghjaabcc")));
    }

    #[test]
    fn test_next_password() {
        let mut v = PwGen::s_to_v("abcdefgh");
        PwGen::next_pw(&mut v, false);
        assert_eq!("abcdffaa", PwGen::v_to_s(&v));

        let mut v = PwGen::s_to_v("ghijklmn");
        PwGen::next_pw(&mut v, false);
        assert_eq!("ghjaabcc", PwGen::v_to_s(&v));
    }

    #[test]
    fn test_part1() {
        // prints answer: cqjxxyzz
        let day = PwGen::new("cqjxjnds");
        assert_eq!(day.part1(), Ok(0));
    }

    #[test]
    fn test_part2() {
        // prints answer: cqkaabcc
        let day = PwGen::new("cqjxjnds");
        assert_eq!(day.part2(), Ok(0));
    }
}