pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut vector: Vec<u8> = vec![0; input.len()];
    let mut pointer: usize = 0;

    let instructions: Vec<&str> = code.split("").collect();
    let mut instruction_pointer: usize = 0;
    let mut open_brackets_position: usize = 0;
    let mut close_brackets_position: usize = 0;

    loop {
        if instruction_pointer >= instructions.len() {
            break;
        }

        println!("pointer: {}\tvalue: {}", vector[pointer], instructions[instruction_pointer]);

        match instructions[instruction_pointer] {
            ">" => { pointer += 1; },
            "<" => { pointer -= 1; },
            "+" => {
                if vector[pointer] == 255 {
                    vector[pointer] = 0;
                } else {
                    vector[pointer] += 1;
                }
            },
            "-" => {
                if vector[pointer] == 0 {
                    vector[pointer] = 255 ;
                } else {
                    vector[pointer] -= 1;
                }
            },
            "[" => {
                open_brackets_position = instruction_pointer;
                if vector[pointer] == 0 {
                    instruction_pointer = close_brackets_position;
                }
            },
            "]" => {
                close_brackets_position = instruction_pointer;
                if vector[pointer] == 0 {
                    instruction_pointer = open_brackets_position;
                }
            },
            "." => { println!("{}", vector[pointer]); pointer += 1; },
            "," => { vector[pointer] = input[pointer]; },
            _ => {},
        };
    }
    vector
}

#[cfg(test)]
mod test {
    use super::{brain_luck};

    #[test]
    fn example_test() {
        let input = vec![67, 111, 100, 101, 119, 97, 114, 115, 255];

        assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", input)).unwrap(), "Codewars");
    }

    #[test]
    fn second_test() {
        let input = vec![67, 111, 100, 101, 119, 97, 114, 115, 0];

        assert_eq!(String::from_utf8(brain_luck(",[.[-],]", input)).unwrap(), "Codewars");
    }

    #[test]
    fn third_test() {
        assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[->>+>>]<<<-]>>.", vec![8, 9]), vec![72]);
    }
}
