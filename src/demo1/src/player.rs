#[derive(Debug, Clone, Default)]
pub struct Player {
    pub(crate) name: String,
    strength: u8,
    hit_points: u8,
}