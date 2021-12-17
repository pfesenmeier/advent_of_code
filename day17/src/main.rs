fn main() {
    let input_x = (179, 201);
    let input_y = (-109, -63);
    println!("{:?}", part_1(input_x, input_y));
}

fn part_1((x_left, x_right): (isize, isize), (y_bottom, y_top): (isize, isize)) {
    let launcher = probe_launcher::ProbeLauncher::new(x_left, x_right, y_top, y_bottom);

    for x in 0..1000 {
    for y in 0..1000 {
       println!("landed: {}", launcher.launch(x, y));
        }
    }
}

mod probe_launcher {
    use crate::probe::Probe;

    struct Target {
        pub x_left: isize,
        pub x_right: isize,
        pub y_top: isize,
        pub y_bottom: isize,
    }

    impl Target {
        pub fn is_struck(&self, x: isize, y: isize) -> bool {
            x >= self.x_left && x <= self.x_right && y <= self.y_top && y >= self.y_bottom
        }

        pub fn has_missed(&self, x: isize, y: isize) -> bool {
            self.is_past(x, y) || self.is_below(x, y)
        }

        fn is_below(&self, _x: isize, y: isize) -> bool {
            y < self.y_bottom
        }

        fn is_past(&self, x: isize, _y: isize) -> bool {
            x > self.x_right
        }
    }

    pub struct ProbeLauncher {
        target: Target,
    }

    impl ProbeLauncher {
        pub fn new(x_left: isize, x_right: isize, y_top: isize, y_bottom: isize) -> Self {
            assert!(x_left < x_right);
            assert!(y_top > y_bottom);
            Self {
                target: Target {
                    x_left,
                    x_right,
                    y_bottom,
                    y_top,
                },
            }
        }

        fn get_landing(&self, x: isize, y: isize) -> Option<bool> {
            if self.target.is_struck(x, y) {
                Some(true)
            } else if self.target.has_missed(x, y) {
                Some(false)
            } else {
                None
            }
        }

        pub fn launch(&self, x: isize, y: isize) -> bool {
            let probe = Probe::new(x, y);
            probe
                .into_iter()
                .find_map(|(x, y)| self.get_landing(x, y))
                .expect("infinite flight error")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_land() {
            let launcher = ProbeLauncher::new(20, 30, -5, -10);
            assert_eq!(launcher.launch(7, 2), true);
            assert_eq!(launcher.launch(6, 3), true);
            assert_eq!(launcher.launch(9, 0), true);
            assert_eq!(launcher.launch(17, -4), false);
        }
    }
}

mod probe {
    use std::cmp::Ordering;

    type Point = (isize, isize);

    #[derive(Default, Clone)]
    struct XY {
        pub x: isize,
        pub y: isize,
    }

    impl XY {
        pub fn new(x: isize, y: isize) -> Self {
            XY { x, y }
        }
    }

    impl From<XY> for Point {
        fn from(xy: XY) -> Self {
            (xy.x, xy.y)
        }
    }

    pub struct Probe {
        position: XY,
        velocity: XY,
    }

    impl Probe {
        pub fn new(x: isize, y: isize) -> Self {
            Probe {
                position: Default::default(),
                velocity: XY::new(x, y),
            }
        }
    }

    impl Iterator for Probe {
        type Item = Point;
        fn next(&mut self) -> Option<Self::Item> {
            self.position.x += self.velocity.x;
            self.position.y += self.velocity.y;

            match self.velocity.x.cmp(&0) {
                Ordering::Greater => self.velocity.x -= 1,
                Ordering::Less => self.velocity.x += 1,
                Ordering::Equal => (),
            };

            self.velocity.y -= 1;

            Some(Point::from(self.position.clone()))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn smoke_probe() {
            let probe = Probe::new(3, 9);

            for point in probe.into_iter() {
                println!("at point: {}, {}", point.0, point.1);

                if point.1 < -100 {
                    break;
                }
            }
        }
    }
}

// Probe: iterator that produces next step as Tuple
// Target: detect if Tuple in into target
/*
*  for (x,y) in probe.into_iter() {
*     if target.is_visible(x,y) {
*        return true;
*     }
*
*     if probe is above target
*     if probe is below target ->
*     - shoot straight until
*  }
* */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1((20, 30), (-10, -5)), (6, 9));
    }
}
