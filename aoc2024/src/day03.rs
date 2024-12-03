use util::lexer::Lexer;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut lexer = Lexer::new(input);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut toggle = true;
    while !lexer.eof() {
        lexer.eat_while(|c| !c.is_ascii_alphabetic() && c != '\0');

        let start = lexer.consumed();
        lexer.eat_while(|c| c.is_ascii_alphabetic() || c == '\'');
        let alpha_end = lexer.consumed();

        if input[start..alpha_end].ends_with("do") && lexer.eat('(') && lexer.eat(')') {
            toggle = true;
            continue;
        } else if input[start..alpha_end].ends_with("don't") && lexer.eat('(') && lexer.eat(')') {
            toggle = false;
            continue;
        }

        if input[start..alpha_end].ends_with("mul") {
            if !lexer.eat('(') {
                continue;
            }
            let left_start = lexer.consumed();
            lexer.eat_while(|c| c.is_ascii_digit());
            let left_end = lexer.consumed();
            let left = input[left_start..left_end].parse::<u64>().unwrap();

            if !lexer.eat(',') {
                continue;
            }
            let right_start = lexer.consumed();
            lexer.eat_while(|c| c.is_ascii_digit());
            let right_end = lexer.consumed();
            let right = input[right_start..right_end].parse::<u64>().unwrap();

            if !lexer.eat(')') {
                continue;
            }

            part1 += left * right;
            if toggle {
                part2 += left * right;
            }
        }
    }

    Solution::from((part1, part2))
}
