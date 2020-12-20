use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day18.txt")
        .expect("Something went wrong reading the file");

    let sum = impl_first_star(&contents);

    println!("day 18.1 - sum of expressions: {}", sum);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day18.txt")
        .expect("Something went wrong reading the file");

    let sum = impl_second_star(&contents);

    println!("day 18.2 - sum of expressions, with precedence: {}", sum);
}

fn impl_first_star(contents: &str) -> isize {
    compute(contents, false)
}

fn impl_second_star(contents: &str) -> isize {
    compute(contents, true)
}

fn compute(contents: &str, precedence: bool) -> isize {
    parse_expressions(contents)
        .iter()
        .map(|expr| {
            let (res, _) = evaluate(precedence, expr);
            res
        })
        .sum()
}

fn parse_expressions(contents: &str) -> Vec<Vec<String>> {
    contents.lines()
        .map(|line|  {
            let str = str::replace(line,"(", " ( ");
            let str = str::replace(&str,")", " ) ");
            str.split_whitespace().map(|s| s.to_string()).collect()
        })
        .collect()
}

fn evaluate(precedence: bool, tokens: &[String]) -> (isize, usize) {
    let mut operands: Vec<isize> = Vec::new();
    let mut ops: Vec<&str> = Vec::new();
    let mut i = 0;
    let mut end = false;
    while i < tokens.len() && !end {
        match tokens[i].parse::<isize>() {
            Result::Ok(num) => operands.push(num),
            _ => match tokens[i].as_str() {
                "(" => {
                    let (res, pos) = evaluate(precedence, &tokens[i+1..]);
                    operands.push(res);
                    i += pos;
                },
                ")" => end = true,
                op => ops.push(op)
            }
        }
        i += 1;
        while !ops.is_empty() &&
            operands.len() == ops.len() + 1 &&
            (!precedence || (ops.last() == Some(&"+") || i == tokens.len() || end)) {
            let op1 = operands.pop().unwrap();
            let op2 = operands.pop().unwrap();
            let op = ops.pop();
            match op {
                Some("+") => operands.push(op1 + op2),
                Some("*") => operands.push(op1 * op2),
                _ => panic!("invalid operation")
            }
        }
    }
    (*operands.last().unwrap(), i)
}

#[test]
fn test0_first_star() {
    let contents = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(impl_first_star(contents), 71);
}

#[test]
fn test1_first_star() {
    let contents = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(impl_first_star(contents), 51);
}

#[test]
fn test2_first_star() {
    let contents = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(impl_first_star(contents), 51);
}

#[test]
fn test3_first_star() {
    let contents = "2 * 3 + (4 * 5)";
    assert_eq!(impl_first_star(contents), 26);
}

#[test]
fn test4_first_star() {
    let contents = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(impl_first_star(contents), 437);
}

#[test]
fn test5_first_star() {
    let contents = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(impl_first_star(contents), 12240);
}

#[test]
fn test6_first_star() {
    let contents = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(impl_first_star(contents), 13632);
}

#[test]
fn test0_second_star() {
    let contents = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(impl_second_star(contents), 231);
}

#[test]
fn test1_second_star() {
    let contents = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(impl_second_star(contents), 51);
}

#[test]
fn test2_second_star() {
    let contents = "2 * 3 + (4 * 5)";
    assert_eq!(impl_second_star(contents), 46);
}

#[test]
fn test3_second_star() {
    let contents = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(impl_second_star(contents), 1445);
}

#[test]
fn test4_second_star() {
    let contents = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(impl_second_star(contents), 669060);
}

#[test]
fn test5_second_star() {
    let contents = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(impl_second_star(contents), 23340);
}
