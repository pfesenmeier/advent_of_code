fn main() {
    const INPUT: &str = "6227618536
2368158384
5385414113
4556757523
6746486724
4881323884
4648263744
4871332872
4724128228
4316512167";
    println!("{}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let octopuses = crate::octopuses::Octopuses::new(input);
    let mut total = 0;
    for _ in 0..100 {
        total += octopuses.run_round();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1(SAMPLE_INPUT), 1656);
    }
}

// TODO Display flashes
mod octopuses {
    use std::collections::HashMap;

    struct Flash;

    struct Glowing;
    struct Dark;

    #[derive(Default)]
    struct Octopus<T> {
        marker: std::marker::PhantomData<T>,
    }

    impl Octopus<Dark> {
        pub fn new() -> Self {
            todo!()
        }
        pub fn energize(&mut self) -> Option<Flash> {
            todo!()
        }
    }

    impl Octopus<Glowing> {
        pub fn reset(&mut self) -> Octopus<Dark> {
            todo!()
        }
    }

    type Location = (usize, usize);

    pub struct Octopuses {
        flashes: u32,
        octopuses: Vec<u8>,
    }

    impl Octopuses {
        pub fn new(input: &str) -> Self {
            todo!()
        }

        // increase all
        // get all tens
        // loop { self.get_all_tens(); for each ten, get neighbors, flash,  }
        pub fn run_round(&mut self) -> u32 {
            self.energize_all();
            while let Some(glowing) = self.get_all_tens() {
                let dark_neighbors = glowing
                    .into_iter()
                    .map(|glow| self.get_dark_neighbors(glow))
                    .flatten()
                    .collect::<Vec<Octopus<Dark>>>();
                for mut octopus in dark_neighbors {
                    octopus.energize();
                }
            }
            42
        }

        fn flash(location: Location) {
            todo!()
        }

        fn energize_all(&mut self) {
            todo!()
        }

        fn get_all_tens(&self) -> Option<Vec<Octopus<Glowing>>> {
            todo!()
        }

        fn get_dark_neighbors(&self, octopus: Octopus<Glowing>) -> Vec<Octopus<Dark>> {
            todo!()
        }
    }

    // find_neighbors
    // increase.. and flash?
    // increase all
    // map of location to Octopus?
    //
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn smoke_flash() {}
    }
}
