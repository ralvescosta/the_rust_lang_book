use lib::{Draw, Screen};

use crate::lib::Bottom;
mod lib;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl SelectBox {
    pub fn new() -> SelectBox {
        SelectBox {
            width: 0,
            height: 0,
            options: vec![],
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    let scree = Screen::new(vec![Box::new(SelectBox::new()), Box::new(Bottom::new())]);
    scree.run();
}
