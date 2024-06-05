#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

