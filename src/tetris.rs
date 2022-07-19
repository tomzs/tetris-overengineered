use crate::shape::Shape;
#[derive(Debug)]
pub struct Tetris {
    width: usize,
    height: usize,
    current_shape: Shape,
    placed_shape: Vec<Shape>,
}

impl Tetris {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            current_shape: Shape::new_random(),
            placed_shape: vec![],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Tetris;
    #[test]
    fn test() {
        println!("{:#?}", Tetris::new(10, 10));
    }
}
