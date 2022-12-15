pub type Color = [f32; 4];

pub trait Entity {
    fn id(&self) -> &String;
    fn triangles(&self) -> &Vec<Triangle>;
    fn update(&mut self, time_passed: f32);
}

#[derive(Clone)]
pub struct Triangle {
    pub coords: [f32; 9],
    pub color: [f32; 4],
}
