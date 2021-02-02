/**
 * You can check this challenge on the following url
 * @link{https://www.codewars.com/kata/550f22f4d758534c1100025a}
 */

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

fn is_opposite(elem: &Direction, aux: &Direction) -> bool {
    match (elem, aux) {
        (Direction::NORTH, Direction::SOUTH)
        | (Direction::SOUTH, Direction::NORTH)
        | (Direction::EAST, Direction::WEST)
        | (Direction::WEST, Direction::EAST) => true,
        _ => false,
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut clone = arr.to_vec();
    let mut idx = 0;
    loop {
        let curr = &clone[idx];
        let next = &clone[idx + 1];
        if is_opposite(curr, next) {
            clone.remove(idx);
            clone.remove(idx);
            idx = 0;
        } else {
            idx += 1;
        }
        if (idx + 1) >= clone.len() {
            break;
        }
    }
    clone
}

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction};

    #[test]
    fn returns_expected() {
        use Direction::*;
        let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
        assert_eq!(dir_reduc(&a), [WEST]);
    }

    #[test]
    fn returns_expected2() {
        use Direction::*;
        let a = [NORTH, WEST, SOUTH, EAST];
        assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
    }
}
