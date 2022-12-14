use glam::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(vec: Vec3) -> Color {
        Color(vec)
    }

    pub fn as_vec(&self) -> &Vec3 {
        &self.0
    }
}

impl From<Vec3> for Color {
    fn from(val: Vec3) -> Color {
        Color(val)
    }
}
