pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn append(&mut self, item: i32) {
        self.list.push(item);
        self.modify_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.modify_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn modify_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Some sort of inheritance with Trait Objects
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // stand-in Trait Object that can be replaced with any Box that holds a value that implements
    // Draw so we can use draw() on every item inside the vector components
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!();
    }
}
