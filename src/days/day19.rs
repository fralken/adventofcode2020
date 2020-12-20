use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day19.txt")
        .expect("Something went wrong reading the file");

    let msgs = impl_first_star(&contents);

    println!("day 19.1 - number of messages that match rule 0: {}", msgs);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day19.txt")
        .expect("Something went wrong reading the file");

    let msgs = impl_second_star(&contents);

    println!("day 19.2 - number of messages that match rule 0 (with new rules): {}", msgs);
}

fn impl_first_star(contents: &str) -> usize {
    let (rules, msgs) = parse(contents);
    count_messages(&rules, &msgs, "0")
}

fn impl_second_star(contents: &str) -> usize {
    let (mut rules, msgs) = parse(contents);
    rules.insert("8", "42 | 42 8");
    rules.insert("11", "42 31 | 42 11 31");
    count_messages(&rules, &msgs, "0")
}

fn count_messages(rules: &HashMap<&str, &str>, msgs: &[&str], rule: &str) -> usize {
    let regex = format!("^{}$", build_regex(&rules, rule, 5));
    let re = Regex::new(&regex).unwrap();
    msgs.iter().filter(|msg| re.is_match(msg)).count()
}

fn parse(contents: &str) -> (HashMap<&str, &str>, Vec<&str>) {
    let mut split = contents.split("\n\n");
    let rules = split.next().unwrap()
        .lines()
        .map(|line| {
            let mut rule = line.split(':');
            let num = rule.next().unwrap();
            let set = rule.next().unwrap();
            (num, set.trim())
        })
        .collect::<HashMap<_, _>>();
    let msgs = split.next().unwrap().lines().collect();
    (rules, msgs)
}

fn build_regex(rules: &HashMap<&str, &str>, base: &str, depth: usize) -> String {
    if depth == 0 {
        String::new()
    } else if rules[base].starts_with('"') && rules[base].ends_with('"') {
        rules[base].trim_matches('"').to_string()
    } else {
        let regex = rules[base].split('|')
            .map(|rule| rule.split_whitespace()
                .map(|r|
                    build_regex(rules, r, if r == base { depth - 1 } else { depth })
                )
                .collect::<Vec<_>>()
                .concat()
            )
            .collect::<Vec<_>>()
            .join("|");
        if rules[base].contains('|') { format!("({})", regex) } else { regex }
    }
}

#[test]
fn test0_first_star() {
    let contents = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    assert_eq!(impl_first_star(contents), 2);
}

#[test]
fn test1_first_star() {
    let contents = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    assert_eq!(impl_first_star(contents), 3);
}

#[test]
fn test0_second_star() {
    let contents = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    assert_eq!(impl_second_star(contents), 12);
}
