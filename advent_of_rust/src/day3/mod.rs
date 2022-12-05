use std::collections::{BTreeMap, BTreeSet};

use anyhow::Error;
use iter_tools::Itertools;
use lazy_static::lazy_static;

pub fn part_one(input: Vec<String>) -> Result<String, Error> {
    let sum: u32 = input.into_iter().map(priority_in_rucksack).sum();
    Ok(format!("Solution for day 3, part one is {}", sum))
}

pub fn part_two(input: Vec<String>) -> Result<String, Error> {
    let sum: u32 = input
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|three_lines| {
            let components_in_group = three_lines
                .into_iter()
                .map(|line| line.chars().collect::<Vec<char>>());
            let sum_in_group: u32 = get_common_components(components_in_group)
                .into_iter()
                .map(component_priority)
                .sum();
            sum_in_group
        })
        .sum();
    Ok(format!("Solution for day 3, part two is {}", sum))
}

lazy_static! {
    static ref component_codes: BTreeMap<char, u32> = ('a'..='z')
        .into_iter()
        .chain('A'..='Z')
        .zip(1..=52)
        .collect();
}

fn get_components(line: String) -> (Vec<char>, Vec<char>) {
    let (a, b) = line.split_at(line.len() / 2).clone();
    (
        a.chars().collect::<Vec<char>>(),
        (b.chars().collect::<Vec<char>>()),
    )
}

fn get_common_components<A: IntoIterator<Item = Vec<char>>>(components: A) -> BTreeSet<char> {
    fn go(common_components: BTreeSet<char>, components: BTreeSet<char>) -> BTreeSet<char> {
        common_components
            .intersection(&components)
            .cloned()
            .collect()
    }
    match components
        .into_iter()
        .map(|components| components.into_iter().collect())
        .reduce(go)
    {
        Some(components) => components,
        None => BTreeSet::new(),
    }
}

fn component_priority(component: char) -> u32 {
    component_codes
        .get(&component)
        .cloned()
        .expect(&format!("Component not found: {}", component))
}
fn priority_in_rucksack(line: String) -> u32 {
    let (component1, component2) = get_components(line);
    let common = get_common_components(vec![component1, component2]);
    common.into_iter().map(component_priority).sum()
}
