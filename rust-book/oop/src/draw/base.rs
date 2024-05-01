use std::path::Component;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component)
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button, {} {}", self.width, self.height);
    }
}

pub struct Input {
    pub length: u32,
    pub label: String,
}

impl Draw for Input {
    fn draw(&self) {
        println!("Drawing Input {}, {}", self.length, self.label)
    }
}
