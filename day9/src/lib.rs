pub struct Checker<'a> {
    map: &'a [&'a [u8]],
    max_x: usize,
    max_y: usize,
}

impl<'a> Checker<'a> {
    pub fn new(map: &'a[&'a [u8]]) -> Checker<'a> {
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
            let value: char = value.into();
            let value = value.to_digit(10).unwrap() as usize;
            Some(value)
        } else {
            None
        }
    }
}
