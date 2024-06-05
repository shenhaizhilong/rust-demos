#[derive(Copy, Clone, Debug)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}