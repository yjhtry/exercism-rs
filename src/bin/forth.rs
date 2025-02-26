mod forth {
    use std::collections::HashMap;

    pub type Value = i32;
    pub type Result = std::result::Result<(), Error>;

    #[derive(Debug, Default)]
    pub struct Forth {
        stack: Vec<Value>,
        context: HashMap<String, Vec<String>>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        DivisionByZero,
        StackUnderflow,
        UnknownWord,
        InvalidWord,
    }

    impl Forth {
        pub fn new() -> Forth {
            Forth::default()
        }

        pub fn stack(&self) -> &[Value] {
            &self.stack
        }

        fn parse_context(&mut self, input: &str) -> Result {
            match input
                .trim_matches([' ', ':', ';'])
                .to_string()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<String>>()
                .as_slice()
            {
                [name, ops @ ..] => match !ops.is_empty() && name.parse::<i32>().is_err() {
                    true => {
                        self.context.insert(
                            name.to_string(),
                            ops.iter()
                                .flat_map(|s| {
                                    self.context
                                        .get(s)
                                        .map(|v| v[0..(100.min(v.len()))].to_owned())
                                        .unwrap_or(vec![s.to_string()])
                                })
                                .collect(),
                        );
                    }
                    false => return Err(Error::InvalidWord),
                },
                _ => return Err(Error::InvalidWord),
            }

            Ok(())
        }

        fn parse_stack(&mut self, input: &str) -> Result {
            let items = input.split_whitespace().collect::<Vec<&str>>();
            let mut stack = vec![];

            for item in items {
                let ops = self
                    .context
                    .get(item)
                    .cloned()
                    .unwrap_or(vec![item.to_string()]);

                for op in ops {
                    match op.parse::<Value>() {
                        Ok(n) => stack.push(n),
                        Err(_) => match op.as_str() {
                            // dup drop over swap
                            op @ ("+" | "-" | "*" | "/" | "swap" | "over") => {
                                if stack.len() < 2 {
                                    return Err(Error::StackUnderflow);
                                }
                                let right = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                match op {
                                    "+" => stack.push(left + right),
                                    "-" => stack.push(left - right),
                                    "*" => stack.push(left * right),
                                    "swap" => {
                                        stack.push(right);
                                        stack.push(left);
                                    }
                                    "over" => {
                                        stack.push(left);
                                        stack.push(right);
                                        stack.push(left);
                                    }
                                    "/" => {
                                        if right == 0 {
                                            return Err(Error::DivisionByZero);
                                        } else {
                                            stack.push(left / right)
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            op @ ("dup" | "drop") => {
                                if stack.is_empty() {
                                    return Err(Error::StackUnderflow);
                                }

                                match op {
                                    "dup" => stack.push(*stack.last().unwrap()),
                                    "drop" => {
                                        stack.pop();
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => {
                                return Err(Error::UnknownWord);
                            }
                        },
                    }
                }
            }

            self.stack = stack;

            Ok(())
        }

        pub fn eval(&mut self, input: &str) -> Result {
            let lower = input.to_lowercase();
            if lower.starts_with(":") {
                self.parse_context(&lower)?;
            } else {
                self.parse_stack(&lower)?;
            }
            Ok(())
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::forth;
    mod parsing_and_numbers {
        use super::forth::*;
        #[test]
        fn numbers_just_get_pushed_onto_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 4 5").is_ok());
            assert_eq!(f.stack(), [1, 2, 3, 4, 5]);
        }
        #[test]
        fn pushes_negative_numbers_onto_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("-1 -2 -3 -4 -5").is_ok());
            assert_eq!(f.stack(), [-1, -2, -3, -4, -5]);
        }
    }
    mod addition {
        use super::forth::*;
        #[test]
        fn can_add_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 +").is_ok());
            assert_eq!(f.stack(), [3]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("+"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 +"), Err(Error::StackUnderflow));
        }
        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 +").is_ok());
            assert_eq!(f.stack(), [1, 5]);
        }
    }
    mod subtraction {
        use super::forth::*;
        #[test]
        fn can_subtract_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("3 4 -").is_ok());
            assert_eq!(f.stack(), [-1]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("-"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 -"), Err(Error::StackUnderflow));
        }
        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 12 3 -").is_ok());
            assert_eq!(f.stack(), [1, 9]);
        }
    }
    mod multiplication {
        use super::forth::*;
        #[test]
        fn can_multiply_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("2 4 *").is_ok());
            assert_eq!(f.stack(), [8]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("*"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 *"), Err(Error::StackUnderflow));
        }
        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 *").is_ok());
            assert_eq!(f.stack(), [1, 6]);
        }
    }
    mod division {
        use super::forth::*;
        #[test]
        fn can_divide_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("12 3 /").is_ok());
            assert_eq!(f.stack(), [4]);
        }
        #[test]
        fn performs_integer_division() {
            let mut f = Forth::new();
            assert!(f.eval("8 3 /").is_ok());
            assert_eq!(f.stack(), [2]);
        }
        #[test]
        fn errors_if_dividing_by_zero() {
            let mut f = Forth::new();
            assert_eq!(f.eval("4 0 /"), Err(Error::DivisionByZero));
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("/"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 /"), Err(Error::StackUnderflow));
        }
        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 12 3 /").is_ok());
            assert_eq!(f.stack(), [1, 4]);
        }
    }
    mod combined_arithmetic {
        use super::forth::*;
        #[test]
        fn addition_and_subtraction() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 + 4 -").is_ok());
            assert_eq!(f.stack(), [-1]);
        }
        #[test]
        fn multiplication_and_division() {
            let mut f = Forth::new();
            assert!(f.eval("2 4 * 3 /").is_ok());
            assert_eq!(f.stack(), [2]);
        }
        #[test]
        fn multiplication_and_addition() {
            let mut f = Forth::new();
            assert!(f.eval("1 3 4 * +").is_ok());
            assert_eq!(f.stack(), [13]);
        }
        #[test]
        fn addition_and_multiplication() {
            let mut f = Forth::new();
            assert!(f.eval("1 3 4 + *").is_ok());
            assert_eq!(f.stack(), [7]);
        }
    }
    mod dup {
        use super::forth::*;
        #[test]
        fn copies_a_value_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 dup").is_ok());
            assert_eq!(f.stack(), [1, 1]);
        }
        #[test]
        fn copies_the_top_value_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 dup").is_ok());
            assert_eq!(f.stack(), [1, 2, 2]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("dup"), Err(Error::StackUnderflow));
        }
    }
    mod drop {
        use super::forth::*;
        #[test]
        fn removes_the_top_value_on_the_stack_if_it_is_the_only_one() {
            let mut f = Forth::new();
            assert!(f.eval("1 drop").is_ok());
            assert_eq!(f.stack(), []);
        }
        #[test]
        fn removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 drop").is_ok());
            assert_eq!(f.stack(), [1]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("drop"), Err(Error::StackUnderflow));
        }
    }
    mod swap {
        use super::forth::*;
        #[test]
        fn swaps_the_top_two_values_on_the_stack_if_they_are_the_only_ones() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 swap").is_ok());
            assert_eq!(f.stack(), [2, 1]);
        }
        #[test]
        fn swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 swap").is_ok());
            assert_eq!(f.stack(), [1, 3, 2]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("swap"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 swap"), Err(Error::StackUnderflow));
        }
    }
    mod over {
        use super::forth::*;
        #[test]
        fn copies_the_second_element_if_there_are_only_two() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 over").is_ok());
            assert_eq!(f.stack(), [1, 2, 1]);
        }
        #[test]
        fn copies_the_second_element_if_there_are_more_than_two() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 over").is_ok());
            assert_eq!(f.stack(), [1, 2, 3, 2]);
        }
        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("over"), Err(Error::StackUnderflow));
        }
        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 over"), Err(Error::StackUnderflow));
        }
    }
    mod user_defined_words {
        use super::forth::*;
        #[test]
        fn can_consist_of_built_in_words() {
            let mut f = Forth::new();
            assert!(f.eval(": dup-twice dup dup ;").is_ok());
            assert!(f.eval("1 dup-twice").is_ok());
            assert_eq!(f.stack(), [1, 1, 1]);
        }
        #[test]
        fn execute_in_the_right_order() {
            let mut f = Forth::new();
            assert!(f.eval(": countup 1 2 3 ;").is_ok());
            assert!(f.eval("countup").is_ok());
            assert_eq!(f.stack(), [1, 2, 3]);
        }
        #[test]
        fn can_override_other_user_defined_words() {
            let mut f = Forth::new();
            assert!(f.eval(": foo dup ;").is_ok());
            assert!(f.eval(": foo dup dup ;").is_ok());
            assert!(f.eval("1 foo").is_ok());
            assert_eq!(f.stack(), [1, 1, 1]);
        }
        #[test]
        fn can_override_built_in_words() {
            let mut f = Forth::new();
            assert!(f.eval(": swap dup ;").is_ok());
            assert!(f.eval("1 swap").is_ok());
            assert_eq!(f.stack(), [1, 1]);
        }
        #[test]
        fn can_override_built_in_operators() {
            let mut f = Forth::new();
            assert!(f.eval(": + * ;").is_ok());
            assert!(f.eval("3 4 +").is_ok());
            assert_eq!(f.stack(), [12]);
        }
        #[test]
        fn can_use_different_words_with_the_same_name() {
            let mut f = Forth::new();
            assert!(f.eval(": foo 5 ;").is_ok());
            assert!(f.eval(": bar foo ;").is_ok());
            assert!(f.eval(": foo 6 ;").is_ok());
            assert!(f.eval("bar foo").is_ok());
            assert_eq!(f.stack(), [5, 6]);
        }
        #[test]
        fn can_define_word_that_uses_word_with_the_same_name() {
            let mut f = Forth::new();
            assert!(f.eval(": foo 10 ;").is_ok());
            assert!(f.eval(": foo foo 1 + ;").is_ok());
            assert!(f.eval("foo").is_ok());
            assert_eq!(f.stack(), [11]);
        }
        #[test]
        fn cannot_redefine_non_negative_numbers() {
            let mut f = Forth::new();
            assert_eq!(f.eval(": 1 2 ;"), Err(Error::InvalidWord));
        }
        #[test]
        fn cannot_redefine_negative_numbers() {
            let mut f = Forth::new();
            assert_eq!(f.eval(": -1 2 ;"), Err(Error::InvalidWord));
        }
        #[test]
        fn errors_if_executing_a_non_existent_word() {
            let mut f = Forth::new();
            assert_eq!(f.eval("foo"), Err(Error::UnknownWord));
        }
        #[test]
        fn only_defines_locally() {
            let mut f = Forth::new();
            assert!(f.eval(": + - ;").is_ok());
            assert!(f.eval("1 1 +").is_ok());
            assert_eq!(f.stack(), [0]);
            let mut f = Forth::new();
            assert!(f.eval("1 1 +").is_ok());
            assert_eq!(f.stack(), [2]);
        }
    }
    mod case_insensitivity {
        use super::forth::*;
        #[test]
        fn dup_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 DUP Dup dup").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }
        #[test]
        fn drop_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 4 DROP Drop drop").is_ok());
            assert_eq!(f.stack(), [1]);
        }
        #[test]
        fn swap_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 SWAP 3 Swap 4 swap").is_ok());
            assert_eq!(f.stack(), [2, 3, 4, 1]);
        }
        #[test]
        fn over_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 OVER Over over").is_ok());
            assert_eq!(f.stack(), [1, 2, 1, 2, 1]);
        }
        #[test]
        fn user_defined_words_are_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval(": foo dup ;").is_ok());
            assert!(f.eval("1 FOO Foo foo").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }
        #[test]
        fn definitions_are_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval(": SWAP DUP Dup dup ;").is_ok());
            assert!(f.eval("1 swap").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }
    }
}
