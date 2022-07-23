use std::{collections::HashSet, mem};

use crate::shape::{Pos, Shape};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    current_shape: Shape,
    placed_shape: Vec<Shape>,
    lost: bool,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            current_shape: &Shape::new_random() + Pos((width as i32) / 2, 0),
            placed_shape: vec![],
            lost: false,
        }
    }
    pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        shape
            .positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }
    pub fn is_colliding(&self, shape: &Shape) -> bool {
        self.placed_shape
            .iter()
            .any(|placed_shape| shape.collides_with(placed_shape))
    }
    pub fn is_line_full(&self, y: i32) -> bool {
        self.placed_shape
            .iter()
            .flat_map(|shape| shape.positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }
    fn remove_line(&mut self, y: i32) {
        for shape in self.placed_shape.iter_mut() {
            shape.remove_line(y)
        }
    }
    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y)
            }
        }
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }
        let translated_curr_shape = &self.current_shape + Pos(0, 1);
        if self.is_out_of_bounds(&translated_curr_shape)
            || self.is_colliding(&translated_curr_shape)
        {
            let newly_placed_shape = mem::replace(
                &mut self.current_shape,
                &Shape::new_random() + Pos((self.width) / 2, 0),
            );
            self.placed_shape.push(newly_placed_shape);
            self.remove_full_lines();
            if self.is_colliding(&self.current_shape) {
                self.lost = true;
            }
        } else {
            self.current_shape = translated_curr_shape;
        }
    }
    pub fn shift_current_shape(&mut self, direction: Direction) {
        if self.lost {
            return;
        }
        let translated_current_shape = &self.current_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };
        if !self.is_out_of_bounds(&translated_current_shape)
            && !self.is_colliding(&translated_current_shape)
        {
            self.current_shape = translated_current_shape;
        }
    }
    pub fn rotation(&mut self) {
        if self.lost {
            return;
        }
        let rotated_current_shape = self.current_shape.rotated();
        if !self.is_out_of_bounds(&rotated_current_shape)
            && !self.is_colliding(&rotated_current_shape)
        {
            self.current_shape = rotated_current_shape;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Tetris;
    #[test]
    fn test() {
        let mut tetris = Tetris::new(10, 30);
        println!("{:#?}", Tetris::new(10, 30));
    }
}
