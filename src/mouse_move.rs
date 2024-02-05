use mouse_rs::{types::Point, Mouse};

const MOUSE_MOVE_DISTANCE: i32 = 50;

pub struct MouseMove {
    x: i32,
    y: i32,
}

impl MouseMove {
    pub fn new(x: i32, y: i32) -> MouseMove {
        MouseMove { x, y }
    }

    fn get_current_position(&self) -> Result<Point, Box<dyn std::error::Error>> {
        let mouse = Mouse::new();
        let pos = mouse.get_position().unwrap();
        println!("X = {}, Y = {}", pos.x, pos.y);
        Ok(pos)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_up(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x, position.y - MOUSE_MOVE_DISTANCE)
            .unwrap();
        self.set_position(position.x, position.y - MOUSE_MOVE_DISTANCE);
    }

    pub fn move_down(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x, position.y + MOUSE_MOVE_DISTANCE)
            .unwrap();

        self.set_position(position.x, position.y + MOUSE_MOVE_DISTANCE);
    }
    pub fn move_left(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x - MOUSE_MOVE_DISTANCE, position.y)
            .unwrap();
        self.set_position(position.x - MOUSE_MOVE_DISTANCE, position.y);
    }
    pub fn move_right(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x + MOUSE_MOVE_DISTANCE, position.y)
            .unwrap();
        self.set_position(position.x + MOUSE_MOVE_DISTANCE, position.y);
    }
}
