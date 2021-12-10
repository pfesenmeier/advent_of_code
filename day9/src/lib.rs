pub struct Checker<'a> {
    map: &'a Vec<&'a [u8]>,
    max_x: usize,
    max_y: usize,
}

impl<'a> Checker<'a> {
    pub fn new(map: &'a Vec<&'a [u8]>) -> Checker<'a> {
        Checker {
            max_x: map[0].len() - 1,
            max_y: map.len() - 1,
            map,
        }
    }
    pub fn check_local_min(&self, x: usize, y: usize) -> Option<usize> {
        let map = &self.map;
        let value = map[y][x];

        let less_than_right = || value < map[y][x + 1];
        let less_than_left = || value < map[y][x - 1];
        let less_than_above = || value < map[y - 1][x];
        let less_than_below = || value < map[y + 1][x];

        let is_horizontal_min = match x {
            0 => less_than_right(),
            a if a == self.max_x => less_than_left(),
            _ => less_than_left() && less_than_right(),
        };

        let is_vertical_min = match y {
            0 => less_than_below(),
            a if a == self.max_y => less_than_above(),
            _ => less_than_below() && less_than_above(),
        };

        if is_horizontal_min && is_vertical_min {
            let value:char = value.into();
            let value = value.to_digit(10).unwrap() as usize;
            Some(value)
        } else {
            None
        }
    }
}
//
// risk
// add(low_point: Point)
// Map -> iter trait
// take while a,b a<b,
// map Each
// let values = Vec
// while None()
// (index, value)
//
//  enumerate().map(
//    str.filter(|| )
//  )
//  for i in width {
//    for j in length {
//       if s[j][i] >
//    }
//  }
//
/*
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678

Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
*/
