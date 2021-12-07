struct LanternFish {
  counter: u32,
}

impl LanternFish {
    fn new(counter: u32) -> Self {
        LanternFish { counter }
    }

    fn age(&mut self) -> Option<LanternFish> {
        match self.counter {
            0 => { self.counter = 6; Some(LanternFish::new(8)) },
            _ => { self.counter -= 1; None }
        }
    }
}

pub struct FishSchool {
    fish: Vec<LanternFish>,
}

impl FishSchool {
    pub fn new(numbers: Vec<u32>) -> Self {
        let mut fish = vec![];
        for num in numbers {
           fish.push(LanternFish::new(num));
        }
        FishSchool{ fish }
    }

    pub fn simulate_day(&mut self) {
        let mut new_fish = vec![];
        for f in &mut self.fish {
            if let Some(baby) = f.age() {
                new_fish.push(baby);
            };
        }
        self.fish.append(&mut new_fish);
    }

    pub fn count_fish(&self) -> usize {
        self.fish.iter().count()
    }
}
