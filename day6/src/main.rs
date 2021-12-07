mod lantern_fish;
mod parse_csv;

use lantern_fish::FishSchool;

fn main() {
    let input = parse_csv::parse_csv(include_str!("input.txt"));

    println!("Day 1 answer: {}", part_1(&input));
    println!("Day 2 answer: {}", part_2(&input));
}

fn part_1(nums: &Vec<u32>) -> u32 {
    let mut fish_school = FishSchool::new(nums.to_vec());

    for _ in 0..80 {
        fish_school.simulate_day();
    }

    fish_school.count_fish().try_into().unwrap()
}

fn part_2(nums: &Vec<u32>) -> u64 {
    let mut fish_school = FishSchool::new(nums.to_vec());

    for day in 0..256 {
        fish_school.simulate_day();
        println!("Day {}. There are {} lantern fish", day, fish_school.count_fish());
    }

    fish_school.count_fish().try_into().unwrap()
}

#[test]
fn test_sample() {
  let input = parse_csv::parse_csv("3,4,3,1,2");
  assert_eq!(part_1(&input), 5934);
}

#[test]
fn test_part_2() {
  let input = parse_csv::parse_csv("3,4,3,1,2");
  assert_eq!(part_2(&input), 26984457539);
}

