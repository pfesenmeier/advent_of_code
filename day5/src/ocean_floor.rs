use crate::hydro_vent::HydroVent;
use crate::hydro_vent::Point;
use core::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
struct OceanFloor {
    map: Vec<Vec<usize>>,
}

impl OceanFloor {
    pub fn new(vents: Vec<HydroVent>) -> Self {
        let width = vents
            .iter()
            .max_by(|lhs, rhs| {
                if lhs.max_x() > rhs.max_x() {
                    Ordering::Greater
                } else if lhs.max_x() == rhs.max_x() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
            .max_x();
        let length = vents
            .iter()
            .max_by(|lhs, rhs| {
                if lhs.max_y() > rhs.max_y() {
                    Ordering::Greater
                } else if lhs.max_y() == rhs.max_y() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
            .max_y();

        let mut map = Vec::with_capacity(length);
        for _ in 0..length {
            let mut columns = vec![];
            for _ in 0..width {
                columns.push(0);
            }
            map.push(columns);
        }

        for vent in vents {
            for point in vent {
                map[point.y][point.x] += 1;
            }
        }

        OceanFloor { map }
    }
    fn count_crossings(&self) -> usize {
        let mut crossings = 0;
        for row in &self.map {
            for column in row {
                if *column > 2 {
                    crossings += 1;
                }
            }
        }
        crossings
    }
}

impl Display for OceanFloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.map {
            let row_display = row.iter().fold("".to_string(), |mut acc, num| {
                acc.push_str(&num.to_string());
                acc
            });
            write!(f, "{}", row_display).unwrap();
        }
        Ok(())
    }
}

#[test]
fn test_three_by_three() {
    let mut vents: Vec<HydroVent> = vec![];
    vents.push(HydroVent::new(Point { x: 0, y: 0 }, Point { x: 0, y: 2 }));
    vents.push(HydroVent::new(Point { x: 0, y: 0 }, Point { x: 2, y: 0 }));
    let floor = OceanFloor::new(vents);

    assert_eq!(floor.count_crossings(), 1);
}
