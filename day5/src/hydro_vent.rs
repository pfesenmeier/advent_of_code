#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct HydroVent {
    start: Point,
    end: Point,
    cur_x: isize,
    cur_y: isize,
}

enum Direction {
    Horizontal,
    Vertical,
    Unknown,
}

// TODO support vents that go right to left or up
// TODO abstract direction code into Line abstract;
// TODO use this logic to improve iterator setup in constructor

impl HydroVent {
    pub fn new(start: Point, end: Point) -> Self {
        
        let mut cur_x: isize = start.x.try_into().unwrap();
        cur_x -= 1;

        let mut cur_y: isize = start.y.try_into().unwrap();
        cur_y -= 1;

        HydroVent {
            cur_x,
            cur_y, 
            start,
            end,
        }
    }

    pub fn max_y(&self) -> usize {
        if self.start.y > self.end.y {
            self.start.y
        } else {
            self.end.y
        }
    }

    pub fn max_x(&self) -> usize {
        if self.start.x > self.end.x {
            self.start.x
        } else {
            self.end.x
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn get_direction(&self) -> Direction {
        if self.is_horizontal() {
            Direction::Horizontal
        } else if self.is_vertical() {
            Direction::Vertical
        } else {
            Direction::Unknown
        }
    }

    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

impl Iterator for HydroVent {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        match self.get_direction() {
            Direction::Horizontal => {
                self.cur_x += 1;
                if self.cur_x > self.end.x.try_into().unwrap() {
                    None
                } else {
                    Some(Point {
                        x: self.cur_x.try_into().unwrap(),
                        y: self.cur_y.try_into().unwrap(),
                    })
                }
            }
            Direction::Vertical => {
                self.cur_y += 1;
                if self.cur_y > self.end.y.try_into().unwrap() {
                    None
                } else {
                    Some(Point {
                        x: self.cur_x.try_into().unwrap(),
                        y: self.cur_y.try_into().unwrap(),
                    })
                }
            }
            _ => panic!("Tried to iterate on a diagonal vent. not yet supported"),
        }
    }
}
