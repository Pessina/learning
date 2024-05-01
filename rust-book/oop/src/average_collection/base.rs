pub struct AverageCollection {
    vec: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, number: i32) {
        self.vec.push(number);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.vec.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn update_average(&mut self) {
        self.average = self.vec.iter().fold(0, |acc, e| acc + e) as f64 / self.vec.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
