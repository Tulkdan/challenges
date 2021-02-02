/**
 * You can check this challenge on the following url
 * @link{https://www.codewars.com/kata/525f50e3b73515a6db000b83}
 */

pub fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.into_iter().map(|n| n.to_string()).collect();
    format!(
        "({}) {}-{}",
        s.get(0..3).unwrap(),
        s.get(3..6).unwrap(),
        s.get(6..).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::{create_phone_number};

    #[test]
    fn example_test() {
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    }

    #[test]
    fn first_test() {
        assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    }

    #[test]
    fn second_test() {
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
    }
}
