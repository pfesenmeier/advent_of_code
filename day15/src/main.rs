fn main() {
    let input = include_str!("input.txt");

    println!("{}", part_1::part_1(input));
}

mod part_1 {
    use crate::map::Map;
    use crate::path::PathBuilder;

    pub fn part_1(input: &str) -> usize {
        let path = PathBuilder::new(Map::new(input));
        path.find_best_path()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part_1_should_pass_with_sample_input() {
            assert_eq!(part_1(include_str!("sample_input.txt")), 40);
        }
    }
}

mod point {
    #[derive(PartialEq, Hash, Eq, Default, Debug)]
    pub struct Point(pub usize,pub usize);
}

mod path {
    use crate::map::Map;
    use crate::point::Point;
    pub struct PathBuilder {
        map: Map,
    }

    impl PathBuilder {
        pub fn new(map: Map) -> Self {
            Self { map }
        }
        pub fn find_best_path(&self) -> usize {
            self.find_smallest_risk(&Point::default(), 0, usize::MAX)
        }
        fn find_smallest_risk(&self, point: &Point, risk: usize, least_risk: usize) -> usize {
            println!("On point {:?}", point);
            match self.map.find_neighbors(point) {
                Some(neighbors) => {
                    let mut best_risk = usize::MAX;
                    for neighbor in neighbors {
                        let neighbor_risk = self.find_smallest_risk(
                            &neighbor,
                            risk + self.map.get_risk(&neighbor),
                            least_risk,
                        );
                        if neighbor_risk < best_risk {
                            best_risk = neighbor_risk;
                        }
                    }
                    best_risk
                }
                None => {
                    if risk < least_risk {
                        risk
                    } else {
                        least_risk
                    }
                }
            }
        }
    }
}

mod map {
    use crate::point::Point;
    use std::collections::HashMap;
    pub struct Map {
        map: HashMap<Point, usize>,
    }

    impl Map {
        pub fn new(input: &str) -> Self {
            let mut map = HashMap::new();
            for (i, line) in input.lines().enumerate() {
              for (j, value) in line.chars().enumerate() {
                  map.insert(Point(j, i), (value as u8).into());
              }
            }
            Self { map }
        }

        pub fn get_risk(&self, point: &Point) -> &usize {
            self.map.get(point).expect("Called get_risk on point not returned by find_neighbors")
        }

        pub fn find_neighbors(&self, point: &Point) -> Option<Vec<Point>> {
            todo!()
        }
    }
}
