use crate::types::{Entity, Triangle};

pub struct Player {
    id: String,
    position: (f32, f32),
    triangles: Vec<Triangle>,
}

impl Entity for Player {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn update(&mut self, time_passed: f32) {
        self.position.0 += 0.001;
    }

    fn position(&self) -> (f32, f32) {
        self.position
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Player {
        Player {
            id: "player".to_string(),
            position,
            triangles: vec![Triangle {
                coords: [
                    0.0, 0.0, 1.0, //
                    3.0, 0.0, 1.0, //
                    0.0, 2.0, 1.0, //
                ],
                color: [1.0, 0.0, 0.0, 1.0],
            }],
        }
    }
}
