pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Screen {
        Screen { components }
    }
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Bottom {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Bottom {
    pub fn new() -> Bottom {
        Bottom {
            width: 0,
            height: 0,
            label: String::from(""),
        }
    }
}

impl Draw for Bottom {
    fn draw(&self) {}
}
