fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u64 {
    let crabs = parser::parse(input);
    let max = *crabs.iter().max().unwrap();

    let mut least_fuel = u64::MAX;
    for i in 0..max {
        let fuel = crabs
        .iter()
        .fold(0, |acc, crab| acc + (i64::from(*crab) - i64::from(i)).abs());
        if (fuel as u64) < least_fuel {
            least_fuel = fuel as u64;
        }
    }
    least_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1("16,1,2,0,4,2,7,1,2,14"), 37);
    }
}

mod parser {
   pub fn parse(input: &str) -> Vec<u32> {
     input
            .trim_end()
            .split(',')
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect::<Vec<u32>>()
    }

    #[cfg(test)]
    mod tests {
       use super::*;

       #[test]
       fn smoke_parser() {
            for num in parse(include_str!("input.txt")) {
                println!("{}", num);
            }
        } 
    }
}
