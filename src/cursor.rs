use std::thread::sleep;
use std::time::Duration;
use enigo::{Button, Coordinate, Direction, Enigo, InputError, Mouse};
use crate::display::Point;

pub(crate) struct Cursor {
    pub(crate) enigo: Enigo,
}

impl Cursor {
    pub(crate) fn new(enigo: Enigo) -> Self {
        Self {
            enigo
        }
    }

    pub(crate) fn click_point(&mut self, point: Point, delay: u64) {
        self.move_mouse(point.x, point.y).unwrap();
        self.enigo.button(Button::Left, Direction::Click).unwrap();
        sleep(Duration::from_millis(delay));
    }

    pub(crate) fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), InputError> {
        let (w, h) = self.enigo.main_display()?;
        let new_x = (65535.0 / (w - 1) as f64) * (x as f64);
        let new_y = (65535.0 / (h - 1) as f64) * (y as f64);

        self.enigo.move_mouse(new_x as i32, new_y as i32, Coordinate::Abs).unwrap();
        Ok(())
    }

    pub(crate) fn get_screen_size(&mut self) -> Result<(i32, i32), InputError> {
        Ok(self.enigo.main_display()?)
    }

    pub(crate) fn location(&self) -> Result<(i32, i32), InputError> {
        Ok(self.enigo.location()?)
    }
}