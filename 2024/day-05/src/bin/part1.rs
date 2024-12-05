use std::collections::HashMap;

fn main() {
    let rules = include_str!("rules.txt");

    let updates = include_str!("updates.txt");

    let output = part1(rules, updates);

    dbg!(output);
}

fn part1(rules: &str, raw_updates: &str) -> i32 {
    let mut map = HashMap::new();

    for rule in rules.lines() {
        let mut two_pages = rule.split("|");

        let key = two_pages.next().unwrap().parse::<i32>().unwrap();

        let value = two_pages.next().unwrap().parse::<i32>().unwrap();

        map.entry(key)
            .and_modify(|e: &mut Vec<i32>| e.push(value))
            .or_insert(vec![value]);
    }

    let updates = input_to_matrix(raw_updates);

    let correct_updates = filter_out_incorrect(map, updates);

    let mid_values = get_mid_values(correct_updates);

    return mid_values.into_iter().sum();
}

fn get_mid_values(vec: Vec<Vec<i32>>) -> Vec<i32> {
    let mut mid_values = vec![];

    for inner in vec {
        let mid_idx = inner.len() / 2;

        let mid = inner[mid_idx];

        mid_values.push(mid);
    }

    return mid_values;
}

fn filter_out_incorrect(rules: HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut correct_updates = vec![];

    for pages in updates {
        let mut correct = true;

        for (i, i_page) in pages.clone().into_iter().enumerate() {
            for (j, j_page) in pages.clone().into_iter().enumerate() {
                if i == j {
                    continue;
                }

                let value = rules.get(&i_page);

                if let Some(vec) = value {
                    if j < i && vec.contains(&j_page) {
                        correct = false;
                        break;
                    }
                } else {
                    continue;
                }
            }

            if !correct {
                break;
            }
        }

        if correct {
            correct_updates.push(pages);
        }
    }

    return correct_updates;
}

fn input_to_matrix(input: &str) -> Vec<Vec<i32>> {
    let mut matrix = vec![];

    for line in input.lines() {
        let pages = line.split(",");

        let mut row = vec![];

        for page in pages {
            row.push(page.parse::<i32>().unwrap());
        }

        matrix.push(row);
    }

    return matrix;
}
