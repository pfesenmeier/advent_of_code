type Elist = Vec<(usize, String)>;
type ElistCmpFn = dyn Fn(Elist, Elist) -> Elist;

pub fn find_oxygen_co2_rating(input: &Vec<&str>) -> (usize, usize) {
    (
        find_rating(&input, &prefer_most_common_and_1),
        find_rating(&input, &prefer_least_common_and_0),
    )
}

fn find_rating(input: &Vec<&str>, compare_fn: &ElistCmpFn) -> usize {
    let mut list = input
        .iter()
        .enumerate()
        .map(|(pos, line)| {
            (
                pos,
                line.chars().rev().collect::<String>(),
            )
        })
        .collect::<Elist>();

    while list.len() > 1 {
        let (zeros_list, ones_list) = list.into_iter().fold(
            (vec![], vec![]),
            |(mut zeros_list, mut ones_list): (Elist, Elist), mut line| {
                if let Some(next_num) = line.1.pop() {
                    if next_num == '0' {
                        zeros_list.push(line)
                    } else {
                        ones_list.push(line)
                    }
                }
                (zeros_list, ones_list)
            },
        );
        list = compare_fn(zeros_list, ones_list);
    }

    let line = input[list[0].0];
    usize::from_str_radix(&line, 2).unwrap()
}

fn prefer_most_common_and_1(zeros_list: Elist, ones_list: Elist) -> Elist {
    if zeros_list.len() == ones_list.len() {
        ones_list
    } else if zeros_list.len() > ones_list.len() {
        zeros_list
    } else {
        ones_list
    }
}

fn prefer_least_common_and_0(zeros_list: Elist, ones_list: Elist) -> Elist {
    if zeros_list.len() == ones_list.len() {
        zeros_list
    } else if zeros_list.len() < ones_list.len() {
        zeros_list
    } else {
        ones_list
    }
}

#[test]
fn oxygen_calculated_correctly() {
    let input = include_str!("sample_input.txt").lines().collect();
    assert_eq!(find_rating(&input, &prefer_most_common_and_1), 23);
}

#[test]
fn co2_calculated_correctly() {
    let input = include_str!("sample_input.txt").lines().collect();
    assert_eq!(find_rating(&input, &prefer_least_common_and_0), 10);
}
