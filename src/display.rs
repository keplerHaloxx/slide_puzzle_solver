#![allow(unused)]

use std::ops::{Deref, DerefMut, Index};
use enigo::{Enigo, InputError, Mouse};
use crate::puzzle::{PuzzleState};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Rectangle {
    pub(crate) top_left: Point,
    pub(crate) bottom_right: Point,
    pub(crate) center: Option<Point>,
}

impl Rectangle {
    pub(crate) fn new(top_left: Point, bottom_right: Point, center: Option<Point>) -> Self {
        Rectangle {
            top_left,
            bottom_right,
            center,
        }
    }

    pub(crate) fn width(&self) -> i32 {
        self.bottom_right.x - self.top_left.x
    }

    pub(crate) fn height(&self) -> i32 {
        self.bottom_right.y - self.top_left.y
    }

    pub(crate) fn grid_positions(&self, screen_size: Point) -> Vec<Vec<Rectangle>> {
        let mut positions: Vec<Vec<Rectangle>> = vec![];

        let inside_width = self.width() / 3;
        let inside_height = self.height() / 3;

        for row in 0..3 {
            positions.push(vec![]);
            for column in 0..3 {
                // Calc top left corner
                let tl_x = self.top_left.x + (column * inside_width);
                let tl_y = self.top_left.y + (row * inside_height);

                // Calc bottom right
                let br_x = tl_x + inside_width;
                let br_y = tl_y + inside_height;

                // Calc center
                let center_x = (tl_x + br_x) / 2;
                let center_y = (tl_y + br_y) / 2;

                positions[row as usize].push(Rectangle {
                    top_left: Point { x: tl_x, y: tl_y },
                    bottom_right: Point { x: br_x, y: br_y },
                    center: Some(Point { x: center_x, y: center_y }),
                });
            }
        }
        positions
    }
}

/// Takes a Vec<PuzzleState> and a Vec<Vec<Point>> then returns where to click based on the puzzle state
pub(crate) fn path_to_pos(path: &Vec<PuzzleState>, positions: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut mouse_positions: Vec<Point> = Vec::new();
    for state in path {
        let empty = state.find_empty();

        let column = &positions[empty.0];
        let position = column[empty.1];
        mouse_positions.push(position);
    }
    mouse_positions
}

pub(crate) fn rect_to_points(positions: &Vec<Vec<Rectangle>>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for row in positions {
        for rect in row {
            points.push(Point {
                x: rect.center.unwrap().x,
                y: rect.center.unwrap().y,
            })
        }
    }
    points
}