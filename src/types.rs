pub type Color = [f32; 4];

pub trait Entity {
    fn id(&self) -> &String;
    fn triangles(&self) -> &Vec<Triangle>;
    fn update(&mut self, _time_passed: f32) {}
    fn position(&self) -> (f32, f32) {
        return (0.0, 0.0);
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub coords: [f32; 9],
    pub color: [f32; 4],
}
