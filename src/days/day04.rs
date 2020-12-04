use std::fs;
use std::collections::HashMap;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day04.txt")
        .expect("Something went wrong reading the file");

    let passports = impl_first_star(&contents);

    println!("day  4.1 - valid passports: {}", passports);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day04.txt")
        .expect("Something went wrong reading the file");

    let passports = impl_second_star(&contents);

    println!("day  4.2 - valid passports with validated fields: {}", passports);
}

fn impl_first_star(contents: &str) -> usize {
    let passports = parse_passports(&contents);

    passports.iter().filter(|passport| is_valid_passport(passport)).count()
}

fn impl_second_star(contents: &str) -> usize {
    let passports = parse_passports(&contents);

    passports.iter().filter(|passport| is_valid_passport_and_fields(passport)).count()
}

fn parse_passports(contents: &str) -> Vec<HashMap<String, String>> {
    let mut passports = Vec::new();
    passports.push(HashMap::new());
    contents
        .lines()
        .for_each(|line| {
            if line.is_empty() {
                passports.push(HashMap::new())
            } else {
                let passport = passports.last_mut().unwrap();
                line.split(' ')
                    .for_each(|field| {
                        let pair = field.split(':').collect::<Vec<_>>();
                        passport.insert(pair[0].to_string(), pair[1].to_string());
                    });
            }
        });
    passports
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn is_valid_passport_and_fields(passport: &HashMap<String, String>) -> bool {
    if is_valid_passport(passport) {
        let byr = passport.get("byr").unwrap().as_str();
        let iyr = passport.get("iyr").unwrap().as_str();
        let eyr = passport.get("eyr").unwrap().as_str();
        let hgt = passport.get("hgt").unwrap().as_str();
        let hcl = passport.get("hcl").unwrap().as_str();
        let ecl = passport.get("ecl").unwrap().as_str();
        let pid = passport.get("pid").unwrap().as_str();

        (byr.len() == 4 && byr >= "1920" && byr <= "2002") &&
        (iyr.len() == 4 && iyr >= "2010" && iyr <= "2020") &&
        (eyr.len() == 4 && eyr >= "2020" && eyr <= "2030") &&
        if hgt[hgt.len()-2..] == *"cm" {
            let val = hgt[..hgt.len()-2].parse::<usize>();
            if let Ok(v) = val { v >= 150 && v <= 193 } else { false }
        } else if hgt[hgt.len()-2..] == *"in" {
            let val = hgt[..hgt.len()-2].parse::<usize>();
            if let Ok(v) = val { v >= 59 && v <= 76 } else { false }
        } else { false } &&
        (hcl[0..1] == *"#" && hcl[1..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') )) &&
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) &&
        (pid.len() == 9 && pid.parse::<usize>().is_ok())
    } else {
        false
    }
}

#[test]
fn test0_first_star() {
    let passports =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
         byr:1937 iyr:2017 cid:147 hgt:183cm\n\
         \n\
         iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
         hcl:#cfa07d byr:1929\n\
         \n\
         hcl:#ae17e1 iyr:2013\n\
         eyr:2024\n\
         ecl:brn pid:760753108 byr:1931\n\
         hgt:179cm\n\
         \n\
         hcl:#cfa07d eyr:2025 pid:166559648\n\
         iyr:2011 ecl:brn hgt:59in";
    assert_eq!(impl_first_star(passports), 2);
}

#[test]
fn test0_second_star() {
    let passports =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
         byr:1937 iyr:2017 cid:147 hgt:183cm\n\
         \n\
         iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
         hcl:#cfa07d byr:1929\n\
         \n\
         hcl:#ae17e1 iyr:2013\n\
         eyr:2024\n\
         ecl:brn pid:760753108 byr:1931\n\
         hgt:179cm\n\
         \n\
         hcl:#cfa07d eyr:2025 pid:166559648\n\
         iyr:2011 ecl:brn hgt:59in";
    assert_eq!(impl_second_star(passports), 2);
}

#[test]
fn test1_second_star() {
    let passports =
        "eyr:1972 cid:100\n\
         hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
         \n\
         iyr:2019\n\
         hcl:#602927 eyr:1967 hgt:170cm\n\
         ecl:grn pid:012533040 byr:1946\n\
         \n\
         hcl:dab227 iyr:2012\n\
         ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
         \n\
         hgt:59cm ecl:zzz\n\
         eyr:2038 hcl:74454a iyr:2023\n\
         pid:3556412378 byr:2007";
    assert_eq!(impl_second_star(passports), 0);
}

#[test]
fn test2_second_star() {
    let passports =
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
         hcl:#623a2f\n\
         \n\
         eyr:2029 ecl:blu cid:129 byr:1989\n\
         iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
         \n\
         hcl:#888785\n\
         hgt:164cm byr:2001 iyr:2015 cid:88\n\
         pid:545766238 ecl:hzl\n\
         eyr:2022\n\
         \n\
         iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    assert_eq!(impl_second_star(passports), 4);
}
