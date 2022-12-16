use crate::user_input::UserInput;

pub type Color = [f32; 4];

pub fn red() -> Color {
    [1.0, 0.0, 0.0, 1.0]
}

pub fn green() -> Color {
    [0.0, 1.0, 0.0, 1.0]
}

pub fn blue() -> Color {
    [0.0, 0.0, 1.0, 1.0]
}

pub fn yellow() -> Color {
    [1.0, 1.0, 0.0, 1.0]
}

pub fn pink() -> Color {
    [1.0, 0.0, 1.0, 1.0]
}

pub fn cyan() -> Color {
    [0.0, 1.0, 1.0, 1.0]
}

pub trait Entity {
    fn id(&self) -> &String;
    fn triangles(&self) -> &Vec<Triangle>;
    fn update(&mut self, _time_passed: f32) {}
    fn position(&self) -> (f32, f32) {
        return (0.0, 0.0);
    }
    fn on_user_input(&mut self, _input: &UserInput) {}
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub coords: [f32; 9],
    pub color: [f32; 4],
    pub shader_id: ShaderId,
}

#[derive(Clone, Default)]
pub enum ShaderId {
    #[default]
    Default,
    Water,
}

impl Triangle {
    pub fn new(coords: [f32; 9], color: [f32; 4]) -> Triangle {
        Triangle { coords, color }
    }

    pub fn from_points(
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        color: [f32; 4],
    ) -> Triangle {
        Triangle {
            coords: [p1.0, p1.1, 0.0, p2.0, p2.1, 0.0, p3.0, p3.1, 0.0],
            color,
        }
    }
}
