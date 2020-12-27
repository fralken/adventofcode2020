use std::fs;
use std::collections::{ HashMap, HashSet };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day21.txt")
        .expect("Something went wrong reading the file");

    let res = impl_first_star(&contents);

    println!("day 21.1 - times that ingredients w/o allergens appear: {}", res);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day21.txt")
        .expect("Something went wrong reading the file");

    let list = impl_second_star(&contents);

    println!("day 21.2 - canonical dangerous ingredient list: {}", list);
}

fn impl_first_star(contents: &str) -> usize {
    let food = parse_food(contents);
    let ingredients = extract_ingredients(&food);
    let allergens = extract_allergens(&food);
    let allergens_for_ingredients = allergens_for_ingredients(&allergens, &food);
    let ingredients_without_allergens = &ingredients -
        &allergens_for_ingredients.values().fold(HashSet::new(), |acc, iset| &acc | iset);
    ingredients_without_allergens.iter()
        .map(|i| food.iter().filter(|(iset, _)| iset.contains(i)).count())
        .sum()
}

fn impl_second_star(contents: &str) -> String {
    let food = parse_food(contents);
    let allergens = extract_allergens(&food);
    let allergens_for_ingredients = allergens_for_ingredients(&allergens, &food);
    let mut allergens_ingredients_pairs = allergens_ingredients_pairs(&allergens_for_ingredients);
    allergens_ingredients_pairs.sort_unstable();
    allergens_ingredients_pairs.iter().map(|(_, i)| *i).collect::<Vec<_>>().join(",")
}

fn parse_food(contents: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    contents
        .lines()
        .map(|line| {
            let mut items = line.split("(contains ");
            let ingredients = items.next().unwrap()
                .split_whitespace()
                .collect::<HashSet<_>>();
            let allergens = items.next().unwrap()
                .split(')')
                .next().unwrap()
                .split(", ")
                .collect::<HashSet<_>>();
            (ingredients, allergens)
        })
        .collect::<Vec<_>>()
}

fn extract_ingredients<'a>(food: &[(HashSet<&'a str>, HashSet<&str>)]) -> HashSet<&'a str> {
    food.iter().fold(HashSet::new(), |s, (i, _)| &s | i)
}

fn extract_allergens<'a>(food: &[(HashSet<&str>, HashSet<&'a str>)]) -> HashSet<&'a str> {
    food.iter().fold(HashSet::new(), |s, (_, a)| &s | a)
}

fn allergens_for_ingredients<'a, 'b>(allergens: &HashSet<&'a str>, food: &[(HashSet<&'b str>, HashSet<&str>)]) -> HashMap<&'a str, HashSet<&'b str>> {
    allergens.iter()
        .map(|a| {
            let i = food.iter()
                .filter_map(|(iset, aset)| if aset.contains(a) { Some(iset) } else { None })
                .fold(None, |acc, iset|
                    match acc {
                        Some(a) => Some(&a & iset),
                        None => Some(iset.clone())
                    }
                ).unwrap();
            (*a, i)
        })
        .collect::<HashMap<_, _>>()
}

fn allergens_ingredients_pairs<'a, 'b>(allergens_for_ingredients: &HashMap<&'a str, HashSet<&'b str>>) -> Vec<(&'a str, &'b str)> {
    let mut list = Vec::new();
    let mut map = allergens_for_ingredients.clone();
    while !map.is_empty() {
        let (&allergen, iset) = map.iter().find(|(_, iset)| iset.len() == 1).unwrap();
        let ingredient = iset.iter().next().unwrap();
        list.push((allergen, *ingredient));
        map = map.iter()
            .filter_map(|(a, i)|
                if *a == allergen { None } else { Some((*a, i - iset)) }
            )
            .collect::<HashMap<_, _>>();
    };
    list
}

#[test]
fn test0_first_star() {
    let contents = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(impl_first_star(contents), 5);
}

#[test]
fn test0_second_star() {
    let contents = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(impl_second_star(contents), "mxmxvkd,sqjhc,fvjkl");
}
